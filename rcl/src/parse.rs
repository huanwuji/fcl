use nom::number::complete::recognize_float;

#[derive(Debug)]
enum AnyVal {
    Str(String),
    Bool(bool),
    Long(i64),
    Float(f64),
}

#[derive(Debug)]
enum Expr {
    Val(AnyVal),
    Var { name: String },
    Func { args: Vec<Expr> },
    FlowFunc { exprs: Vec<Expr> },
    CurryingFunc { exprs: Vec<Vec<Expr>> },
}

named!(boolean<bool>, alt!(
    tag!("true") => {|_|true} |
    tag!("false") => {|_|false})
);

named!(float<f64>, flat_map!(recognize_float, parse_to!(f64)));

named!(long<i64>, flat_map!(recognize_float, parse_to!(i64)));

named!(val_expr<AnyVal>, alt!(
    boolean => { |b| AnyVal::Bool(b) } |
//    float => { |f| AnyVal::Float(f) }
    long => { |l| AnyVal::Long(l) }
));
named!(parser<&str, f64>, flat_map!(recognize_float, parse_to!(f64)));
#[test]
fn main() {
//    eprintln!("haha = {:?}", val_expr("11".as_bytes()));
    let l = long("111111111;".as_bytes());
//    let l = parser("123.45");
    eprintln!("haha = {:?}", l);
}
