use super::attribute::Attribute;
use super::function::Function;
use super::node::FromPair;
use super::super::parser::Rule;
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub struct Class {
    pos: (usize, usize),
    attributes: Vec<Attribute>,
    name: String,
    super_name: String,
    implemented_interfaces: Vec<String>,
    members: Vec<ClassMember>,
}

#[derive(Clone, Debug)]
pub enum ClassMember {
    Field(Field),
    Method(Function),
    Block(Block),
}

#[derive(Clone, Debug)]
pub struct Field {
    pos: (usize, usize),
    attributes: Vec<Attribute>,
    name: String,
    type_name: String,
}

#[derive(Clone, Debug)]
pub struct Block {
    pos: (usize, usize),
    attributes: Vec<Attribute>,
    members: Vec<ClassMember>,
}

impl Class {
    pub fn get_attributes(&mut self) -> &mut Vec<Attribute> {
        &mut self.attributes
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_super_name(&self) -> &str {
        &self.super_name
    }

    pub fn get_implemented_interfaces(&mut self) -> &mut Vec<String> {
        &mut self.implemented_interfaces
    }

    pub fn get_members(&mut self) -> &mut Vec<ClassMember> {
        &mut self.members
    }

    pub fn new(pos: (usize, usize),  attributes: Vec<Attribute>, name: String,
            super_name: String, implemented_interfaces: Vec<String>,
            members: Vec<ClassMember>) -> Self {
        Class {
            pos,
            attributes,
            name,
            super_name,
            implemented_interfaces,
            members,
        }
    }
}

impl<'a> FromPair<'a> for Class {
    fn from_pair<'b>(pair: Pair<'b, Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::class_decl);

        let pos = pair.as_span().start_pos().line_col();
        let mut inner_iter = pair.into_inner();
        let attributes_iter = inner_iter.next().unwrap().into_inner();
        let mut attributes: Vec<Attribute> = vec![];
        let mut implemented_interfaces: Vec<String> = vec![];
        let mut members: Vec<ClassMember> = vec![];
        let name = String::from(inner_iter.next().unwrap().as_str());
        let super_name = String::from(inner_iter.next().unwrap().as_str());
        for attr in attributes_iter {
            attributes.push(Attribute::from_pair(attr));
        }
        for pair in inner_iter {
            match pair.as_rule() {
                Rule::interface_list => {
                    for interface in pair.into_inner() {
                        implemented_interfaces.push(String::from(interface.as_str()));
                    }
                },
                Rule::field_decl => {
                    members.push(ClassMember::Field(Field::from_pair(pair)));
                },
                Rule::method_decl => {
                    members.push(ClassMember::Method(Function::from_pair(pair)));
                },
                Rule::block_decl => {
                    members.push(ClassMember::Block(Block::from_pair(pair)));
                },
                _ => unreachable!()
            }
        }

        Class {
            pos,
            attributes,
            name,
            super_name,
            implemented_interfaces,
            members,
        }
    }

    fn get_pos(&self) -> (usize, usize) {
        self.pos
    }
}

impl Field {
    pub fn get_attributes(&mut self) -> &mut Vec<Attribute> {
        &mut self.attributes
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_type_name(&self) -> &str {
        &self.type_name
    }

    pub fn new(pos: (usize, usize), attributes: Vec<Attribute>, name: String,
            type_name: String) -> Self {
        Field {
            pos,
            attributes,
            name,
            type_name,
        }
    }
}

impl<'a> FromPair<'a> for Field {
    fn from_pair<'b>(pair: Pair<'b, Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::field_decl);

        let pos = pair.as_span().start_pos().line_col();
        let mut inner_iter = pair.into_inner();
        let mut attributes: Vec<Attribute> = vec![];
        let attributes_iter = inner_iter.next().unwrap().into_inner();
        for attr in attributes_iter {
            attributes.push(Attribute::from_pair(attr));
        }
        let name = String::from(inner_iter.next().unwrap().as_str());
        let type_name = String::from(inner_iter.next().unwrap().as_str());

        Field {
            pos,
            attributes,
            name,
            type_name,
        }
    }

    fn get_pos(&self) -> (usize, usize) {
        self.pos
    }
}

impl<'a> Block {
    pub fn get_attributes(&mut self) -> &mut Vec<Attribute> {
        &mut self.attributes
    }

    pub fn get_members(&mut self) -> &mut Vec<ClassMember> {
        &mut self.members
    }

    pub fn new(pos: (usize, usize), attributes: Vec<Attribute>,
            members: Vec<ClassMember>) -> Self {
        Block {
            pos,
            attributes,
            members,
        }
    }
}

impl<'a> FromPair<'a> for Block {
    fn from_pair<'b>(pair: Pair<'b, Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::block_decl);

        let pos = pair.as_span().start_pos().line_col();
        let mut inner_iter = pair.into_inner();
        let attributes_iter = inner_iter.next().unwrap().into_inner();
        let mut attributes: Vec<Attribute> = vec![];
        let mut members: Vec<ClassMember> = vec![];
        for attr in attributes_iter {
            attributes.push(Attribute::from_pair(attr));
        }
        for pair in inner_iter {
            match pair.as_rule() {
                Rule::field_decl => {
                    members.push(ClassMember::Field(Field::from_pair(pair)));
                },
                Rule::method_decl => {
                    members.push(ClassMember::Method(Function::from_pair(pair)));
                },
                _ => unreachable!()
            }
        }

        Block {
            pos,
            attributes,
            members,
        }
    }

    fn get_pos(&self) -> (usize, usize) {
        self.pos
    }
}
