use std::any::Any;
use std::fmt::Debug;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

use dynamic::Dynamic;

use fcl::ast::AnyVal;
use fcl::fcl::Fcl;
use fcl::types;

fn main() {
    let fcl = Fcl::new();
    let result = fcl.eval_str("add(1,2).add(_, add(1,2))");
    eprintln!("result = {:?}", result);
}

#[test]
fn performance_test() {
    let fcl = Fcl::new();
    let ast = fcl.ast("add(4,2)");
    let ctx = fcl.new_context();
    let mut i = 100000;
    let now = SystemTime::now();
    while i > 0 {
        let r = fcl.eval_ctx(&ctx, &ast, &AnyVal::None);
//        eprintln!("r = {:?}", r);
        i -= 1;
    }
    eprintln!("elasped = {:?}", now.elapsed().unwrap());
}