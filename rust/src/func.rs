use core::fmt;
use std::any::Any;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::intrinsics;
use std::ptr::hash;

use crate::ast::AstNode;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct FuncDef<'a> {
    pub name: &'a str,
    pub a_types: &'a [&'a str],
    pub r_type: &'a str,
    pub func_id: u64,
}

impl<'a> FuncDef<'a> {
    pub fn new(name: &'a str, a_types: &'a [&'a str], r_type: &'a str) -> FuncDef {
        let func_id = FuncDef::func_id(name, a_types);
        FuncDef {
            name,
            a_types,
            r_type,
            func_id,
        }
    }

    fn func_id(name: &str, a_type: &'a [&'a str]) -> u64 {
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        a_types.hash(&mut hasher);
        hasher.finish()
    }
}

impl fmt::Display for FuncDef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}({}) -> {}", self.name, self.a_types.join(","), self.r_type)
    }
}

pub struct Context {}

pub trait FuncA {
    fn eval(ctx: Context, nodes: Vec<AstNode>, curr: Option<&Any>) -> Box<Any>;
    fn apply(&self, _func_def: FuncDef<'_>, args: &[&Any]) -> Box<Any> {
        self.apply1(_func_def, args, None)
    }
    fn apply1(&self, _func_def: FuncDef<'_>, args: &[&Any], curr: Option<&Any>) -> Box<Any>;
}

pub struct Funcs<'a> {
    funcs: &'a mut HashMap<u64, FuncEntity<'a>>
}

pub struct FuncEntity<'a> {
    func_def: FuncDef<'a>,
    func: dyn FuncA,
}

impl<'a> Funcs<'a> {
    pub fn add<F: FuncA>(&self, func_def: FuncDef, func: F) {
        let entity = FuncEntity { func_def, func };
        self.funcs.insert(func_def.func_id, entity);
    }

    pub fn get_entity(&self, func_def: &FuncDef) -> &FuncEntity<'a> {
        self.funcs.get(&func_def.func_id)
            .expect(&format!("Not found func {}", func_def))
    }
}

#[test]
fn type_id() {
    unsafe {
        let type_id = intrinsics::type_id::<i32>();
        eprintln!("type_id = {:?}", type_id);
        let type_name = intrinsics::type_name::<Vec<i32>>();
        eprintln!("type_name = {:?}", type_name);
    }
}