use core::fmt;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fmt::Formatter;
use std::hash::{Hash, Hasher};

use crate::ast::{AnyVal, AstNode};
use crate::eval::Eval;
use crate::func_mgt::FuncMgt;
use crate::parser::FclParser;

#[derive(Hash, Eq, PartialEq)]
pub enum Args {
    ATypes(Vec<String>),
    CTypes(Vec<Args>),
}

impl Args {
    pub fn new_s(a_types: Vec<String>) -> Args {
        Args::ATypes(a_types)
    }
    pub fn new(a_types: Vec<Vec<String>>) -> Args {
        match a_types.as_slice() {
            [arg1] => Args::ATypes(arg1.clone()),
            _ => {
                let types = a_types.into_iter()
                    .map(|args1| Args::ATypes(args1))
                    .collect::<Vec<Args>>();
                Args::CTypes(types)
            }
        }
    }
    fn display(args: &Args) -> String {
        match args {
            Args::ATypes(types) => format!("({})", types.join(",")),
            Args::CTypes(types_arr) => types_arr.iter()
                .map(|types| Args::display(types))
                .collect::<Vec<String>>().join("")
        }
    }
}

impl fmt::Debug for Args {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Args::display(self))
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct FuncDesc {
    pub name: String,
    pub args: Args,
    pub r_type: String,
    pub fid: u64,
}

impl FuncDesc {
    pub fn new(name: String, a_types: &Vec<Vec<String>>, r_type: String) -> FuncDesc<> {
        let args = Args::new(a_types.clone());
        let fid = FuncDesc::func_id(name.as_str(), &args);
        FuncDesc { name, args, r_type, fid }
    }

    pub fn func_id(name: &str, args: &Args) -> u64 {
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        args.hash(&mut hasher);
        hasher.finish()
    }
}

impl fmt::Debug for FuncDesc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{} -> {}", self.name, Args::display(&self.args), self.r_type)
    }
}

pub struct FuncDef {
    pub desc: FuncDesc,
    pub func: Box<FuncA>,
}

impl fmt::Debug for FuncDef {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}: {:?}", stringify!(self.func), self.desc)
    }
}

pub struct Context<'r> {
    pub scope: HashMap<&'static str, AnyVal>,
    pub mgt: &'r FuncMgt,
    pub parser: &'r FclParser<'r>,
    pub eval: &'r Eval<'r>,
}

pub trait FuncA {
    fn eval(&self, ctx: &Context, func_def: &FuncDef,
            nodes: &Vec<AstNode>, curr: &AnyVal) -> AnyVal {
        let any_vals = ctx.eval.eval_vec(ctx, nodes, curr);
        self.apply1(ctx, func_def, any_vals, curr)
    }
    fn eval_currying(&self, ctx: &Context, func_def: &FuncDef,
                     nodes: &Vec<Vec<AstNode>>, curr: &AnyVal) -> AnyVal {
        unimplemented!()
    }
    fn apply(&self, ctx: &Context, func_def: &FuncDef,
             args: Vec<AnyVal>) -> AnyVal {
        self.apply1(ctx, func_def, args, &AnyVal::None)
    }
    fn apply1(&self, ctx: &Context, func_def: &FuncDef,
              args: Vec<AnyVal>, curr: &AnyVal) -> AnyVal;
}