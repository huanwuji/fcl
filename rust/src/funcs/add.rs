use std::any::Any;

use crate::ast::AstNode;
use crate::func::*;
use crate::types::*;

lazy_static! {
    static ref ADD_LL: FuncDesc<'static> = def!( add(i32, i32) -> i32 );
}

pub struct Add {}

impl FuncA for Add {
    #[allow(unused)]
    fn eval(&self, ctx: Context, nodes: Vec<AstNode>, curr: Option<&Any>) -> Box<Any> {
        unimplemented!()
    }
    #[allow(unused)]
    fn apply1(&self, _func_def: &FuncDesc, args: &[&Any], curr: Option<&Any>) -> Box<Any> {
        if _func_def.fid == ADD_LL.fid {
            match args {
                &[a, b] => {
                    let r = self.add(*cast::<i32>(a), *cast::<i32>(b));
                    Box::new(r)
                }
                _ => panic!("Not coverd")
            }
        } else {
            panic!("Not coverd")
        }
    }
}

impl Add {
    pub fn new() -> Add {
        Add {}
    }
    fn add(&self, v1: i32, v2: i32) -> i32 {
        v1 + v2
    }
}

#[test]
fn add_test() {
    let def: FuncDesc = def!( add(i32, i32) -> i32 );
    eprintln!("def = {:?}", def);
    let _result = Add {}.apply(&def, &[&2, &3]);
//    println!("{}", stringify!(_result));
    println!("{}", cast::<i32>(&*_result));
}
