use core::fmt;
use std::any::Any;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::intrinsics;
use std::ptr::hash;

use itertools::Itertools;

use crate::ast::AstNode;

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum Args<'a> {
    ATypes(&'a [&'a str]),
    CTypes(&'a [Args<'a>]),
}

impl<'a> Args<'a> {
    fn display(args: &'a Args) -> String {
        match args {
            Args::ATypes(types) => fmt!("({})",types.join(",")),
            Args::CTypes(types_arr) => types_arr.iter()
                .map(|types| Args::display(types))
                .join(",")
        }
    }
}

impl<'a> fmt::Display for Args<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!("{}", String::from(Args::display(self)))
    }
}

impl<'a> Args<'a> {
    pub fn new_s(a_types: &'a [&'a str]) -> Args<'a> {
        Args::ATypes(a_types)
    }
    pub fn new(a_types: &'a [&'a [&'a str]]) -> Args<'a> {
        match a_types {
            &[a]  if a_types.len() > 0 => Args::ATypes(a),
            _ => {
                let types = a_types.iter()
                    .map(|args1| Args::ATypes(args1)).collect::<&'a [Args]>();
                Args::CTypes(types)
            }
        };
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct FuncDef<'a> {
    pub name: &'a str,
    pub args: &'a Args<'a>,
    pub r_type: &'a str,
    pub func_id: u64,
}

impl<'a> FuncDef<'a> {
    pub fn new(name: &'a str, a_types: &'a [&'a [&'a str]], r_type: &'a str) -> FuncDef<'a> {
        let args = Args::new(a_types);
        let func_id = FuncDef::func_id(name, &args);
        FuncDef {
            name,
            args: &args,
            r_type,
            func_id,
        }
    }

    fn func_id(name: &str, args: &'a Args<'a>) -> u64 {
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        args.hash(&mut hasher);
        hasher.finish()
    }
}

impl<'a> fmt::Display for FuncDef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}({}) -> {}", self.name, Args::display(self.args), self.r_type)
    }
}

pub struct Context {}

pub trait FuncA {
    fn eval(ctx: Context, nodes: Vec<AstNode>, curr: Option<&Any>) -> Box<Any>;
    fn apply(&self, _func_def: FuncDef<'_>, args: &[&Any]) -> Box<Any> {
        self.apply1(_func_def, args, None)
    }
    fn apply1(&self, _func_def: FuncDef<'_>, args: &[&Any], curr: Option<&Any>) -> Box<Any>;
}

pub struct Funcs<'a> {
    funcs: &'a mut HashMap<u64, &'a FuncEntity<'a>>
}

pub struct FuncEntity<'a> {
    pub func_def: &'a FuncDef<'a>,
    pub func: Box<impl FuncA>,
}

impl<'a> Funcs<'a> {
    pub fn new() -> Funcs<'a> {
        Funcs { funcs: &mut HashMap::new() }
    }
    pub fn add<F: FuncA>(&self, func_def: &'a FuncDef, func: F) {
        let entity = FuncEntity { func_def, func };
        self.funcs.insert(func_def.func_id, &entity);
    }

    pub fn get_by_type(&self, name: &'a str, args: &'a Args<'a>) -> &FuncEntity<'a> {
        let func_id = FuncDef::func_id(name, args);
        self.funcs.get(&func_id)
            .expect(format!("Not found func {}{}", name, args).as_str())
    }

    pub fn get_entity(&self, func_def: &'a FuncDef) -> &FuncEntity<'a> {
        self.funcs.get(&func_def.func_id)
            .expect(&format!("Not found func {}"))
    }
}

#[test]
fn type_id() {
    unsafe {
        let type_id = intrinsics::type_id::<i32>();
        eprintln!("type_id = {:?}", type_id);
        let type_name = intrinsics::type_name::<Vec<i32>>();
        eprintln!("type_name = {:?}", type_name);
    }
}