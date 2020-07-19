use std::str::FromStr;

use pest::iterators::Pair;
use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::Parser;

use crate::ast::{Identifier, Statement, Symbol};

#[derive(Parser)]
#[grammar = "parser/grammar.pest"] // relative to src
struct ProgramParser;

fn parse_pair(pair: Pair<'_, Rule>) -> Symbol {
    match pair.as_rule() {
        Rule::ap => Symbol::Ap,
        Rule::var => {
            let value = pair.into_inner().as_str();
            Symbol::Var(usize::from_str(&value).unwrap())
        }
        Rule::cons => Symbol::Cons,
        Rule::car => Symbol::Car,
        Rule::cdr => Symbol::Cdr,
        Rule::number => Symbol::Lit(i64::from_str(pair.as_str()).unwrap()),
        Rule::nil => Symbol::Nil,
        Rule::eq => Symbol::Eq,
        Rule::lt => Symbol::Lt,
        Rule::neg => Symbol::Neg,
        Rule::inc => Symbol::Inc,
        Rule::s => Symbol::S,
        Rule::c => Symbol::C,
        Rule::b => Symbol::B,
        Rule::i => Symbol::I,
        Rule::t => Symbol::T,
        Rule::f => Symbol::F,
        Rule::mul => Symbol::Mul,
        Rule::add => Symbol::Add,
        Rule::div => Symbol::Div,
        Rule::isnil => Symbol::IsNil,
        Rule::modulate => Symbol::Mod,
        Rule::demodulate => Symbol::Dem,
        Rule::if0 => Symbol::If0,
        Rule::list => {
            let inner: Vec<_> = pair.into_inner().map(|pair| parse_pair(pair)).collect();
            if inner.is_empty() {
                Symbol::Nil
            } else {
                Symbol::List(inner)
            }
        }
        _ => unimplemented!("Unhandled Pair {:?}", pair),
    }
}

pub fn parse_as_lines(input: &str) -> Vec<Statement> {
    let lines = input.split('\n');
    let mut statements = Vec::new();

    for line in lines {
        let parsed_line = ProgramParser::parse(Rule::line, line)
            .expect("failed to parse line")
            .next()
            .unwrap();

        let assignment: Pairs<'_, _> = parsed_line.into_inner();
        let assignment = assignment.peek().unwrap().into_inner();

        let id = assignment.peek().unwrap();
        let id = id.into_inner().peek().unwrap();

        let id = match id.as_rule() {
            Rule::var => {
                Identifier::Var(usize::from_str(id.into_inner().peek().unwrap().as_str()).unwrap())
            }
            Rule::identifier => Identifier::Name(id.as_str().to_string()),
            _ => unimplemented!("Invalid variable id {:?}", id),
        };

        let symbols: Vec<Symbol> = assignment
            .skip(1) // Skips the lvalue
            .map(|pair| parse_pair(pair))
            .collect();

        statements.push(Statement(id, symbols))
    }

    statements
}

#[cfg(test)]
mod tests;
