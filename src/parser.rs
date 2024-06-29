use pest::Parser;
use pest_derive::Parser;
//TODO: fix the build errors
#[derive(Parser)]
#[grammar = "vera.pest"]
pub struct SimpleLangParser;

pub fn parse_program(input: &str) -> Result<pest::iterators::Pairs<Rule>, pest::error::Error<Rule>> {
    SimpleLangParser::parse(Rule::program, input)
}
