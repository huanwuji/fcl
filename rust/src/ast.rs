use std::any::Any;

use crate::func::FuncDef;

#[derive(Debug, Copy, Clone)]
pub enum AnyVal<'a> {
    None,
    Bool(bool),
    Long(i64),
    Float(f64),
    Str(&'a str),
//    Any { val: Box<Any>, any_type: &'a str },
}

pub struct ValType {}

impl ValType {
    pub const NONE: &'static str = "none";
    pub const BOOL: &'static str = "bool";
    pub const LONG: &'static str = "long";
    pub const FLOAT: &'static str = "float";
    pub const STR: &'static str = "str";
    pub const ANY: &'static str = "any";

    pub fn get_type<'a>(val: &AnyVal<'a>) -> &'a str {
        match val {
            AnyVal::None => ValType::NONE,
            AnyVal::Str(_) => ValType::STR,
            AnyVal::Bool(_) => ValType::BOOL,
            AnyVal::Long(_) => ValType::LONG,
            AnyVal::Float(_) => ValType::FLOAT,
//            AnyVal::Any { any_type, .. } => any_type,
        }
    }
}

#[derive(Debug)]
pub enum AstNode<'a> {
    Val(AnyVal<'a>),
    Var { name: &'a str },
    Func { name: &'a str, args: Vec<AstNode<'a>>, func_def: &'a FuncDef<'a> },
    CurryingFunc { name: &'a str, args: Vec<Vec<AstNode<'a>>>, func_def: &'a FuncDef<'a> },
    FlowFunc { exprs: Vec<AstNode<'a>> },
    Exprs(Vec<AstNode<'a>>),
    FuncEnd,
}

impl<'a> AstNode<'a> {
    const VAR: &'a str = "var";
    pub fn get_type(&self) -> &str {
        match self {
            AstNode::Val(val) => ValType::get_type(val),
            AstNode::Var { .. } => AstNode::VAR,
            AstNode::Func { func_def, .. } => func_def.desc.r_type,
            AstNode::CurryingFunc { func_def, .. } => func_def.desc.r_type,
            AstNode::FlowFunc { exprs } => exprs.last().unwrap().get_type(),
            _ => panic!("Unsupport astNode type")
        }
    }
}