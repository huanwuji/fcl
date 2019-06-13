use fcl::func_mgt::FuncMgt;
use fcl::funcs::add::AddLL;
use fcl::parser::FclParser;
use fcl::runner::Runner;

fn main() {
    let funcs = vec![AddLL::new_def()];
    let mut mgt = FuncMgt::new();
    mgt.registers(funcs);

    let parser = FclParser { mgt: &mgt };
    let runner = Runner::new(&mgt);

    let ast = parser.ast("add(1,2)");
}