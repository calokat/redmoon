use std::{collections::HashMap, rc::Rc, hash::Hash, borrow::BorrowMut, cell::RefCell};
use crate::Value;

pub type Table = HashMap<Value, Value>;

// struct representing tables that can be created by Lua code. This wraps the internal Table type.
#[derive(Clone)]
pub struct UserTable {
    pub table: Rc<RefCell<Table>>
}

impl PartialEq for UserTable {
    fn eq(&self, other: &Self) -> bool {
        let self_addr = self.table.as_ptr() as *const Table as usize;
        let other_addr = other.table.as_ptr() as *const Table as usize;
        return self_addr == other_addr;
    }
}

impl Eq for UserTable {}

impl Hash for UserTable {
 fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    let self_addr = self.table.as_ptr() as *const Table as usize;
    state.write_usize(self_addr);
 }   
}

impl UserTable {
    pub fn new() -> Self {
        Self { table: Rc::new(RefCell::new(HashMap::new())) }
    }
}