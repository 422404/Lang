extern crate lang_parser;

use lang_parser::ast::*;
use source_file::SourceFile;
use ast_actions::ast_visitor::AstVisitor;
use std::collections::HashMap;

// @@image(../../.docs/compiler/symbol_table.jpg)

/**
 * First level symbol table.
 * Constains all the namespace symbol tables.
 */
pub type GlobalSymbolTable = HashMap<String, NamespaceSymbolTable>;

/**
 * Second level symbol table.
 * Contains the defined classes and functions inside a namespace.
 */
pub type NamespaceSymbolTable = HashMap<String, NSTEntry>;

#[derive(Clone, Debug)]
pub enum NSTEntry {
    Class(String, Vec<String>, (usize, usize), ClassSymbolTable), // (base class, implemented interfaces, pos, CST)
    Fun(String, (usize, usize), FunctionSymbolTable), // (return type, pos, FST)
}

/**
 * Third level symbol table.
 * Contains the defined fields and methods in a class.
 */
pub type ClassSymbolTable = HashMap<String, CSTEntry>;

#[derive(Clone, Debug)]
pub enum CSTEntry {
    Field(String, (usize, usize)), // (type, pos)
    Method(String, (usize, usize), FunctionSymbolTable), // (return type, pos, FST)
}

/**
 * Thrid and fourth level symbol table.
 * Constains the defined parameters and variables in a function or method.
 */
pub type FunctionSymbolTable = HashMap<String, FSTEntry>;

#[derive(Clone, Debug)]
pub enum FSTEntry {
    Param(String, (usize, usize)), // (type, pos)
    Var(String, (usize, usize)),   // (type, pos)
    Closure(String, (usize, usize), FunctionSymbolTable), // (return type, pos, FST)
}

/**
 * Create a global symbol table from processed ASTs
 */
pub fn create_symbol_table(namespaces: HashMap<String, Vec<SourceFile>>) -> GlobalSymbolTable {
    let mut gst = GlobalSymbolTable::new();
    for (ns, files) in namespaces {
        let mut nst = NamespaceSymbolTable::new();
        {
            let mut stc = SymbolTableCreator::new(&mut nst);
            for mut file in files {
                stc.visit_file(file.get_ast());
            }
        }
        // nst no longer borrowed
        gst.insert(ns, nst);
    }
    gst
}

struct SymbolTableCreator<'a> {
    nst: &'a mut NamespaceSymbolTable,
    cst: Option<ClassSymbolTable>,
    fst: Option<FunctionSymbolTable>,
    closures_fst: Vec<FunctionSymbolTable>,
    in_closure: bool,
}

impl<'a, 'b: 'a> SymbolTableCreator<'a> {
    pub fn new(nst: &'b mut NamespaceSymbolTable) -> Self {
        SymbolTableCreator {
            nst,
            cst: None,
            fst: None,
            closures_fst: vec![],
            in_closure: false,
        }
    }
}

impl<'a> AstVisitor for SymbolTableCreator<'a> {
    fn visit_function(&mut self, n: &mut Function) -> () {
        let fst = FunctionSymbolTable::new();
        self.fst = Some(fst);

        for param in n.get_params() {
            self.visit_param(param);
        }
        for stmt in n.get_statements() {
            self.visit_statement(stmt);
        }

        let fst_copy = self.fst.clone().unwrap();
        match &mut self.cst {
            Some(cst) => {
                // if cst is some then we are a method
                cst.insert(String::from(n.get_name()), CSTEntry::Method(String::from(n.get_return_type()), n.get_pos(), fst_copy));
            },
            None => {
                // if cst is none we are a freestanding fuunction
                self.nst.insert(String::from(n.get_name()), NSTEntry::Fun(String::from(n.get_return_type()), n.get_pos(), fst_copy));
            }
        }
        self.fst = None;
    }

    fn visit_closure(&mut self, n: &mut Closure) -> () {
        let fst = FunctionSymbolTable::new();
        self.closures_fst.push(fst);
        self.in_closure = true;

        for param in n.get_params() {
            self.visit_param(param);
        }
        for stmt in n.get_statements() {
            self.visit_statement(stmt);
        }

        let fst_copy = self.closures_fst.last().unwrap().clone();
        let name = format!("<closure{}>", self.closures_fst.len());
        let entry = FSTEntry::Closure(String::from(n.get_return_type()), n.get_pos(), fst_copy);
        if self.closures_fst.len() == 1 {
            if let Some(cur_fst) = &mut self.fst {
                cur_fst.insert(name, entry);
            }
        } else {
            let i = self.closures_fst.len() - 1;
            self.closures_fst[i].insert(name, entry);
        }

        self.closures_fst.pop();
        self.in_closure = self.closures_fst.len() == 0;
    }

    fn visit_param(&mut self, n: &mut Param) -> () {
        match &mut self.fst {
            Some(fst) => {
                if self.in_closure {
                    let i = self.closures_fst.len() - 1;
                    self.closures_fst[i].insert(String::from(n.get_name()), FSTEntry::Param(String::from(n.get_type()), n.get_pos()));
                } else {
                    fst.insert(String::from(n.get_name()), FSTEntry::Param(String::from(n.get_type()), n.get_pos()));
                }
            },
            None => unreachable!()
        }
    }

    fn visit_variable_declaration(&mut self, n: &mut VariableDeclaration) -> () {
        match &mut self.fst {
            Some(fst) => {
                if self.in_closure {
                    let i = self.closures_fst.len() - 1;
                    self.closures_fst[i].insert(String::from(n.get_name()), FSTEntry::Var(String::from(n.get_type_name()), n.get_pos()));
                } else {
                    fst.insert(String::from(n.get_name()), FSTEntry::Var(String::from(n.get_type_name()), n.get_pos()));
                }
            },
            None => unreachable!()
        }
    }

    fn visit_class(&mut self, n: &mut Class) -> () {
        let cst = ClassSymbolTable::new();
        self.cst = Some(cst);

        for member in n.get_members() {
            self.visit_class_member(member);
        }
        let cst_copy = self.cst.clone().unwrap();
        self.nst.insert(String::from(n.get_name()),
                NSTEntry::Class(String::from(n.get_super_name()), n.get_implemented_interfaces().clone(), n.get_pos(), cst_copy));
        self.cst = None;
    }

    fn visit_field(&mut self, n: &mut Field) -> () {
        match &mut self.cst {
            Some(cst) => {
                cst.insert(String::from(n.get_name()), CSTEntry::Field(String::from(n.get_type_name()), n.get_pos()));
            },
            None => unreachable!()
        }
    }
}
