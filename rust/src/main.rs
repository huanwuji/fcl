use std::time::SystemTime;

use fcl::ast::AnyVal;
use fcl::fcl::Fcl;

fn main() {
    let fcl = Fcl::new();
    let result = fcl.eval_str("+(1,2).+(_, +(1,2));+(1,2).+(_,4)");
    eprintln!("result = {:?}", result);
}

#[test]
fn performance_test() {
    test_add();
    test_add();
}

fn test_add() {
    let fcl = Fcl::new();
    let ast = fcl.ast("+(1,2).+(_, +(1,2))");
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