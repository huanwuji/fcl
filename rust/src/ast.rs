use std::any::Any;

use crate::func::FuncDef;

#[derive(Debug, Copy, Clone)]
pub enum AnyVal {
    None,
    Bool(bool),
    Long(i64),
    Float(f64),
    Str(&'static str),
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

    pub fn get_type(val: &AnyVal) -> &str {
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
pub enum AstNode {
    Val(AnyVal),
    Var { name: &'static str },
    Func { name: &'static str, args: Vec<AstNode>, func_def: &'static FuncDef },
    CurryingFunc { name: &'static str, args: Vec<Vec<AstNode>>, func_def: &'static FuncDef },
    FlowFunc { exprs: Vec<AstNode> },
    Exprs(Vec<AstNode>),
    FuncEnd,
}

impl AstNode {
    const VAR: &'static str = "var";
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