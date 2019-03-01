extern crate lang_parser;

use lang_parser::ast::*;
use super::super::ast_visitor::AstVisitor;
use source_file::SourceFile;

use errors::functions::function_err::{function_error};

/**
 * Checks that freestanding functions have a body or are .native
 */
pub fn check_functions_body(s: &mut SourceFile) {
    let mut checker = FunctionsAttributesChecker::new(String::from(s.get_path()));
    checker.visit_file(s.get_ast());
}

struct FunctionsAttributesChecker {
    file: String,
    namespace: String,
}

impl FunctionsAttributesChecker {
    pub fn new(file: String) -> Self {
        FunctionsAttributesChecker {
            file,
            namespace: String::from(""),
        }
    }
}

impl AstVisitor for FunctionsAttributesChecker {
    fn visit_file(&mut self, n: &mut File) {
        self.namespace = String::from(n.get_namespace());
        for entity in n.get_entities() {
            match entity {
                FirstClassEntity::Function(f) => {
                    self.visit_function(f);
                },
                _ => {}
            }
        }
    }

    fn visit_function(&mut self, n: &mut Function) {
        let mut is_native = false;
        let name = String::from(n.get_name());
        let pos = n.get_pos();
        
        for attribute in n.get_attributes() {
            if attribute.get_name() == "native" {
                is_native = true;
            } else if attribute.get_name() == "abstract" {
                function_error("Freestanding functions cannot be abstract", &self.namespace, &name, &self.file, pos);
            }
        }

        if n.has_body() && is_native {
            function_error(".native freestanding functions cannot have a body", &self.namespace, &name, &self.file, pos);
        } else if !n.has_body() && !is_native {
            function_error("Non .natives freestanding functions must have a body", &self.namespace, &name, &self.file, pos);
        }
    }
}