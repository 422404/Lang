extern crate lang_parser;
mod source_file;

use source_file::SourceFile;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let source_file = SourceFile::from_file(file);
    println!("{:#?}", source_file);
}
