use eosim::{
    context::{Component, Context},
    people::PersonId,
    person_properties::PersonPropertyContext,
    reports::{Report, ReportsContext},
};
use serde_derive::Serialize;

use super::person_properties::DiseaseStatus;

pub struct DeathReport {}

#[derive(Serialize)]
pub struct Death {
    pub time: f64,
}

impl Report for DeathReport {
    type Item = Death;
}

pub fn handle_person_death(
    context: &mut Context,
    person_id: PersonId,
    _: DiseaseStatus,
) {
    let disease_status = context.get_person_property_value::<DiseaseStatus>(person_id);
    if matches!(disease_status, DiseaseStatus::D) {
        context.release_report_item::<DeathReport>(Death {
            time: context.get_time(),
        })
    }
}

impl Component for DeathReport {
    fn init(context: &mut Context) {
        context
            .observe_person_property_changes::<DiseaseStatus>(handle_person_death);
    }
}
