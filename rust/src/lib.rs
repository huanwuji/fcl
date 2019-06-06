//extern crate pest;
//#[macro_use]
//extern crate nom;
//extern crate pest_derive;
#![feature(core_intrinsics)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
pub mod macros;

pub mod ast;
pub mod parser;
pub mod runner;
pub mod func;
pub mod types;
pub mod funcs;
pub mod func_reg;