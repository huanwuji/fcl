use core::fmt;
use std::any::Any;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use crate::ast::AstNode;

#[derive(Hash, Eq, PartialEq)]
pub enum Args<'a> {
    ATypes(Vec<&'a str>),
    CTypes(Vec<Args<'a>>),
}

impl<'a> Args<'a> {
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

impl<'a> Args<'a> {
    pub fn new_s(a_types: Vec<&str>) -> Args {
        Args::ATypes(a_types)
    }
    pub fn new(a_types: Vec<Vec<&str>>) -> Args {
        match a_types.as_slice() {
            [arg1] => Args::ATypes(arg1.to_owned()),
            _ => {
                let types = a_types.into_iter()
                    .map(|args1| Args::ATypes(args1))
                    .collect::<Vec<Args>>();
                Args::CTypes(types)
            }
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct FuncDef<'a> {
    pub name: &'a str,
    pub args: Args<'a>,
    pub r_type: &'a str,
    pub fid: u64,
}

impl<'a> FuncDef<'a> {
    pub fn new(name: &'a str, a_types: &'a Vec<Vec<&str>>, r_type: &'a str) -> FuncDef<'a> {
        let args = Args::new(a_types.to_owned());
        let fid = FuncDef::func_id(name, &args);
        FuncDef { name, args, r_type, fid }
    }

    pub fn func_id(name: &str, args: &Args) -> u64 {
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        args.hash(&mut hasher);
        hasher.finish()
    }
}

impl fmt::Debug for FuncDef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{} -> {}", self.name, Args::display(&self.args), self.r_type)
    }
}

pub struct Context {}

pub trait FuncA {
    fn eval(&self, ctx: Context, nodes: Vec<AstNode>, curr: Option<&Any>) -> Box<Any>;
    fn apply(&self, _func_def: &FuncDef, args: &[&Any]) -> Box<Any> {
        self.apply1(_func_def, args, None)
    }
    fn apply1(&self, _func_def: &FuncDef, args: &[&Any], curr: Option<&Any>) -> Box<Any>;
}

pub struct Funcs<'a> {
    funcs: HashMap<u64, FuncEntity<'a>>
}

pub struct FuncEntity<'a> {
    pub func_def: FuncDef<'a>,
    pub func: Box<FuncA>,
}

impl<'a> Funcs<'a> {
    pub fn new() -> Funcs<'a> {
        Funcs { funcs: HashMap::new() }
    }

    pub fn add<F: FuncA>(&mut self, func_def: FuncDef, func: F) {
        let entity = FuncEntity { func_def, func: Box::new(func) };
        self.funcs.insert(func_def.fid, entity);
    }

    pub fn get_by_type<'c>(&self, name: &'c str, args: &'c Args<'c>) -> &FuncEntity<'a> {
        let func_id = FuncDef::func_id(name, args);
        self.funcs.get(&func_id)
            .expect(format!("Not found func {}{:?}", name, args).as_str())
    }

    pub fn get_entity(&self, func_def: &'a FuncDef) -> &FuncEntity<'a> {
        self.funcs.get(&func_def.fid)
            .expect(&format!("Not found func {:?}", func_def))
    }
}