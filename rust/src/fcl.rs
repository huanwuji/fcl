use std::rc::Rc;

use crate::ast::{AnyVal, AstNode};
use crate::eval::Eval;
use crate::func::Context;
use crate::func_mgt::FuncMgt;
use crate::funcs::calc::add::Add;
use crate::parser::FclParser;

pub struct Fcl {
    mgt: Rc<FuncMgt>,
    parser: FclParser,
    eval: Eval,
}

impl Fcl {
    pub fn new() -> Fcl {
        let mut mgt = FuncMgt::new();
        Fcl::funcs_init(&mut mgt);
        let mgt = Rc::new(mgt);
        let parser = FclParser { mgt: Rc::clone(&mgt) };
        let eval = Eval { mgt: Rc::clone(&mgt) };
        Fcl { mgt, parser, eval }
    }

    pub fn funcs_init(mgt: &mut FuncMgt) {
        Add::register(mgt);
    }

    pub fn ast(&self, str: &str) -> AstNode {
        self.parser.ast(str)
    }

    pub fn eval(&self, ast: &AstNode, curr: &AnyVal) -> AnyVal {
        self.eval_ctx(&self.new_context(), ast, curr)
    }

    pub fn eval_str(&self, str: &str) -> AnyVal {
        let ast = self.parser.ast(str);
        self.eval(&ast, &AnyVal::None)
    }
    pub fn eval_ctx(&self, ctx: &Context, ast: &AstNode, curr: &AnyVal) -> AnyVal {
        self.eval.eval(ctx, ast, curr)
    }

    pub fn new_context(&self) -> Context {
        Context {
            scope: Default::default(),
            mgt: &self.mgt,
            parser: &self.parser,
            eval: &self.eval,
        }
    }
}