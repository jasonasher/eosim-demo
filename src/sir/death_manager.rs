use eosim::{
    context::{Component, Context},
    global_properties::GlobalPropertyContext,
    people::PersonId,
    person_properties::PersonPropertyContext,
    random::{RandomContext, RandomId},
};
use rand::Rng;
use rand_xoshiro::Xoshiro256PlusPlus;

use super::{global_properties::DeathRate, person_properties::DiseaseStatus};

pub struct DeathManager {}

impl Component for DeathManager {
    fn init(context: &mut Context) {
        context
            .observe_person_property_changes::<DiseaseStatus>(handle_person_disease_status_change);
    }
}

pub struct DeathRandomId {}

impl RandomId for DeathRandomId {
    type RngType = Xoshiro256PlusPlus;

    fn seed_offset() -> u64 {
        fxhash::hash64("DeathRandomId")
    }
}

pub fn handle_person_disease_status_change(
    context: &mut Context,
    person_id: PersonId,
    _: DiseaseStatus,
) {
    let disease_status = context.get_person_property_value::<DiseaseStatus>(person_id);
    if matches!(disease_status, DiseaseStatus::I) {
        schedule_death_check(context, person_id);
    }
}

pub fn schedule_death_check(context: &mut Context, person_id: PersonId) {
    let death_rate = *context
        .get_global_property_value::<DeathRate>()
        .expect("Death Rate not specified");
    let mut rng = context.get_rng::<DeathRandomId>();
    let should_die = rng.gen::<f64>() < death_rate;
    drop(rng);
    if should_die {
        context.set_person_property_value::<DiseaseStatus>(person_id, DiseaseStatus::D);
    }
}