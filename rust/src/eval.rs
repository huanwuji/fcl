use crate::ast::{AnyVal, AstNode};
use crate::func::{Context, FuncDef};
use crate::func_mgt::FuncMgt;

pub struct Eval<'a> {
    mgt: &'a FuncMgt<'a>
}

impl<'a> Eval<'a> {
    pub fn new(mgt: &'a FuncMgt<'a>) -> Eval {
        Eval { mgt }
    }

    pub fn eval_vec(&self, ctx: &'a Context<'a>, ast: &'a Vec<AstNode<'a>>,
                    curr: &'a AnyVal<'a>) -> Vec<AnyVal<'a>> {
        ast.iter()
            .map(|node| ctx.eval.eval(ctx, node, curr))
            .collect()
    }

    pub fn eval(&self, ctx: &'a Context<'a>, ast: &'a AstNode<'a>, curr: &'a AnyVal<'a>) -> AnyVal<'a> {
        match ast {
            AstNode::Val(any_val) => *any_val,
//            AstNode::Var { name } => {
//                ctx.scope.get(name).map(|v| AnyVal::Any {
//                    val: v,
//                    any_type: ValType::ANY,
//                }).unwrap_or(AnyVal::None)
//            }
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

    fn eval_func(&self, ctx: &'a Context<'a>, args: &'a Vec<AstNode<'a>>,
                 func_def: &'a FuncDef<'a>, curr: &'a AnyVal<'a>) -> AnyVal<'a> {
        func_def.func.eval(ctx, func_def, args, curr)
    }

    fn eval_currying_func(&self, ctx: &'a Context<'a>, args: &'a Vec<Vec<AstNode<'a>>>,
                          func_def: &'a FuncDef<'a>, curr: &'a AnyVal<'a>) -> AnyVal<'a> {
        func_def.func.eval_currying(ctx, func_def, args, curr)
    }

    fn eval_flow_func(&self, ctx: &'a Context<'a>,
                      exprs: &'a Vec<AstNode<'a>>, curr: &'a AnyVal<'a>) -> AnyVal<'a> {
//        let mut curr = curr;
        let mut result = AnyVal::None;
//        for node in exprs {
//            result = match node {
//                AstNode::FuncEnd => AnyVal::None,
//                _ => self.eval(ctx, node, &result),
//            };
////            curr = &result;
//        }
        result
    }

    fn eval_exprs(&self, ctx: &'a Context<'a>,
                  exprs: &'a Vec<AstNode<'a>>, curr: &'a AnyVal<'a>) -> AnyVal<'a> {
        let mut result = AnyVal::None;
        for node in exprs {
            result = self.eval(ctx, node, curr);
        }
        result
    }
}