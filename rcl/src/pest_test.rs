use pest::Parser;
use pest_derive::*;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct FclParser;

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

#[test]
fn main() {
    let pairs = FclParser::parse(Rule::functions, "f1(1,2)(4,3).ab().bd(cd(1,2));").unwrap_or_else(|e| panic!("{}", e));

    // Because ident_list is silent, the iterator will contain idents
    for pair in pairs {
        // A pair is a combination of the rule which matched and a span of input
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.as_span());
        println!("Text:    {}", pair.as_str());
        println!("Inner:    {}", pair.into_inner());

        // A pair can be converted to an iterator of the tokens which make it up:
//        for inner_pair in pair.into_inner() {
//            match inner_pair.as_rule() {
//                Rule::alpha => println!("Letter:  {}", inner_pair.as_str()),
//                Rule::digit => println!("Digit:   {}", inner_pair.as_str()),
//                _ => unreachable!()
//            };
//        }
    }
//    assert_eq!(1, 3);
}