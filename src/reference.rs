use std::rc::Rc;
use std::cell::RefCell;
use std::hash::Hash;
pub struct Reference {
    name: Rc<RefCell<String>>
}

impl PartialEq for Reference {
    fn eq(&self, other: &Self) -> bool {
        self.name.as_ptr() == other.name.as_ptr()
    }
}

impl Hash for Reference {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_usize(self.name.as_ptr() as usize)
    }
}

impl Clone for Reference {
    fn clone(&self) -> Self {
        Self { name: self.name.clone() }
    }
}

impl Reference {
    pub fn new(n: String) -> Self {
        Self { name: Rc::new(RefCell::new(n)) }
    }
}
