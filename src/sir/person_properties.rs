use eosim::{data_containers::PropertyWithDefault, person_properties::PersonProperty};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DiseaseStatus {
    S,
    I,
    R,
}
impl PropertyWithDefault for DiseaseStatus {
    type Value = DiseaseStatus;

    fn get_default() -> Self::Value {
        DiseaseStatus::S
    }
}
impl PersonProperty for DiseaseStatus {}
