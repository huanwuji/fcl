use std::collections::HashMap;

use crate::func::{Args, FuncDef, FuncDesc};

pub struct FuncMgt {
    funcs: HashMap<u64, FuncDef>
}

impl FuncMgt {
    pub fn new() -> FuncMgt {
        FuncMgt { funcs: HashMap::new() }
    }

    pub fn registers(&mut self, func_entities: Vec<FuncDef>) {
        for entity in func_entities {
            self.add(entity);
        }
    }

    pub fn add(&mut self, def: FuncDef) {
        self.funcs.insert(def.desc.fid, def);
    }

    pub fn get_by_type(&self, name: &str, args: Args) -> &FuncDef {
        let func_id = FuncDesc::func_id(name, &args);
        self.funcs.get(&func_id)
            .expect(format!("Not found func {}{:?}", name, args).as_str())
    }
}

#[test]
fn run() {}