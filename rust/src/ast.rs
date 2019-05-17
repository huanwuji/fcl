use crate::parser;

#[derive(Debug)]
pub enum AnyVal<'a> {
    Str(&'a str),
    Bool(bool),
    Long(i64),
    Float(f64),
    Null,
}

#[derive(Debug)]
pub enum AstNode<'a> {
    Empty,
    Val(AnyVal<'a>),
    Var { name: &'a str },
    Func { name: &'a str, args: Vec<AstNode<'a>> },
    FlowFunc { exprs: Vec<AstNode<'a>> },
    CurryingFunc { name: &'a str, args: Vec<Vec<AstNode<'a>>> },
    FuncEnd,
    Exprs(Vec<AstNode<'a>>),
}

pub fn parse(str: &str) -> AstNode {
    parser::parse(str)
}

#[test]
fn parse_test() {
    let ast = parse("f1().f2(12,3)");
    eprintln!("ast = {:?}", ast);
}