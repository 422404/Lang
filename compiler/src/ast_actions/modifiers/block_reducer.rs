extern crate lang_parser;

use lang_parser::ast::*;
use source_file::SourceFile;
use super::super::ast_fold::AstFold;

/**
 * Applies block's attributes to its members and move them directly in
 * the surrounding class
 */
pub fn reduce_blocks(s: &SourceFile) -> SourceFile {
    let mut folder = BlockReducer::new();
    let ast = folder.fold_file(s.get_ast().clone());

    SourceFile::new(String::from(s.get_path()), String::from(s.get_code()), ast)
}

fn reduce_block(b: &Block) -> Vec<ClassMember> {
    let attributes = b.get_attributes();
    let members: &Vec<ClassMember> = b.get_members();
    let mut new_members: Vec<ClassMember> = vec![];

    for member in members {
        match member {
            ClassMember::Field(f) => {
                new_members.push(ClassMember::Field(
                    Field::new(
                        f.get_pos(),
                        attributes.clone(),
                        String::from(f.get_name()),
                        String::from(f.get_type_name()),
                    )
                ));
            },
            ClassMember::Method(m) => {
                new_members.push(ClassMember::Method(
                    Function::new(
                        m.get_pos(),
                        attributes.clone(),
                        String::from(m.get_name()),
                        m.get_params().clone(),
                        String::from(m.get_return_type()),
                        m.get_statements().clone(),
                    )
                ));
            },
            ClassMember::Block(b) => {
                for reduced_block in reduce_block(b) {
                    new_members.push(reduced_block);
                }
            }
        }
    }

    new_members
}

struct BlockReducer;

impl BlockReducer {
    pub fn new() -> Self {
        BlockReducer {}
    }
}

impl AstFold for BlockReducer {
    fn fold_file(&mut self, n: File) -> File {
        let mut new_entities: Vec<FirstClassEntity> = vec![];
        for entity in n.get_entities() {
            match entity {
                FirstClassEntity::Function(f) => {
                    new_entities.push(FirstClassEntity::Function(f.clone()));
                },
                FirstClassEntity::Class(c) => {
                   new_entities.push(FirstClassEntity::Class(self.fold_class(c.clone())));
                },
            }
        }

        File::new(String::from(n.get_namespace()), n.get_imports().clone(),
                new_entities)
    }

    fn fold_class(&mut self, n: Class) -> Class {
        let mut new_members: Vec<ClassMember> = vec![];
        for member in n.get_members() {
            match member {
                ClassMember::Field(f) => {
                    new_members.push(ClassMember::Field(f.clone()));
                },
                ClassMember::Method(m) => {
                    new_members.push(ClassMember::Method(m.clone()));
                },
                ClassMember::Block(b) => {
                    for member in reduce_block(b) {
                        new_members.push(member);
                    }
                }
            }
        }

        Class::new(n.get_pos(), n.get_attributes().clone(), String::from(n.get_name()),
                String::from(n.get_super_name()), n.get_implemented_interfaces().clone(),
                new_members)
    }
}
