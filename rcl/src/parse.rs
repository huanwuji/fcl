//use nom::error::{ErrorKind, ParseError};
//use nom::IResult;
//use nom::number::complete::recognize_float;
//
//#[derive(Debug)]
//enum AnyVal {
//    Str(String),
//    Bool(bool),
//    Long(i64),
//    Float(f64),
//}
//
//#[derive(Debug)]
//enum Expr {
//    Val(AnyVal),
//    Var { name: String },
//    Func { args: Vec<Expr> },
//    FlowFunc { exprs: Vec<Expr> },
//    CurryingFunc { exprs: Vec<Vec<Expr>> },
//}
//
//named!(boolean<bool>, alt!(
//    tag!("true") => {|_|true} |
//    tag!("false") => {|_|false})
//);
//
//named!(float<f64>, flat_map!(recognize_float, parse_to!(f64)));
//
////named!(long<&str, i64>, map_res!(recognize_float, |s:&str|s.parse::<i64>()));
//
//named!(value<AnyVal>, alt!(
//    boolean => { |b| AnyVal::Bool(b) } |
//    float => { |f| AnyVal::Float(f) }
////    long => { |l| AnyVal::Long(l) }
//));
//
//#[test]
//fn main() {
////    eprintln!("haha = {:?}", value("11".as_bytes()));
//    let l = value("true".as_bytes());
////    let l = parser("123.45");
//    eprintln!("haha = {:?}", l);
//}
