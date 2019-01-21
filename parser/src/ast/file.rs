use super::node::FromPair;
use super::function::Function;
use super::class::Class;
use super::super::parser::Rule;
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub struct File {
    namespace: String,
    imports: Vec<String>,
    entities: Vec<FirstClassEntity>
}

#[derive(Clone, Debug)]
pub enum FirstClassEntity {
    Function(Function),
    Class(Class),
}

impl File {
    pub fn get_namespace(&self) -> &str {
        &self.namespace
    }

    pub fn get_imports(&self) -> &Vec<String> {
        &self.imports
    }

    pub fn get_entities(&self) -> &Vec<FirstClassEntity> {
        &self.entities
    }
}

impl<'a> FromPair<'a> for File {
    fn from_pair<'b>(pair: Pair<'b, Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::file);
        let mut imports: Vec<String> = vec![];
        let mut namespace: String = String::new();
        let mut entities: Vec<FirstClassEntity> = vec![];

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::namespace => {
                    for pair in pair.into_inner() {
                        match pair.as_rule() {
                            Rule::qualified_type => {
                                namespace.insert_str(0, pair.as_str());
                            },
                            _ => {}
                        }
                    }
                },
                Rule::import => {
                    for pair in pair.into_inner() {
                        match pair.as_rule() {
                            Rule::qualified_type => {
                                imports.push(String::from(pair.as_str()));
                            },
                            _ => {}
                        }
                    }
                },
                Rule::method_decl => {
                    entities.push(FirstClassEntity::Function(Function::from_pair(pair)));
                },
                Rule::class_decl => {
                    entities.push(FirstClassEntity::Class(Class::from_pair(pair)));
                },
                _ => {}
            }
        }

        File {
            namespace,
            imports,
            entities,
        }
    }

    fn get_pos(&self) -> (usize, usize) {
        (0, 0)
    }
}
