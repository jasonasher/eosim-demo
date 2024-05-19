use eosim::{data_containers::Property, global_properties::GlobalProperty};

eosim::define_global_property!(R0, f64);

eosim::define_global_property!(InfectiousPeriod, f64);

eosim::define_global_property!(Population, usize);

eosim::define_global_property!(InitialInfections, usize);
