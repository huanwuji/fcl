use crate::func::FuncDef;

#[macro_export]
macro_rules! def {
     ( $name:ident($($arg:ty),* ) -> $out:ty ) => {{
          $crate::func::FuncDef{
              name: stringify!($name),
              a_types: &[$(stringify!($arg)),*],
              r_type: stringify!($out)
          }
     }};
}

#[test]
fn type_test() {
    let def_str = def!(haha(i32, i64) -> Vec);
    println!("{:?}", def_str);
}

#[test]
fn test_derive() {
    let def1: FuncDef = def!(dd(i32, i64) -> haha);
    let def2: FuncDef = def!(dd(i32, i64) -> haha);
    eprintln!("{}", def1 == def2);
}