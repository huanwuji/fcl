use std::rc::Rc;

use crate::func::{FuncA, FuncDef, FuncEntity, Funcs};
use crate::funcs::add::Add;

pub struct FuncManager<'a> {
    funcs: Funcs<'a>
}

impl<'a> FuncManager<'a> {
    pub fn new() -> FuncManager<'a> {
        let mut funcs = Funcs::new();
        let mut manager = FuncManager { funcs };

        let add = Add::new();
        add.register(&mut manager);

        manager
    }

    pub fn register(&mut self, entity: &'a FuncEntity<'a>, func: &'a FuncA) {
        self.funcs.add(entity, func);
    }
}

pub trait FuncReg<'a> {
    fn register(&self, manager: &'a mut FuncManager<'a>);
}

#[test]
fn run() {}