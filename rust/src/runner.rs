use std::any::Any;

use crate::ast::{AnyVal, AstNode};
use crate::func_mgt::FuncMgt;
use crate::types;

pub struct Runner<'a: 'static> {
    mgt: &'a FuncMgt<'a>
}

impl<'a: 'static> Runner<'a> {
    pub fn new(mgt: &'a FuncMgt<'a>) -> Runner {
        Runner { mgt }
    }

//    fn eval(&self, ast: &'a AstNode<'a>) -> Box<Any> {
//        match ast {
//            AstNode::Val(v) => v,
//            AstNode::Var { .. } => {}
//            AstNode::Func { .. } => {}
//            AstNode::CurryingFunc { .. } => {}
//            AstNode::FlowFunc { .. } => {}
//            AstNode::Exprs(_) => {}
//            AstNode::VOID => {}
//            AstNode::FuncEnd => {}
//        }
//    }

    fn eval_val(val: &AnyVal) -> &dyn Any {
        let any = match val {
            AnyVal::Bool(b) => types::any(b),
            AnyVal::Long(l) => types::any(l),
            AnyVal::Float(f) => types::any(f),
            AnyVal::Str(s) => types::any(s),
        };
        any
    }
}