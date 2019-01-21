extern crate pest;
#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate lazy_static;

pub mod ast;
mod parser;

pub use parser::{
    LangParser,
    parse
};
