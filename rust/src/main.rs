use fcl::ast::AnyVal;
use fcl::eval::Eval;
use fcl::func::Context;
use fcl::func_mgt::FuncMgt;
use fcl::funcs::add::AddLL;
use fcl::parser::FclParser;

fn main() {
    let funcs = vec![AddLL::new_def()];
    let mut mgt = FuncMgt::new();
    mgt.registers(funcs);
    let parser = FclParser { mgt: &mgt };
    let eval = Eval { mgt: &mgt };
    let ast = parser.ast("add(4,2)");
    let ctx = Context {
        scope: Default::default(),
        mgt: &mgt,
        parser: &parser,
        eval: &eval,
    };
    let r = eval.eval(&ctx, &ast, &AnyVal::None);
    eprintln!("r = {:?}", r);
}