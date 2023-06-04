use std::collections::HashMap;
use crate::gc::gc_key::GcKey;
use crate::gc::gc_values::GcValue;
pub struct GcStore {
    store: HashMap<GcKey, GcValue>
}

impl GcStore {

    pub fn new() -> Self {
        GcStore { store: HashMap::new() }
    }

    pub fn store(&mut self, key: GcKey, value: GcValue) {
        self.store.insert(key, value);
    }

    pub fn get_value(&self, key: &GcKey) -> Option<&GcValue> {
        self.store.get(key)
    }

    pub fn modify_value(&mut self, key: &GcKey) -> Option<&mut GcValue> {
        self.store.get_mut(&key)
    }
}