use std::{hash::Hash, rc::Rc, borrow::{Borrow, BorrowMut}, collections::VecDeque, cell::RefCell};

use crate::{stmt::Stmt, expr::Expr, table::UserTable};

#[derive(Clone)]
pub struct Function {
    fi: Rc<RefCell<FunctionImpl>>
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        let self_addr = self.fi.as_ptr() as *const FunctionImpl as usize;
        let other_addr = other.fi.as_ptr() as *const FunctionImpl as usize;
        return self_addr == other_addr;
    }
}

impl Eq for Function {}

impl Function {
    pub fn new(body: Box<Stmt>, params: Vec<Expr>, name: Option<String>, closure: VecDeque<UserTable>) -> Self {
        Self { fi: Rc::new(RefCell::new(FunctionImpl::new(body, params, name, closure))) }
    }

    pub fn get_name(&self) -> Option<String> {
        self.fi.as_ref().borrow().name.clone()
    }

    pub fn get_params(&self) -> Vec<Expr> {
        self.fi.as_ref().borrow().params.clone()
    }

    pub fn get_body(&self) -> Stmt {
        *self.fi.as_ref().borrow().body.clone()
    }

    pub fn get_closure(&self) -> VecDeque<UserTable> {
        self.fi.as_ref().borrow().closure.clone()
    }

    pub fn set_closure(&mut self, new_closure: VecDeque<UserTable>) {
        self.fi.as_ref().borrow_mut().closure.borrow_mut().clear();
        for nc in new_closure {
            self.fi.as_ref().borrow_mut().closure.push_back(nc);
        }
    }
}

pub struct FunctionImpl {
    pub body: Box<Stmt>,
    pub params: Vec<Expr>,
    pub name: Option<String>,
    pub closure: VecDeque<UserTable>,
}

impl Hash for Function {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let to_hash = self as *const Function as usize;
        state.write_usize(to_hash);
    }
}

impl Clone for FunctionImpl {
    fn clone(&self) -> Self {
        return Self { body: self.body.clone(), params: self.params.clone(), name: self.name.clone(), closure: self.closure.clone() }
    }
}

impl FunctionImpl {

    pub fn new(body: Box<Stmt>, params: Vec<Expr>, name: Option<String>, closure: VecDeque<UserTable>) -> Self {
        let this = Self {
            body,
            params,
            name,
            closure
        };
        this
    }
}
