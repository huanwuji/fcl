use crate::func::FuncDef;

pub enum AnyVal<'a> {
    Bool(bool),
    Long(i64),
    Float(f64),
    Str(&'a str),
}

pub struct ValType<'a> {}

impl<'a> ValType<'a> {
    const BOOL: &'a str = "bool";
    const LONG: &'a str = "long";
    const FLOAT: &'a str = "float";
    const STR: &'a str = "str";

    pub fn get_type(val: &AnyVal<'a>) -> &'a str {
        match val {
            Str(_) => ValType::STR,
            Bool => ValType::BOOL,
            Long => ValType::LONG,
            Float => ValType::FLOAT,
        }
    }
}

#[derive(Debug)]
pub enum AstNode<'a> {
    Val(AnyVal<'a>),
    Var { name: &'a str },
    Func { name: &'a str, args: Vec<AstNode<'a>>, func_def: FuncDef<'a> },
    CurryingFunc { name: &'a str, args: Vec<Vec<AstNode<'a>>>, func_def: FuncDef<'a> },
    FlowFunc { exprs: Vec<AstNode<'a>> },
    Exprs(Vec<AstNode<'a>>),
    VOID,
    FuncEnd,
}

impl<'a> AstNode<'a> {
    const VAR: &'a str = "var";
    fn get_type(&self) -> &'a str {
        match self {
            AstNode::Val(val) => ValType::get_type(val),
            AstNode::Var { name } => AstNode::VAR,
            AstNode::Func { name, args, func_def } => func_def.r_type,
            AstNode::CurryingFunc { name, args, func_def } => func_def.r_type,
            AstNode::FlowFunc { exprs } => exprs.last().unwrap().get_type(),
            _ => panic!("Unsupport astNode type")
        }
    }
}