use crate::ast::AnyVal;
use crate::func::*;

pub struct AddLL {}

impl<'a> FuncA<'a> for AddLL {
    fn apply1(&self, ctx: &'a Context<'a>, func_def: &'a FuncDef<'a>,
              args: Vec<AnyVal<'a>>, curr: &'a AnyVal<'a>) -> AnyVal<'a> {
        if let &[AnyVal::Long(a), AnyVal::Long(b)] = args.as_slice() {
            let r = self.add(a, b);
            AnyVal::Long(r)
        } else {
            panic!("Not coverd")
        }
    }
}

impl<'a> AddLL {
    pub fn new_def() -> FuncDef<'a> {
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
//    eprintln!("def = {:?}", def);
//    let _result = AddLL {}.apply(&def, &[&2, &3]);
//    println!("{}", stringify!(_result));
//    println!("{}", cast::<i32>(&*_result));
}
