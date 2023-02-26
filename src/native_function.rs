use std::rc::Rc;
use core::hash::Hash;
use crate::{Interpreter, Value};

pub type NativeFunctionImpl = dyn Fn(&Interpreter, &mut Vec<Value>) -> Option<Value>;

pub struct NativeFunction {
    nfi: Rc<Box<NativeFunctionImpl>>
}

impl PartialEq for NativeFunction {
    fn eq(&self, other: &Self) -> bool {
        let self_addr = self.nfi.as_ref().as_ref() as *const NativeFunctionImpl as *const() as usize;
        let other_addr = other.nfi.as_ref().as_ref() as *const NativeFunctionImpl as *const() as usize;
        return self_addr == other_addr;
    }
}

impl Eq for NativeFunction {}

impl Hash for NativeFunction {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let to_hash = self.nfi.as_ref().as_ref() as *const NativeFunctionImpl as *const() as usize;
        state.write_usize(to_hash);
    }
}

impl Clone for NativeFunction {
    fn clone(&self) -> Self {
        return Self { nfi: self.nfi.clone() }
    }
}

impl NativeFunction {
    pub fn new(closure: Box<NativeFunctionImpl>) -> Self {
        Self { nfi: Rc::new(closure) }
    }
    pub fn call(&self, interp: &Interpreter, args: &mut Vec<Value>) -> Option<Value> {
        self.nfi.as_ref().as_ref()(interp, args)
    }
}
