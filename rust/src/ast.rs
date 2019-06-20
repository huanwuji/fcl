use std::rc::Rc;

use dynamic::Dynamic;

use crate::func::FuncDef;

#[derive(Debug, Clone)]
pub enum AnyVal {
    None,
    Bool(bool),
    Long(i64),
    Float(f64),
    Str(String),
    Any { val: Rc<Box<Dynamic>>, s_type: String },
}

impl AnyVal {
    pub fn get_type(&self) -> &str {
        AnyVal::get_val_type(self)
    }

    pub fn get_val_type(val: &AnyVal) -> &str {
        match val {
            AnyVal::None => Types::NONE,
            AnyVal::Str(_) => Types::STR,
            AnyVal::Bool(_) => Types::BOOL,
            AnyVal::Long(_) => Types::LONG,
            AnyVal::Float(_) => Types::FLOAT,
            AnyVal::Any { s_type, .. } => s_type,
        }
    }
}

pub struct Types {}

impl Types {
    pub const NONE: &'static str = "none";
    pub const BOOL: &'static str = "bool";
    pub const LONG: &'static str = "long";
    pub const FLOAT: &'static str = "float";
    pub const STR: &'static str = "str";
    pub const ANY: &'static str = "any";
    pub const OMITS: &'static str = "..";
}

#[derive(Debug, Clone)]
pub enum AstNode<'a> {
    Curr { v_type: String },
    Val { value: AnyVal, v_type: String },
    Var { name: String, v_type: String },
    Func { name: String, args: Vec<AstNode<'a>>, func_def: &'a FuncDef, v_type: String },
    CurryingFunc { name: String, args: Vec<Vec<AstNode<'a>>>, func_def: &'a FuncDef, v_type: String },
    FlowFunc { exprs: Vec<AstNode<'a>>, v_type: String },
    Exprs { exprs: Vec<AstNode<'a>>, v_type: String },
    FuncEnd { v_type: String },
    None,
}

impl<'a> AstNode<'a> {
    pub fn get_v_type(&self) -> String {
        match self {
            AstNode::Curr { v_type } => v_type.clone(),
            AstNode::Val { v_type, .. } => v_type.clone(),
            AstNode::Var { v_type, .. } => v_type.clone(),
            AstNode::Func { v_type, .. } => v_type.clone(),
            AstNode::CurryingFunc { v_type, .. } => v_type.clone(),
            AstNode::FlowFunc { v_type, .. } => v_type.clone(),
            AstNode::FuncEnd { v_type, .. } => v_type.clone(),
            _ => panic!("Unsupport astNode type")
        }
    }
}