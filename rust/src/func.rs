use std::any::{Any, TypeId};
use std::intrinsics;
use std::intrinsics::sub_with_overflow;

use crate::ast::AstNode;
use crate::types::*;

#[derive(Debug, PartialEq, PartialOrd, Hash)]
struct FuncDef<'a> {
    name: &'a str,
    a_types: &'a [&'a str],
    r_type: &'a str,
}

#[macro_export]
macro_rules! def {
     ( $name:ident($($arg:ty),* ) -> $out:ty ) => {{
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
    const f2: FuncDef<'static> = def!( add(i32, i32) -> i32 );
    fn apply(&self, _func_def: FuncDef<'_>, args: &[&Any]) -> impl Any {
        match _func_def {
            f2 =>
                match args {
                    &[a, b] =>
                        self.apply1(*cast::<i32>(a), *cast::<i32>(b)),
                    _ => panic!("Not coverd")
                },
            _ => panic!("Not coverd")
        }
    }

    fn apply1(&self, v1: i32, v2: i32) -> i32 {
        v1 + v2
    }
}

#[test]
fn fn_trait() {
    let def: FuncDef = def!( add(i32, i32) -> i32 );
    let _result = Add {}.apply(def, &[&2, &3]);
//    println!("{}", stringify!(_result));
    println!("{}", cast::<i32>(&_result));
}

#[test]
fn type_test() {
    let def_str = def!(haha(i32, i64) -> Vec);
    println!("{:?}", def_str);
}

#[test]
fn test_derive() {
    let def1: FuncDef = def!(dd(i32, i64) -> haha);
    let def2: FuncDef = def!(dd(i32, i64) -> haha);
    eprintln!("{}", def1 == def2);
}

#[test]
fn type_id() {
//    intrinsics

//    intrinsics::
    unsafe {
        let type_id = intrinsics::type_id::<i32>();
        eprintln!("type_id = {:?}", type_id);
        let type_name = intrinsics::type_name::<Vec<i32>>();
        eprintln!("type_name = {:?}", type_name);
    }
}