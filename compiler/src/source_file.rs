extern crate lang_parser;

use lang_parser::ast::File;
use std::fs;

#[derive(Clone, Debug)]
pub struct SourceFile {
    code: String,
    ast: File,
}

impl SourceFile {
    pub fn from_file(path: &String) -> Self {
        let mut code_string: String = String::new();

        match fs::read_to_string(path) {
            Ok(code) => {
                code_string.insert_str(0, &code);
            },
            Err(error) => panic!(error)
        }

        let ast = lang_parser::parse(&code_string.to_owned());

        SourceFile {
            code: code_string,
            ast,
        }
    }
}
