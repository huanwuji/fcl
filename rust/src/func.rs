use std::any::Any;

use crate::ast::AstNode;

#[derive(Debug)]
struct FuncDef<'a> {
    name: &'a str,
    a_types: &'a [&'a str],
    r_type: &'a str,
}

#[macro_export]
macro_rules! def {
    ( $name:ident($($arg:ty),*) -> $out:ty ) => {{
          FuncDef{
              name: stringify!($name),
              a_types: &[$(stringify!($arg)),*],
              r_type: stringify!($out)
          }
     }};
}

struct Context {}

trait Func<'a, C, Out> {
    fn eval(ctx: Context, nodes: Vec<AstNode>, curr: C) -> Out;
    fn apply<T: Any>(ctx: Context, arguments: Vec<T>, curr: C) -> Out;
}

struct Add {}

impl Add {
    const f2: FuncDef<'static> = def!( add(i32,i32) -> i32 );
    fn apply<T>(&self, func_def: FuncDef<'_>, args: &[T]) -> T {
        match args {
            &[a, b] => self.apply1(a, b),
            _ => panic!("Not coverd")
        }
    }

    fn apply1(&self, v1: i32, v2: i32) -> i32 {
        v1 + v2
    }
}

#[test]
fn fn_trait() {
    let context = Context {};
    let def: FuncDef = def!(dd() -> haha);
    let result = Add {}.apply(def, &[2, 3]);
    println!("{}", result);
}

#[test]
fn type_test() {
    let def_str = def!(haha(i32, i64) -> Vec);
    println!("{:?}", def_str);
}
