extern crate lang_parser;

use lang_parser::ast::*;
use super::super::ast_visitor::AstVisitor;
use source_file::SourceFile;
use errors::classes::class_err::{class_error, class_member_error};

/**
 * Validate .native, .interface and .abstract attributes on classes and
 * theirs methods
 */
pub fn check_methods_body(s: &mut SourceFile) {
    let mut checker = MethodsAttributesChecker::new(String::from(s.get_path()));
    checker.visit_file(s.get_ast());
}

struct MethodsAttributesChecker {
    class: String,
    namespace: String,
    file: String,
    is_abstract: bool,
    is_interface: bool,
}

impl MethodsAttributesChecker {
    pub fn new(file: String) -> Self {
        MethodsAttributesChecker {
            class: String::from(""),
            namespace: String::from(""),
            file,
            is_abstract: false,
            is_interface: false,
        }
    }
}

impl AstVisitor for MethodsAttributesChecker {
    fn visit_file(&mut self, n: &mut File) -> () {
        self.namespace = String::from(n.get_namespace());
        for entity in n.get_entities() {
            match entity {
                FirstClassEntity::Class(c) => {
                    self.class = String::from(c.get_name());
                    self.visit_class(c);
                },
                _ => {}
            }
        }
    }

    fn visit_class(&mut self, n: &mut Class) -> () {
        for attribute in n.get_attributes() {
            if attribute.get_name() == "abstract" {
                self.is_abstract = true;
            }
            if attribute.get_name() == "interface" {
                self.is_interface = true;
            }
        }

        if self.is_abstract && self.is_interface {
            class_error("cannot be abstract and interface at the same time",
                &self.namespace, &self.class, &self.file, n.get_pos());
        }
        for member in n.get_members() {
            self.visit_class_member(member);
        }
        // cleanup
        self.is_interface = false;
        self.is_abstract = false;
        self.class = String::from("");
    }

    fn visit_function(&mut self, n: &mut Function) -> () {
        let mut is_native_function = false;
        let mut is_abstract_function = false;
        let name = String::from(n.get_name());
        let pos = n.get_pos();
        for attribute in n.get_attributes() {
            if !self.is_abstract && attribute.get_name() == "abstract" {
                class_member_error("no abstract methods allowed in non-abstract classes",
                        &self.namespace, &self.class, &name, &self.file, pos);
            } else if attribute.get_name() == "native" {
                is_native_function = true;
            } else if attribute.get_name() == "abstract" {
                is_abstract_function = true;
            }
        }
        if is_abstract_function && is_native_function {
            class_member_error("cannot be abstract and native at the same time",
                &self.namespace, &self.class, &name, &self.file, pos);
        }
        let body = n.has_body();
        if (!self.is_abstract && !self.is_interface && !is_native_function) && !body {
            class_member_error("bodyless methods in normal classes are only allowed if natives",
                    &self.namespace, &self.class, &name, &self.file, pos);
        } else if self.is_interface && body {
            class_member_error("interfaces can only have bodyless methods", &self.namespace,
                    &self.class, &name, &self.file, pos);
        } else if is_native_function && body {
            class_member_error("native methods cannot have a body", &self.namespace,
                    &self.class, &name, &self.file, pos);
        } else if is_abstract_function && body {
            class_member_error("abstract methods cannot have a body", &self.namespace,
                    &self.class, &name, &self.file, pos);
        }
    }
}
