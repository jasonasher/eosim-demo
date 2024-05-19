use eosim::{
    context::{Component, Context},
    global_properties::GlobalPropertyContext,
    people::PersonId,
    person_properties::PersonPropertyContext,
    random::{RandomContext, RandomId},
};
use rand_distr::{Distribution, Exp};
use rand_xoshiro::Xoshiro256PlusPlus;

use super::{global_properties::InfectiousPeriod, person_properties::DiseaseStatus};

pub struct InfectionManager {}

pub struct InfectionRandomId {}

impl RandomId for InfectionRandomId {
    type RngType = Xoshiro256PlusPlus;

    fn seed_offset() -> u64 {
        fxhash::hash64("InfectionRandomId")
    }
}

pub fn handle_person_disease_status_change(
    context: &mut Context,
    person_id: PersonId,
    _: DiseaseStatus,
) {
    let disease_status = context.get_person_property_value::<DiseaseStatus>(person_id);
    if matches!(disease_status, DiseaseStatus::I) {
        schedule_recovery(context, person_id)
    }
}

pub fn schedule_recovery(context: &mut Context, person_id: PersonId) {
    let infectious_period = *context
        .get_global_property_value::<InfectiousPeriod>()
        .expect("Infectious Period not Specified");
    let infectious_period_dist = Exp::new(1.0 / infectious_period).unwrap();
    let recovery_time = context.get_time()
        + infectious_period_dist.sample(&mut *context.get_rng::<InfectionRandomId>());
    context.add_plan(recovery_time, move |context| {
        context.set_person_property_value::<DiseaseStatus>(person_id, DiseaseStatus::R);
    });
}

impl Component for InfectionManager {
    fn init(context: &mut Context) {
        context
            .observe_person_property_changes::<DiseaseStatus>(handle_person_disease_status_change);
    }
}
