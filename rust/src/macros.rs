use crate::func::FuncDesc;

#[macro_export]
macro_rules! def {
     ( $name:tt$(($($arg:ty),* ))* -> $out:ty ) => {{
        let a_types = vec![$(vec![$(String::from(stringify!($arg))),*]),*];
        let args = $crate::func::Args::new(a_types);
        let name = stringify!($name);
        let fid = $crate::func::FuncDesc::func_id(name, &args);
        let r_type = stringify!($out);
        FuncDesc { name: String::from(name), args, r_type: String::from(r_type), fid }
     }};
}

#[test]
fn type_test() {
    let def_str = def!(haha(i32, i64)(i64,u33) -> Vec);
    println!("{:?}", def_str);
}

#[test]
fn test_derive() {
    let def1: FuncDesc = def!(dd(i32, i64) -> haha);
    let def2: FuncDesc = def!(dd(i32, i64) -> haha);
    eprintln!("{}", def1 == def2);
}