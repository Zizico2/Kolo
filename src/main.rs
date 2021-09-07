/*
use rhai::{Engine, EvalAltResult, Scope, INT, Array, Dynamic};

fn main() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();
    let mut scope = Scope::new();

    engine.eval_with_scope::<()>(&mut scope, "let a = 3;")?;

    let result = engine.eval_expression_with_scope::<Vec<Dynamic>>(&mut scope, "[23, 54]")?;

    //println!("Answer: {}", result); // prints 3

    Ok(())
}
*/
extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct IdentParser;

fn main() {
    let pairs = IdentParser::parse(Rule::main, "{sasa{bl{bl2}}{bl3}}}").unwrap_or_else(|e| panic!("{}", e));

    // Because ident_list is silent, the iterator will contain idents
    for pair in pairs {
        // A pair is a combination of the rule which matched and a span of input
        //println!("Rule:    {:?}", pair.as_rule());
        //println!("Span:    {:?}", pair.as_span());
        //println!("Text:    {}", pair.as_str());

        // A pair can be converted to an iterator of the tokens which make it up:
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::block => println!("Block:  {}", inner_pair.as_str()),
                Rule::html => println!("HTML:   {}", inner_pair.as_str()),
                _ => unreachable!()
            };
        }
    }
}