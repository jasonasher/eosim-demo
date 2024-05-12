use eosim::{context::Context, global_properties::GlobalPropertyContext, random::RandomContext};
use eosim_demo::sir::{
    global_properties::{InfectiousPeriod, InitialInfections, Population, R0},
    incidence_report::IncidenceReport,
    infection_manager::InfectionManager,
    infection_seeder::InfectionSeeder,
    population_loader::PopulationLoader,
    transmission_manager::TransmissionManager,
};

fn main() {
    let mut context = Context::new();

    context.set_global_property_value::<Population>(1000000);
    context.set_global_property_value::<R0>(1.5);
    context.set_global_property_value::<InfectiousPeriod>(4.0);
    context.set_global_property_value::<InitialInfections>(100);

    context.set_base_random_seed(8675309);

    context.add_component::<IncidenceReport>();
    context.add_component::<PopulationLoader>();
    context.add_component::<InfectionManager>();
    context.add_component::<TransmissionManager>();
    context.add_component::<InfectionSeeder>();

    context.execute();
}
