use std::any::Any;
use std::intrinsics;
use std::intrinsics::sub_with_overflow;

use crate::ast::AstNode;
use crate::types::*;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct FuncDef<'a> {
    pub name: &'a str,
    pub a_types: &'a [&'a str],
    pub r_type: &'a str,
}

pub struct Context {}

pub trait FuncA {
    fn eval(ctx: Context, nodes: Vec<AstNode>, curr: Option<&Any>) -> Box<Any>;
    fn apply(&self, _func_def: FuncDef<'_>, args: &[&Any]) -> Box<Any> {
        self.apply1(_func_def, args, None)
    }
    fn apply1(&self, _func_def: FuncDef<'_>, args: &[&Any], curr: Option<&Any>) -> Box<Any>;
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