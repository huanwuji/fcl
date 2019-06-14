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
pub enum Args<'a> {
    ATypes(Vec<&'a str>),
    CTypes(Vec<Args<'a>>),
}

impl<'a> Args<'a> {
    pub fn new_s(a_types: Vec<&str>) -> Args {
        Args::ATypes(a_types)
    }
    pub fn new(a_types: Vec<Vec<&str>>) -> Args {
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
    fn display(args: &'a Args) -> String {
        match args {
            Args::ATypes(types) => format!("({})", types.join(",")),
            Args::CTypes(types_arr) => types_arr.iter()
                .map(|types| Args::display(types))
                .collect::<Vec<String>>().join("")
        }
    }
}

impl<'a> fmt::Debug for Args<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Args::display(self))
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct FuncDesc<'a> {
    pub name: &'a str,
    pub args: Args<'a>,
    pub r_type: &'a str,
    pub fid: u64,
}

impl<'a> FuncDesc<'a> {
    pub fn new(name: &'a str, a_types: &'a Vec<Vec<&str>>, r_type: &'a str) -> FuncDesc<'a> {
        let args = Args::new(a_types.clone());
        let fid = FuncDesc::func_id(name, &args);
        FuncDesc { name, args, r_type, fid }
    }

    pub fn func_id(name: &str, args: &Args) -> u64 {
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        args.hash(&mut hasher);
        hasher.finish()
    }
}

impl fmt::Debug for FuncDesc<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{} -> {}", self.name, Args::display(&self.args), self.r_type)
    }
}

pub struct FuncDef<'a> {
    pub desc: FuncDesc<'a>,
    pub func: Box<FuncA<'a>>,
}

impl<'a> fmt::Debug for FuncDef<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}: {:?}", stringify!(self.func), self.desc)
    }
}

pub struct Context<'a> {
    pub scope: HashMap<&'a str, AnyVal<'a>>,
    pub mgt: &'a FuncMgt<'a>,
    pub parser: &'a FclParser<'a>,
    pub eval: &'a Eval<'a>,
}

pub trait FuncA<'a> {
    fn eval(&self, ctx: &'a Context<'a>, func_def: &'a FuncDef<'a>,
            nodes: &'a Vec<AstNode<'a>>, curr: &'a AnyVal<'a>) -> AnyVal<'a> {
        let any_vals = ctx.eval.eval_vec(ctx, nodes, curr);
        self.apply1(ctx, func_def, any_vals, curr)
    }
    fn eval_currying(&self, ctx: &'a Context<'a>, func_def: &'a FuncDef<'a>,
                     nodes: &'a Vec<Vec<AstNode<'a>>>, curr: &'a AnyVal<'a>) -> AnyVal<'a> {
        unimplemented!()
    }
    fn apply(&self, ctx: &'a Context<'a>, func_def: &'a FuncDef<'a>,
             args: Vec<AnyVal<'a>>) -> AnyVal<'a> {
        self.apply1(ctx, func_def, args, &AnyVal::None)
    }
    fn apply1(&self, ctx: &'a Context<'a>, func_def: &'a FuncDef<'a>,
              args: Vec<AnyVal<'a>>, curr: &'a AnyVal<'a>) -> AnyVal<'a>;
}

pub trait Def<'a> {
    fn def(&self) -> FuncDef<'a>;
}