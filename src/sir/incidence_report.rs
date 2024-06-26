use eosim::{
    context::{Component, Context},
    people::PersonId,
    person_properties::PersonPropertyContext,
    reports::{Report, ReportsContext},
};
use serde_derive::Serialize;

use super::person_properties::DiseaseStatus;

pub struct IncidenceReport {}

#[derive(Serialize)]
pub struct Infection {
    pub time: f64,
}

impl Report for IncidenceReport {
    type Item = Infection;
}

pub fn handle_person_disease_status_change(
    context: &mut Context,
    person_id: PersonId,
    _: DiseaseStatus,
) {
    let disease_status = context.get_person_property_value::<DiseaseStatus>(person_id);
    if matches!(disease_status, DiseaseStatus::I) {
        context.release_report_item::<IncidenceReport>(Infection {
            time: context.get_time(),
        })
    }
}

impl Component for IncidenceReport {
    fn init(context: &mut Context) {
        context
            .observe_person_property_changes::<DiseaseStatus>(handle_person_disease_status_change);
    }
}
