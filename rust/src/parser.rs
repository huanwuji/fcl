use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::*;

use crate::ast::{AnyVal, AstNode};
use crate::func::FuncDef;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct FclParser { a: i32 }

impl FclParser {
    pub fn ast(&self, str: &str) -> AstNode {
        let pairs: Pairs<Rule> = FclParser::parse(Rule::functions, str).unwrap_or_else(|e| panic!("{}", e));
        self.build_functions(pairs)
    }

    fn build_functions(&self, pairs: Pairs<Rule>) -> AstNode {
        let mut exprs = vec![];
        for pair in pairs {
            match pair.as_rule() {
                Rule::function => {
                    for func_pair in pair.into_inner() {
                        exprs.push(self.build_function(func_pair));
                    }
                }
                Rule::func_end => exprs.push(AstNode::FuncEnd),
                Rule::EOI => {}
                _ => unreachable!()
            }
        }
        AstNode::Exprs(exprs)
    }

    fn build_function(&self, pair: Pair<Rule>) -> AstNode {
        match pair.as_rule() {
            Rule::flow_func => self.build_flow_func(pair),
            Rule::currying_func => self.build_currying_func(pair),
            Rule::normal_func => self.build_normal_func(pair),
            _ => unreachable!()
        }
    }

    fn build_flow_func(&self, pair: Pair<Rule>) -> AstNode {
        let mut functions = vec![];
        for inner_pair in pair.into_inner() {
            functions.push(self.build_function(inner_pair))
        }
        AstNode::FlowFunc { exprs: functions }
    }

    fn build_currying_func(&self, pair: Pair<Rule>) -> AstNode {
        let mut func_name = None;
        let mut args_vec = vec![];
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::func_name => func_name = Some(inner_pair.as_str()),
                Rule::arguments => args_vec.push(self.build_arguments(inner_pair)),
                _ => unreachable!()
            }
        }
        AstNode::CurryingFunc { name: func_name.unwrap(), args: args_vec }
    }

    fn build_normal_func(&self, pair: Pair<Rule>) -> AstNode {
        let mut pairs = pair.into_inner();
        let func_name = pairs.next().unwrap();
        let args_pair = pairs.next().unwrap();
        let arguments = self.build_arguments(args_pair);
        AstNode::Func { name: func_name.as_str(), args: arguments }
    }

    fn build_arguments(&self, pair: Pair<Rule>) -> Vec<AstNode> {
        let mut args = vec![];
        for arg_pair in pair.into_inner() {
            args.push(self.build_argument(arg_pair));
        }
        args
    }

    fn build_argument(&self, pair: Pair<Rule>) -> AstNode {
        let arg_pair = pair.into_inner().next().unwrap();
        match arg_pair.as_rule() {
            Rule::function => self.build_function(arg_pair),
            Rule::value => {
                let any_val = self.build_value(arg_pair.into_inner().next().unwrap());
                AstNode::Val(any_val)
            }
            Rule::variable => self.build_variable(arg_pair),
            _ => AstNode::Empty
        }
    }

    fn build_variable(&self, pair: Pair<Rule>) -> AstNode {
        AstNode::Var { name: pair.as_str() }
    }

    fn build_value(&self, pair: Pair<Rule>) -> AnyVal {
        match pair.as_rule() {
            Rule::string => AnyVal::Str(pair.as_str()),
            Rule::float => AnyVal::Float(pair.as_str().parse().unwrap()),
            Rule::long => AnyVal::Long(pair.as_str().parse().unwrap()),
            Rule::boolean => AnyVal::Bool(pair.as_str().parse().unwrap()),
            _ => unreachable!()
        }
    }
}

#[test]
fn main() {
    let exprs = "f1(1,abc)(dd,11).dd().abc(1,2);";
//    let exprs = "f1(1,2)(4,3,dd).ab().bd(cd(1,2));";
    let parser = FclParser { a: 32 };
    let ast = parser.parse(exprs);
    eprintln!("ast = {:?}", ast);

//    assert_eq!(1, 3);
}