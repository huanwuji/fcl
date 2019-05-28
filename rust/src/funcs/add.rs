use std::any::Any;

use crate::ast::AstNode;
use crate::func::*;
use crate::types::*;

struct Add {}

impl Add {
    fn add(&self, v1: i32, v2: i32) -> i32 {
        v1 + v2
    }
}

const F2: FuncDef<'static> = def!( add(i32, i32) -> i32 );

impl FuncA for Add {
    fn eval(ctx: Context, nodes: Vec<AstNode>, curr: Option<&Any>) -> Box<Any> {
        unimplemented!()
    }

    fn apply1(&self, _func_def: FuncDef, args: &[&Any], curr: Option<&Any>) -> Box<Any> {
        match _func_def {
            F2 =>
                match args {
                    &[a, b] => {
                        let r = self.add(*cast::<i32>(a), *cast::<i32>(b));
                        Box::new(r)
                    }
                    _ => panic!("Not coverd")
                },
            _ => panic!("Not coverd")
        }
    }
}

#[test]
fn add_test() {
    let def: FuncDef = def!( add(i32, i32) -> i32 );
    let _result = Add {}.apply(def, &[&2, &3]);
//    println!("{}", stringify!(_result));
    println!("{}", cast::<i32>(&*_result));
}
