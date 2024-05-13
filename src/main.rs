use std::{fs::File, path::Path};

use clap::Parser;
use eosim::{
    context::Context,
    global_properties::GlobalPropertyContext,
    random::RandomContext,
    reports::{get_file_report_handler, ReportsContext},
};
use eosim_demo::sir::{
    global_properties::{InfectiousPeriod, InitialInfections, Population, R0},
    incidence_report::IncidenceReport,
    infection_manager::InfectionManager,
    infection_seeder::InfectionSeeder,
    population_loader::PopulationLoader,
    transmission_manager::TransmissionManager,
};
use serde_derive::{Deserialize, Serialize};

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
}

fn main() {
    // Parse args and load parameters
    let args = SirArgs::parse();
    let config_file = File::open(&args.input)
        .unwrap_or_else(|_| panic!("Could not open config file: {}", args.input));
    let parameters: Parameters =
        serde_yaml::from_reader(config_file).expect("Could not parse config file");

    let mut context = Context::new();

    // Set up parameters in simulation
    context.set_global_property_value::<Population>(parameters.population);
    context.set_global_property_value::<R0>(parameters.r0);
    context.set_global_property_value::<InfectiousPeriod>(parameters.infectious_period);
    context.set_global_property_value::<InitialInfections>(parameters.initial_infections);

    // Set up RNG
    context.set_base_random_seed(parameters.random_seed);

    // Add reports
    context.add_component::<IncidenceReport>();
    context.set_report_item_handler::<IncidenceReport>(get_file_report_handler::<IncidenceReport>(
        File::create(Path::new(&args.output).join("incidence_report.csv"))
            .expect("Could not create output file."),
    ));

    // Add model components
    context.add_component::<PopulationLoader>();
    context.add_component::<InfectionManager>();
    context.add_component::<TransmissionManager>();
    context.add_component::<InfectionSeeder>();

    context.execute();
}
