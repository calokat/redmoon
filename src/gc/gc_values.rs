use crate::{table::Table, vm::VmEnv};

#[derive(Clone)]
pub enum GcValue {
    Table(Table),
    Process(VmEnv),
}
