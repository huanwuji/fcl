use crate::ast::AnyVal;
use crate::fcl::Fcl;
use crate::func::*;
use crate::func_mgt::FuncMgt;

pub struct AddLL {}

impl FuncA for AddLL {
    fn apply1(&self, ctx: &Context, func_def: &FuncDef,
              args: Vec<AnyVal>, curr: &AnyVal) -> AnyVal {
        if let &[AnyVal::Long(a), AnyVal::Long(b)] = args.as_slice() {
            AnyVal::Long(a + b)
        } else {
            panic!("Unsupport type")
        }
    }
}

pub struct AddFF {}

impl FuncA for AddFF {
    fn apply1(&self, ctx: &Context, func_def: &FuncDef,
              args: Vec<AnyVal>, curr: &AnyVal) -> AnyVal {
        if let &[AnyVal::Float(a), AnyVal::Float(b)] = args.as_slice() {
            AnyVal::Float(a + b)
        } else {
            panic!("Unsupport type")
        }
    }
}

pub struct Add {}

impl Add {
    pub fn register(mgt: &mut FuncMgt) {
        let funcs = vec![
            FuncDef {
                desc: def!( +(long, long) -> long ),
                func: Box::new(AddLL {}),
            },
            FuncDef {
                desc: def!( +(float, float) -> float ),
                func: Box::new(AddFF {}),
            }];
        mgt.registers(funcs);
    }
}

#[test]
fn add_ll() {
    let fcl = Fcl::new();
    let result = fcl.eval_str("+(1,2)");
    eprintln!("result = {:?}", result);
}

#[test]
fn add_ff() {
    let fcl = Fcl::new();
    let result = fcl.eval_str("+(1.0,2.0)");
    eprintln!("result = {:?}", result);
}
