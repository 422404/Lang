use pest::Parser;
use ast::File;
use ast::FromPair;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct LangParser;

pub fn parse<'a>(code: &'a str) -> File {
    let f = LangParser::parse(Rule::file, code).expect("cannot parse string")
            .next().unwrap();
    File::from_pair(f)
}
