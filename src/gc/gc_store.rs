use std::borrow::BorrowMut;
use std::collections::HashMap;
use crate::gc::gc_key::GcKey;
use crate::gc::gc_values::GcValue;
use crate::interpreter::Interpreter;
use crate::table::Table;
use crate::values::Value;

pub struct GcStore {
    store: HashMap<GcKey, GcValue>,
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

    pub fn collect_garbage(&mut self, stack: &std::collections::VecDeque<crate::table::UserTable>) {
        let mut marked_gc_keys = vec![];
        for s in stack.iter() {
            println!("collect_garbage root iter");
            for (key, value) in s.table.as_ref().borrow().iter() {
                match key {
                    &Value::Table(ref gc_key) => {
                        println!("Key is table");
                        let gc_value = self.get_value(gc_key).unwrap();
                        if let GcValue::Table(gct) = gc_value {
                            marked_gc_keys.push(gc_key.clone());
                            let newly_marked_keys = &mut self.collect_garbage_from(gct, &mut marked_gc_keys);
                            marked_gc_keys.append(newly_marked_keys);
                        }
                    },
                    _ => {}
                };
                match value {
                    &Value::Table(ref gc_key) => {
                        println!("Value is table");
                        let gc_value = self.get_value(gc_key).unwrap();
                        if let GcValue::Table(gct) = gc_value {
                            marked_gc_keys.push(gc_key.clone());
                            let newly_marked_keys = &mut self.collect_garbage_from(gct, &mut marked_gc_keys);
                            marked_gc_keys.append(newly_marked_keys);
                        }
                    },
                    _ => {}
                }
            }
        }
        let len_before_collect = self.store.len();
        println!("We found {} garbage collectable objects through marking, a total of {} have been allocated", marked_gc_keys.len(), self.store.len());
        self.store.retain(|key, value| {
            marked_gc_keys.contains(key)
        });
        println!("Removed {} element(s)", len_before_collect - self.store.len());
    }

    fn collect_garbage_from(&self, table: &Table, visited_list: &mut Vec<GcKey>) -> Vec<GcKey> {
        let mut res = vec![];
        for (key, value) in table.iter() {
            match key {
                &Value::Table(ref gck) => {
                    if visited_list.contains(gck) {
                        return res;
                    }
                    if let Some(GcValue::Table(child_table)) = self.get_value(&gck) {
                        res.append(&mut self.collect_garbage_from(child_table, visited_list));
                    }
                    res.push(gck.clone());
                },
                _ => {}
            };
            match value {
                &Value::Table(ref gck) => {
                    if visited_list.contains(gck) {
                        return res;
                    }
                    if let Some(GcValue::Table(child_table)) = self.get_value(&gck) {
                        res.append(&mut self.collect_garbage_from(child_table, visited_list));
                    }
                    res.push(gck.clone());
                },
                _ => {}
            };

        }
        return res;
    }
}