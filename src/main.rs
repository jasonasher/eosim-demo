use std::{fs::File, path::Path};

use clap::Parser;
use eosim::{
    context::Context,
    global_properties::GlobalPropertyContext,
    random::RandomContext,
    reports::{get_channel_report_handler, ReportsContext, Report},
};
use eosim_demo::sir::{
    global_properties::{InfectiousPeriod, InitialInfections, Population, R0, DeathRate},
    incidence_report::{IncidenceReport, Infection},
    infection_manager::InfectionManager,
    infection_seeder::InfectionSeeder,
    population_loader::PopulationLoader,
    transmission_manager::TransmissionManager,
    death_manager::DeathManager,
    death_report::{DeathReport, Death}
};
use serde_derive::{Deserialize, Serialize};
use threadpool::ThreadPool;
use tokio::sync::mpsc::{self, Sender};
use tokio::runtime::Handle;

#[derive(Debug, Parser)]
struct SirArgs {
    /// Input config file
    #[arg(short, long)]
    input: String,
    /// Output directory
    #[arg(short, long)]
    output: String,
    /// Number of threads
    #[arg(short, long, default_value_t = 1)]
    threads: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
struct Parameters {
    population: usize,
    r0: f64,
    infectious_period: f64,
    initial_infections: usize,
    random_seed: u64,
    death_rate: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
struct Scenario {
    scenario: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum Config {
    Single(Parameters),
    Multiple(Vec<Parameters>),
}

fn setup_context(context: &mut Context, parameters: &Parameters) {
    // Set up parameters in simulation
    context.set_global_property_value::<Population>(parameters.population);
    context.set_global_property_value::<R0>(parameters.r0);
    context.set_global_property_value::<InfectiousPeriod>(parameters.infectious_period);
    context.set_global_property_value::<InitialInfections>(parameters.initial_infections);
    context.set_global_property_value::<DeathRate>(parameters.death_rate);

    // Set up RNG
    context.set_base_random_seed(parameters.random_seed);

    // Add reports
    context.add_component::<IncidenceReport>();
    context.add_component::<DeathReport>();

    // Add model components
    context.add_component::<PopulationLoader>();
    context.add_component::<InfectionManager>();
    context.add_component::<TransmissionManager>();
    context.add_component::<InfectionSeeder>();
    context.add_component::<DeathManager>();
}

pub fn get_bounded_channel_report_handler<T: Report, S>(
    sender: Sender<(S, T::Item)>,
    id: S,
) -> impl FnMut(T::Item) + 'static
where
    T::Item: serde::Serialize + Send + 'static, 
    S: serde::Serialize + Send + Copy + 'static, 
{
    move |item| {
        tokio::spawn({
            let sender = sender.clone(); 
            async move {
                if let Err(e) = sender.send((id, item)).await {
                    panic!("Due to receiver being closed, failed to send item: {:?}", e);
                }
            }
        });
    }
}

fn run_single_threaded(parameters_vec: Vec<Parameters>, output_path: &Path) {
    let output_file = File::create(output_path.join("incidence_report.csv"))
        .expect("Could not create output file.");
    for (scenario, parameters) in parameters_vec.iter().enumerate() {
        let mut writer_builder = csv::WriterBuilder::new();
        // Don't re-write the headers
        if scenario > 0 {
            writer_builder.has_headers(false);
        }
        let mut writer = writer_builder.from_writer(
            output_file
                .try_clone()
                .expect("Could not write to output file"),
        );
        // Set up and execute context
        let mut context = Context::new();
        context.set_report_item_handler::<IncidenceReport>(move |item| {
            if let Err(e) = writer.serialize((Scenario { scenario }, item)) {
                eprintln!("{}", e);
            }
        });
        setup_context(&mut context, parameters);
        context.execute();
        println!("Scenario {} completed", scenario);
    }
}

async fn run_multi_threaded(parameters_vec: Vec<Parameters>, output_path: &Path, threads: u8) {
    let output_file = File::create(output_path.join("incidence_report.csv"))
        .expect("Could not create output file.");
    let death_file = File::create(output_path.join("death_report.csv"))
        .expect("Could not create death report file.");

    let pool = ThreadPool::new(threads.into());
    let (sender, mut receiver) = mpsc::channel::<(Scenario, Infection)>(100000);
    let (death_sender, mut death_receiver) = mpsc::channel::<(Scenario, Death)>(100000);

    let handle = Handle::current();

    for (scenario, parameters) in parameters_vec.iter().enumerate() {
        let sender = sender.clone();
        let death_sender = death_sender.clone();
        let parameters = *parameters;
        let handle = handle.clone();
        pool.execute(move || {
            let _guard = handle.enter();
            // Set up and execute context
            let mut context = Context::new();
            context.set_report_item_handler::<IncidenceReport>(get_bounded_channel_report_handler::<
                IncidenceReport,
                Scenario
            >(
                sender, Scenario { scenario }
            ));            
            context.set_report_item_handler::<DeathReport>(get_bounded_channel_report_handler::<
                DeathReport,
                Scenario
            >(
                death_sender, Scenario { scenario }
            ));            
            setup_context(&mut context, &parameters);
            context.execute();
            println!("Scenario {} completed", scenario);
        });
    }
    drop(sender);
    drop(death_sender);

    // Write output from main thread 
    let mut incidence_writer = csv::Writer::from_writer(output_file);
    let mut death_writer = csv::Writer::from_writer(death_file);
    loop {
        tokio::select! {
            Some(item) = receiver.recv() =>{
                incidence_writer.serialize(item).unwrap();
            },
            Some(item) = death_receiver.recv() =>{
                death_writer.serialize(item).unwrap();
            },
            else => break,
        }
    }
}

#[tokio::main]
async fn main() {
    // Parse args and load parameters
    let args = SirArgs::parse();
    let config_file = File::open(&args.input)
        .unwrap_or_else(|_| panic!("Could not open config file: {}", args.input));
    let config: Config = serde_yaml::from_reader(config_file).expect("Could not parse config file");
    let output_path = Path::new(&args.output);

    match config {
        Config::Single(parameters) => {
            run_single_threaded(vec![parameters], output_path)
        }
        Config::Multiple(parameters_vec) => {
            if args.threads <= 1 {
                run_single_threaded(parameters_vec, output_path)
            } else {
                run_multi_threaded(parameters_vec, output_path, args.threads).await;
            }
        }
    }
}
