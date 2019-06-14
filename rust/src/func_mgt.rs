use std::collections::HashMap;

use crate::func::{Args, FuncDef, FuncDesc};

pub struct FuncMgt<'a> {
    funcs: HashMap<u64, FuncDef<'a>>
}

impl<'a> FuncMgt<'a> {
    pub fn new() -> FuncMgt<'static> {
        FuncMgt { funcs: HashMap::new() }
    }

    pub fn registers(&mut self, func_entities: Vec<FuncDef<'a>>) {
        for entity in func_entities {
            self.add(entity);
        }
    }

    pub fn add(&mut self, def: FuncDef<'a>) {
        self.funcs.insert(def.desc.fid, def);
    }

    pub fn get_by_type<'b>(&self, name: &'b str, args: Args<'b>) -> &FuncDef<'a> {
        let func_id = FuncDesc::func_id(name, &args);
        self.funcs.get(&func_id)
            .expect(format!("Not found func {}{:?}", name, args).as_str())
    }
}

#[test]
fn run() {}