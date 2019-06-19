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

pub struct ValType {}

impl ValType {
    pub const NONE: &'static str = "none";
    pub const BOOL: &'static str = "bool";
    pub const LONG: &'static str = "long";
    pub const FLOAT: &'static str = "float";
    pub const STR: &'static str = "str";
    pub const ANY: &'static str = "any";
    pub const VAR: &'static str = "var";
    pub const CURR: &'static str = "curr";

    pub fn get_type(val: &AnyVal) -> &str {
        match val {
            AnyVal::None => ValType::NONE,
            AnyVal::Str(_) => ValType::STR,
            AnyVal::Bool(_) => ValType::BOOL,
            AnyVal::Long(_) => ValType::LONG,
            AnyVal::Float(_) => ValType::FLOAT,
            AnyVal::Any { s_type, .. } => s_type,
        }
    }
}

#[derive(Debug, Clone)]
pub enum AstNode<'a> {
    Curr,
    Val(AnyVal),
    Var { name: String },
    Func { name: String, args: Vec<AstNode<'a>>, func_def: &'a FuncDef },
    CurryingFunc { name: String, args: Vec<Vec<AstNode<'a>>>, func_def: &'a FuncDef },
    FlowFunc { exprs: Vec<AstNode<'a>> },
    Exprs(Vec<AstNode<'a>>),
    FuncEnd,
    None,
}

impl<'a> AstNode<'a> {
    pub fn get_r_type(&self) -> String {
        AstNode::get_r_type_raw(self, None)
    }

    pub fn get_r_type_raw<'c>(node: &'c AstNode, curr_type: Option<&'c str>) -> String {
        match node {
            AstNode::Curr => {
                String::from(curr_type
                    .unwrap_or_else(|| panic!("Can't found curr type")))
            }
            AstNode::Val(val) => String::from(ValType::get_type(val)),
            AstNode::Var { .. } => String::from(ValType::VAR),
            AstNode::Func { func_def, .. } => func_def.desc.r_type.clone(),
            AstNode::CurryingFunc { func_def, .. } => func_def.desc.r_type.clone(),
            AstNode::FlowFunc { exprs } => exprs.last().unwrap().get_r_type(),
            _ => panic!("Unsupport astNode type")
        }
    }
}