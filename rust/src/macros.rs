use crate::func::FuncDef;

#[macro_export]
macro_rules! def {
     ( $name:ident$(($($arg:ty),* ))* -> $out:ty ) => {{
        let a_types = vec![$(vec![$(stringify!($arg)),*]),*];
        let args = $crate::func::Args::new(a_types);
        let name = stringify!($name);
        let fid = $crate::func::FuncDef::func_id(name, &args);
        let r_type = stringify!($out);
        FuncDef { name, args, r_type, fid }
     }};
}

#[test]
fn type_test() {
    let def_str = def!(haha(i32, i64)(i64,u33) -> Vec);
    println!("{:?}", def_str);
}

#[test]
fn test_derive() {
    let def1: FuncDef = def!(dd(i32, i64) -> haha);
    let def2: FuncDef = def!(dd(i32, i64) -> haha);
    eprintln!("{}", def1 == def2);
}