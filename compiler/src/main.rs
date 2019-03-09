extern crate lang_parser;
mod source_file;
mod errors;
mod ast_actions;
mod symbol_table;

use source_file::SourceFile;
use ast_actions::{
    modifiers::{block_expander},
    validators::{
        class_methods_checker1,
        functions_checker1
    },
};
use symbol_table::{create_symbol_table};
use std::env;
use std::collections::{HashMap, hash_map::Entry};

fn main() {
    let raw_asts = collect_raw_asts();
    let mut namespaces = namespaces_map(raw_asts);

    // debug
    for source_files in namespaces.values_mut() {
        for source_file in source_files {
            // validations
            class_methods_checker1::check_methods_body(source_file);
            functions_checker1::check_functions_body(source_file);

            // alterations
            block_expander::expand_blocks(source_file);
        }
    }
    /*for source_files in namespaces.values_mut() {
        for source_file in source_files {
            println!("{:#?}", source_file);
        }
    }*/
    let symbols = create_symbol_table(namespaces);
    println!("{:#?}", symbols);
}

fn collect_raw_asts() -> Vec<SourceFile> {
    env::args()
            .skip(1)
            .map(|file_path| SourceFile::from_file(&file_path))
            .collect()
}

fn namespaces_map(asts: Vec<SourceFile>) -> HashMap<String, Vec<SourceFile>> {
    let mut map: HashMap<String, Vec<SourceFile>> = HashMap::new();
    for mut source_file in asts {
        match map.entry(String::from(source_file.get_ast().get_namespace())) {
            Entry::Occupied(mut e) => {
                e.get_mut().push(source_file);
            },
            Entry::Vacant(mut e) => {
                e.insert(vec![source_file]);
            }
        }
    }

    map
}
