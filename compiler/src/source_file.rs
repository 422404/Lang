extern crate lang_parser;

use lang_parser::ast::File;
use std::fs;

#[derive(Clone, Debug)]
pub struct SourceFile {
    path: String,
    code: String,
    ast: File,
}

impl SourceFile {
    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn get_code(&self) -> &str {
        &self.code
    }

    pub fn get_ast(&self) -> &File {
        &self.ast
    }

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
            path: path.to_owned(),
            code: code_string,
            ast,
        }
    }

    pub fn new(path: String, code: String, ast: File) -> Self {
        SourceFile {
            path,
            code,
            ast,
        }
    }
}
