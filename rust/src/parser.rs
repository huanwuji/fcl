use pest::{Parser, RuleType};
use pest::iterators::{Pair, Pairs};
use pest_derive::*;

use crate::ast::{AnyVal, AstNode};

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct FclParser;

pub fn parse(str: &str) -> AstNode {
    let pairs: Pairs<Rule> = FclParser::parse(Rule::functions, str).unwrap_or_else(|e| panic!("{}", e));
    build_functions(pairs)
}

fn build_functions(pairs: Pairs<Rule>) -> AstNode {
    let mut exprs = vec![];
    for pair in pairs {
        match pair.as_rule() {
            Rule::function => {
                for func_pair in pair.into_inner() {
                    exprs.push(build_function(func_pair));
                }
            }
            Rule::func_end => exprs.push(AstNode::FuncEnd),
            Rule::EOI => {}
            _ => unreachable!()
        }
    }
    AstNode::Exprs(exprs)
}

fn build_function(pair: Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::flow_func => build_flow_func(pair),
        Rule::currying_func => build_currying_func(pair),
        Rule::normal_func => build_normal_func(pair),
        _ => unreachable!()
    }
}

fn build_flow_func(pair: Pair<Rule>) -> AstNode {
    let mut functions = vec![];
    for inner_pair in pair.into_inner() {
        functions.push(build_function(inner_pair))
    }
    AstNode::FlowFunc { exprs: functions }
}

fn build_currying_func(pair: Pair<Rule>) -> AstNode {
    let mut func_name = None;
    let mut args_vec = vec![];
    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::func_name => func_name = Some(inner_pair.as_str()),
            Rule::arguments => args_vec.push(build_arguments(inner_pair)),
            _ => unreachable!()
        }
    }
    AstNode::CurryingFunc { name: func_name.unwrap(), args: args_vec }
}

fn build_normal_func(pair: Pair<Rule>) -> AstNode {
    let mut pairs = pair.into_inner();
    let func_name = pairs.next().unwrap();
    let args_pair = pairs.next().unwrap();
    let arguments = build_arguments(args_pair);
    AstNode::Func { name: func_name.as_str(), args: arguments }
}

fn build_arguments(pair: Pair<Rule>) -> Vec<AstNode> {
    let mut args = vec![];
    for arg_pair in pair.into_inner() {
        args.push(build_argument(arg_pair));
    }
    args
}

fn build_argument(pair: Pair<Rule>) -> AstNode {
    let arg_pair = pair.into_inner().next().unwrap();
    match arg_pair.as_rule() {
        Rule::function => build_function(arg_pair),
        Rule::value => {
            let any_val = build_value(arg_pair.into_inner().next().unwrap());
            AstNode::Val(any_val)
        }
        Rule::variable => build_variable(arg_pair),
        _ => AstNode::Empty
    }
}

fn build_variable(pair: Pair<Rule>) -> AstNode {
    AstNode::Var { name: pair.as_str() }
}

fn build_value(pair: Pair<Rule>) -> AnyVal {
    match pair.as_rule() {
        Rule::string => AnyVal::Str(pair.as_str()),
        Rule::float => AnyVal::Float(pair.as_str().parse().unwrap()),
        Rule::long => AnyVal::Long(pair.as_str().parse().unwrap()),
        Rule::boolean => AnyVal::Bool(pair.as_str().parse().unwrap()),
        Rule::null => AnyVal::Null,
        _ => unreachable!()
    }
}

#[test]
fn main() {
    let exprs = "f1(1,abc)(dd,11).dd().abc(1,2);";
//    let exprs = "f1(1,2)(4,3,dd).ab().bd(cd(1,2));";
    let ast = parse(exprs);
    eprintln!("ast = {:?}", ast);

//    assert_eq!(1, 3);
}