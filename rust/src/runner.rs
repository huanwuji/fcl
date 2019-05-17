use std::any::Any;

use crate::ast::AstNode;

struct Context {}

trait Func {
    fn eval(ctx: Context, nodes: Vec<AstNode>) -> impl Any;
    fn apply<T: Any>(ctx: Context, arguments: Vec<T>) -> impl Any;
}

#[test]
fn type_test() {
    let tt = test { t: 11 as i64 };
    eprintln!("tt.haha() = {:?}", tt.haha());
}
