extern crate lang_parser;

use lang_parser::ast::*;
use source_file::SourceFile;
use super::super::ast_visitor::AstVisitor;

/**
 * Applies block's attributes to its members and move them directly in
 * the surrounding class
 */
pub fn expand_blocks(s: &mut SourceFile) -> () {
    let mut visitor = BlockReducer::new();
    visitor.visit_file(s.get_ast());
}

fn expand_block(b: &mut Block) -> Vec<ClassMember> {
    let attributes = b.get_attributes().clone();
    let mut new_members: Vec<ClassMember> = vec![];

    for mut member in b.get_members() {
        match member {
            ClassMember::Field(f) => {
                new_members.push(ClassMember::Field(
                    Field::new(
                        f.get_pos(),
                        {
                            let mut v = vec![];
                            v.append(f.get_attributes());
                            v.append(&mut attributes.clone());
                            v
                        },
                        String::from(f.get_name()),
                        String::from(f.get_type_name()),
                    )
                ));
            },
            ClassMember::Method(m) => {
                new_members.push(ClassMember::Method(
                    Function::new(
                        m.get_pos(),
                        {
                            let mut v = vec![];
                            v.append(m.get_attributes());
                            v.append(&mut attributes.clone());
                            v
                        },
                        String::from(m.get_name()),
                        m.get_params().clone(),
                        String::from(m.get_return_type()),
                        m.has_body(),
                        m.get_statements().clone(),
                    )
                ));
            },
            ClassMember::Block(b) => {
                for reduced_block in expand_block(b) {
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

impl AstVisitor for BlockReducer {
    fn visit_class(&mut self, n: &mut Class) -> () {
        let mut expanded_blocks: Vec<ClassMember> = vec![];
        for member in n.get_members() {
            match member {
                ClassMember::Block(b) => {
                    for member in expand_block(b) {
                        expanded_blocks.push(member);
                    }
                },
                _ => {}
            }
        }
        // remove all blocks
        n.get_members().retain(|m| match m {
            ClassMember::Block(_) => false,
            _ => true
        });
        n.get_members().append(&mut expanded_blocks);
    }
}
