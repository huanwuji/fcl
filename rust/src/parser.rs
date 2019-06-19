use std::rc::Rc;

use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::*;

use crate::ast::{AnyVal, AstNode};
use crate::func::Args;
use crate::func::FuncDef;
use crate::func_mgt::FuncMgt;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct FclParser { pub mgt: Rc<FuncMgt> }

impl FclParser {
    pub fn ast(&self, str: &str) -> AstNode {
        let pairs: Pairs<Rule> = FclParser::parse(Rule::functions, str)
            .unwrap_or_else(|e| panic!("{}", e));
        self.build_functions(pairs)
    }

    fn build_functions(&self, pairs: Pairs<Rule>) -> AstNode {
        let mut exprs = vec![];
        for pair in pairs {
            match pair.as_rule() {
                Rule::function => {
                    for func_pair in pair.into_inner() {
                        exprs.push(self.build_function(func_pair, None));
                    }
                }
                Rule::func_end => exprs.push(AstNode::FuncEnd),
                Rule::EOI => {}
                _ => unreachable!()
            }
        }
        AstNode::Exprs(exprs)
    }

    fn build_function(&self, pair: Pair<Rule>, curr_type: Option<&str>) -> AstNode {
        match pair.as_rule() {
            Rule::flow_func => self.build_flow_func(pair),
            Rule::currying_func => self.build_currying_func(pair, curr_type),
            Rule::normal_func => self.build_normal_func(pair, curr_type),
            _ => unreachable!()
        }
    }

    fn build_flow_func(&self, pair: Pair<Rule>) -> AstNode {
        let mut functions = vec![];
        let mut pre_r_type = None;
        for inner_pair in pair.into_inner() {
            let node = self.build_function(inner_pair, pre_r_type);
            pre_r_type = Some(AstNode::get_r_type_raw(&node, pre_r_type).as_str());
            functions.push(node);
        }
        AstNode::FlowFunc { exprs: functions }
    }

    fn build_normal_func(&self, pair: Pair<Rule>, curr_type: Option<&str>) -> AstNode {
        let mut pairs = pair.into_inner();
        let func_name = pairs.next().unwrap().as_str();
        let args_pair = pairs.next().unwrap();
        let arguments = self.build_arguments(args_pair, curr_type);
        let func_def = self.get_def(func_name, &arguments, curr_type);
        AstNode::Func {
            name: String::from(func_name),
            args: arguments,
            func_def,
        }
    }

    fn build_currying_func(&self, pair: Pair<Rule>, curr_type: Option<&str>) -> AstNode {
        let mut func_name = None;
        let mut args_vec = vec![];
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::func_name => func_name = Some(inner_pair.as_str()),
                Rule::arguments => args_vec.push(self.build_arguments(inner_pair, curr_type)),
                _ => unreachable!()
            }
        }
        let def = self.get_currying_def(func_name.unwrap(), &args_vec, curr_type);
        AstNode::CurryingFunc {
            name: String::from(func_name.unwrap()),
            args: args_vec,
            func_def: def,
        }
    }

    fn get_def<'b>(&self, name: &'b str, args: &'b Vec<AstNode>, curr_type: Option<&str>) -> &FuncDef {
        let a_types = args.iter()
            .map(|node| String::from(AstNode::get_r_type_raw(node, curr_type)))
            .collect::<Vec<String>>();
        let args = Args::new(vec![a_types]);
        self.mgt.get_by_type(name, args)
    }

    fn get_currying_def<'b>(&self, name: &'b str, args: &'b Vec<Vec<AstNode>>, curr_type: Option<&str>) -> &FuncDef {
        let a_types = args.iter()
            .map(|types| types.iter()
                .map(|node| String::from(AstNode::get_r_type_raw(node, curr_type)))
                .collect::<Vec<String>>()
            )
            .collect::<Vec<Vec<String>>>();
        let args = Args::new(a_types);
        self.mgt.get_by_type(name, args)
    }

    fn build_arguments(&self, pair: Pair<Rule>, curr_type: Option<&str>) -> Vec<AstNode> {
        let mut args = vec![];
        for arg_pair in pair.into_inner() {
            args.push(self.build_argument(arg_pair, curr_type));
        }
        args
    }

    fn build_argument(&self, pair: Pair<Rule>, curr_type: Option<&str>) -> AstNode {
        let arg_pair = pair.into_inner().next().unwrap();
        match arg_pair.as_rule() {
            Rule::curr => AstNode::Curr,
            Rule::function => self.build_function(arg_pair.into_inner().next().unwrap(), curr_type),
            Rule::value => {
                let any_val = self.build_value(arg_pair.into_inner().next().unwrap());
                AstNode::Val(any_val)
            }
            Rule::variable => self.build_variable(arg_pair.into_inner().next().unwrap()),
            _ => panic!("UnMatched rule")
        }
    }

    fn build_variable(&self, pair: Pair<Rule>) -> AstNode {
        AstNode::Var { name: String::from(pair.as_str()) }
    }

    fn build_value(&self, pair: Pair<Rule>) -> AnyVal {
        match pair.as_rule() {
            Rule::string => AnyVal::Str(String::from(pair.as_str())),
            Rule::float => AnyVal::Float(pair.as_str().parse().unwrap()),
            Rule::long => AnyVal::Long(pair.as_str().parse().unwrap()),
            Rule::boolean => AnyVal::Bool(pair.as_str().parse().unwrap()),
            _ => unreachable!()
        }
    }
}