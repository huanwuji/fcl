use std::any::Any;

use crate::ast::AstNode;
use crate::func::*;
use crate::types::*;

pub struct AddLL {}

impl FuncA for AddLL {
    fn eval(&self, ctx: Context, nodes: Vec<AstNode>, curr: Option<&Any>) -> Box<Any> {
        unimplemented!()
    }

    fn apply1(&self, _func_def: &FuncDesc, args: &[&Any], curr: Option<&Any>) -> Box<Any> {
        if let &[a, b] = args {
            let r = self.add(*cast::<i64>(a), *cast::<i64>(b));
            Box::new(r)
        } else {
            panic!("Not coverd")
        }
    }
}

impl AddLL {
    pub fn new_def() -> FuncDef<'static> {
        FuncDef {
            desc: def!( add(long, long) -> long ),
            func: Box::new(AddLL::new()),
        }
    }

    pub fn new() -> AddLL {
        AddLL {}
    }

    fn add(&self, v1: i64, v2: i64) -> i64 {
        v1 + v2
    }
}

#[test]
fn add_test() {
//    let ref ADD_LL: FuncDesc<'static> = def!( add(i32, i32) -> i32 );
    let def: FuncDesc = def!( add(i32, i32) -> i32 );
    eprintln!("def = {:?}", def);
    let _result = AddLL {}.apply(&def, &[&2, &3]);
//    println!("{}", stringify!(_result));
    println!("{}", cast::<i32>(&*_result));
}
