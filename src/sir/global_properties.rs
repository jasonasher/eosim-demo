use eosim::{data_containers::Property, global_properties::GlobalProperty};

#[derive(Eq, PartialEq, Hash)]
pub struct R0 {}
impl Property for R0 {
    type Value = f64;
}
impl GlobalProperty for R0 {}

#[derive(Eq, PartialEq, Hash)]
pub struct InfectiousPeriod {}
impl Property for InfectiousPeriod {
    type Value = f64;
}
impl GlobalProperty for InfectiousPeriod {}

#[derive(Eq, PartialEq, Hash)]
pub struct Population {}
impl Property for Population {
    type Value = usize;
}
impl GlobalProperty for Population {}

#[derive(Eq, PartialEq, Hash)]
pub struct InitialInfections {}
impl Property for InitialInfections {
    type Value = usize;
}
impl GlobalProperty for InitialInfections {}
