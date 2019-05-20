use std::any::Any;

use crate::ast::AstNode;

struct Context {}

#[derive(Debug)]
struct FuncStruct<'a> {
    name: &'a str,
    a_types: &'a [&'a str],
    r_type: &'a str,
}

#[macro_export]
macro_rules! def {
    ( $name:ident($($arg:ty),* ) -> $out:ty ) => {{
//        let mut string = String::from(stringify!($name));
//        string.push_str("(");
//        $(
//            string.push_str(stringify!($arg));
//            string.push_str(",");
//        )*
//        string.push_str(") -> ");
//        string.push_str(stringify!($out));
//        string
     }};
}

#[test]
fn type_test() {
    let def_str = def!(haha(i32, i64) -> Vec);
    println!("{}", def_str);
}

trait Func<C, Out> {
    fn eval(ctx: Context, nodes: Vec<AstNode>, curr: C) -> Out;
    fn apply<T: Any>(ctx: Context, arguments: Vec<T>, curr: C) -> Out;
}