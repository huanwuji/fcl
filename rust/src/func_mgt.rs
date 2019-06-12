use std::collections::HashMap;

use crate::func::{Args, FuncDef, FuncDesc};

pub struct FuncMgt<'a> {
    funcs: HashMap<u64, FuncDef<'a>>
}

impl<'a> FuncMgt<'a> {
    pub fn new() -> FuncMgt<'a> {
        FuncMgt { funcs: HashMap::new() }
    }
    pub fn registers(&'a mut self, func_entities: Vec<FuncDef<'a>>) {
        for entity in func_entities {
            self.add(entity);
        }
    }

    pub fn add(&mut self, def: FuncDef<'a>) {
        self.funcs.insert(def.desc.fid, def);
    }

    pub fn get_by_type(&self, name: &'a str, args: Args<'a>) -> &FuncDef<'a> {
        let func_id = FuncDesc::func_id(name, &args);
        self.funcs.get(&func_id)
            .expect(format!("Not found func {}{:?}", name, args).as_str())
    }

    pub fn get_entity(&self, func_def: &'a FuncDesc) -> &FuncDef<'a> {
        self.funcs.get(&func_def.fid)
            .expect(&format!("Not found func {:?}", func_def))
    }
}

#[test]
fn run() {}