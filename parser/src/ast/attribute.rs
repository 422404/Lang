use super::node::FromPair;
use super::super::parser::Rule;
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub struct Attribute {
    pos: (usize, usize),
    name: String,
}

impl Attribute {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn new(pos: (usize, usize), name: String) -> Self {
        Attribute {
            pos,
            name,
        }
    }
}

impl<'a> FromPair<'a> for Attribute {
    fn from_pair<'b>(pair: Pair<'b, Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::attribute);
        
        let pos = pair.as_span().start_pos().line_col();
        let inner_pair = pair.into_inner().next().unwrap();

        Attribute {
            pos,
            name: String::from(inner_pair.as_str())
        }
    }

    fn get_pos(&self) -> (usize, usize) {
        self.pos
    }
}
