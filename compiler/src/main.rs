extern crate lang_parser;
mod source_file;
mod errors;
mod ast_actions;

use source_file::SourceFile;
use ast_actions::{
    modifiers::{block_reducer},
    validators::{class_methods_checker},
};
use std::env;
use std::collections::{HashMap, hash_map::Entry};

fn main() {
    let raw_asts = collect_raw_asts();
    let namespaces = namespaces_map(raw_asts);

    // debug
    namespaces.values().for_each(|sources_files|
        sources_files.iter().for_each(|source_file| {
            class_methods_checker::check_methods_body(source_file);
            println!("Namespace: {:#?}", block_reducer::reduce_blocks(source_file));
        })
    );
}

fn collect_raw_asts() -> Vec<SourceFile> {
    env::args()
            .skip(1)
            .map(|file_path| SourceFile::from_file(&file_path))
            .collect()
}

fn namespaces_map(asts: Vec<SourceFile>) -> HashMap<String, Vec<SourceFile>> {
    let mut map: HashMap<String, Vec<SourceFile>> = HashMap::new();
    for source_file in asts {
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
