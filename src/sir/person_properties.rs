use eosim::{data_containers::PropertyWithDefault, person_properties::PersonProperty};

pub enum DiseaseStatus {
    S,
    I,
    R,
}
eosim::define_person_property_from_enum!(DiseaseStatus, DiseaseStatus::S);
