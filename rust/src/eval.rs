use std::rc::Rc;

use crate::ast::{AnyVal, AstNode};
use crate::func::{Context, FuncDef};
use crate::func_mgt::FuncMgt;

pub struct Eval {
    pub mgt: Rc<FuncMgt>
}

impl Eval {
    pub fn eval_vec(&self, ctx: &Context, ast: &Vec<AstNode>,
                    curr: &AnyVal) -> Vec<AnyVal> {
        ast.iter()
            .map(|node| ctx.eval.eval(ctx, node, curr))
            .collect()
    }

    pub fn eval(&self, ctx: &Context, ast: &AstNode, curr: &AnyVal) -> AnyVal {
        match ast {
            AstNode::Curr => curr.clone(),
            AstNode::Val(ref any_val) => any_val.clone(),
            AstNode::Var { name } => {
                ctx.scope.get(name.as_str()).map(|v| v.clone())
                    .unwrap_or(AnyVal::None)
            }
            AstNode::Func { args, func_def, .. } =>
                self.eval_func(ctx, args, func_def, curr),
            AstNode::CurryingFunc { args, func_def, .. } =>
                self.eval_currying_func(ctx, args, func_def, curr),
            AstNode::FlowFunc { exprs } => self.eval_flow_func(ctx, exprs, curr),
            AstNode::Exprs(exprs) => self.eval_exprs(ctx, exprs, curr),
            AstNode::FuncEnd => AnyVal::None,
            _ => panic!()
        }
    }

    fn eval_func(&self, ctx: &Context, args: &Vec<AstNode>,
                 func_def: &FuncDef, curr: &AnyVal) -> AnyVal {
        func_def.func.eval(ctx, func_def, args, curr)
    }

    fn eval_currying_func(&self, ctx: &Context, args: &Vec<Vec<AstNode>>,
                          func_def: &FuncDef, curr: &AnyVal) -> AnyVal {
        func_def.func.eval_currying(ctx, func_def, args, curr)
    }

    fn eval_flow_func(&self, ctx: &Context,
                      exprs: &Vec<AstNode>, curr: &AnyVal) -> AnyVal {
        let mut result = curr.clone();
        for node in exprs {
            result = self.eval(ctx, node, &result);
        }
        result
    }

    fn eval_exprs(&self, ctx: &Context,
                  exprs: &Vec<AstNode>, curr: &AnyVal) -> AnyVal {
        let mut result = AnyVal::None;
        for node in exprs {
            result = self.eval(ctx, node, &result);
        }
        result
    }
}