use std::rc::Rc;

use crate::func::{FuncDef, FuncEntity};

#[derive(Debug)]
pub enum AnyVal<'a> {
    Bool(bool),
    Long(i64),
    Float(f64),
    Str(&'a str),
}

pub struct ValType {}

impl<'a> ValType {
    const BOOL: &'static str = "bool";
    const LONG: &'static str = "long";
    const FLOAT: &'static str = "float";
    const STR: &'static str = "str";

    pub fn get_type(val: &AnyVal<'a>) -> &'a str {
        match val {
            AnyVal::Str(_) => ValType::STR,
            AnyVal::Bool(_) => ValType::BOOL,
            AnyVal::Long(_) => ValType::LONG,
            AnyVal::Float(_) => ValType::FLOAT,
        }
    }
}

#[derive(Debug)]
pub enum AstNode<'a> {
    Val(AnyVal<'a>),
    Var { name: &'a str },
    Func { name: &'a str, args: Vec<AstNode<'a>>, entity: &'a FuncEntity<'a> },
    CurryingFunc { name: &'a str, args: Vec<Vec<AstNode<'a>>>, entity: &'a FuncEntity<'a> },
    FlowFunc { exprs: Vec<AstNode<'a>> },
    Exprs(Vec<AstNode<'a>>),
    VOID,
    FuncEnd,
}

impl<'a> AstNode<'a> {
    const VAR: &'a str = "var";
    pub fn get_type(&self) -> &str {
        match self {
            AstNode::Val(val) => ValType::get_type(val),
            AstNode::Var { .. } => AstNode::VAR,
            AstNode::Func { entity, .. } => entity.func_def.r_type,
            AstNode::CurryingFunc { entity, .. } => entity.func_def.r_type,
            AstNode::FlowFunc { exprs } => exprs.last().unwrap().get_type(),
            _ => panic!("Unsupport astNode type")
        }
    }
}