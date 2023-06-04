use std::{hash::Hash, rc::Rc};
#[derive(Eq, Clone)]
pub struct GcKey {
    _rc: Rc<u8>
}

impl GcKey {
    pub fn new() -> Self {
        Self { _rc: Rc::new(0) }
    }
}

impl Hash for GcKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let addr: usize = self._rc.as_ref() as *const u8 as usize;
        state.write_usize(addr);
    }
}

impl PartialEq for GcKey {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self._rc, &other._rc)
    }
}