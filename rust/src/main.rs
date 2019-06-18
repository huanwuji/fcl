use std::time::SystemTime;

use fcl::ast::AnyVal;
use fcl::fcl::Fcl;

fn main() {
    let fcl = Fcl::new();
    let ast = fcl.ast("add(4,2)");
    let ctx = fcl.new_context();
    let mut i = 100000;
    let now = SystemTime::now();
    while i > 0 {
        let r = fcl.eval_ctx(&ctx, &ast, &AnyVal::None);
        i -= 1;
    }
    eprintln!("elasped = {:?}", now.elapsed().unwrap());
}