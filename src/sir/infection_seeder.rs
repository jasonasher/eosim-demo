use eosim::{
    context::{Component, Context},
    global_properties::GlobalPropertyContext,
    people::PersonId,
    person_properties::PersonPropertyContext,
    random::{RandomContext, RandomId},
};
use rand::seq::index::sample;
use rand_xoshiro::Xoshiro256PlusPlus;

use super::{
    global_properties::{InitialInfections, Population},
    person_properties::DiseaseStatus,
};

pub struct SeedingRandomId {}

impl RandomId for SeedingRandomId {
    type RngType = Xoshiro256PlusPlus;

    fn seed_offset() -> u64 {
        fxhash::hash64("SeedingRandomId")
    }
}
pub struct InfectionSeeder {}

impl Component for InfectionSeeder {
    fn init(context: &mut Context) {
        let population = *context
            .get_global_property_value::<Population>()
            .expect("Population not specified");
        let initial_infections = *context
            .get_global_property_value::<InitialInfections>()
            .expect("Initial infections not specified.");
        let mut rng = context.get_rng::<SeedingRandomId>();
        let infection_ids = sample(&mut *rng, population, initial_infections);
        drop(rng);
        for id in infection_ids {
            context.set_person_property_value::<DiseaseStatus>(PersonId::new(id), DiseaseStatus::I)
        }
    }
}
