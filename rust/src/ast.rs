use crate::func::FuncDef;

#[derive(Debug)]
pub enum AnyVal {
    Bool(bool),
    Long(i64),
    Float(f64),
    Str(&'static str),
}

pub struct ValType {}

impl ValType {
    const BOOL: &'static str = "bool";
    const LONG: &'static str = "long";
    const FLOAT: &'static str = "float";
    const STR: &'static str = "str";

    pub fn get_type(val: &AnyVal) -> &str {
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
    Val(AnyVal),
    Var { name: &'a str },
    Func { name: &'a str, args: Vec<AstNode<'a>>, func_def: &'a FuncDef<'a> },
    CurryingFunc { name: &'a str, args: Vec<Vec<AstNode<'a>>>, func_def: &'a FuncDef<'a> },
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
            AstNode::Func { func_def, .. } => func_def.desc.r_type,
            AstNode::CurryingFunc { func_def, .. } => func_def.desc.r_type,
            AstNode::FlowFunc { exprs } => exprs.last().unwrap().get_type(),
            _ => panic!("Unsupport astNode type")
        }
    }
}