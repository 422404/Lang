use super::node::FromPair;
use super::super::parser::Rule;
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub struct Param {
    pos: (usize, usize),
    name: String,
    type_name: String
}

impl Param {
    pub fn get_type(&self) -> &str {
        &self.type_name
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

impl<'a> FromPair<'a> for Param {
    fn from_pair<'b>(pair: Pair<'b, Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::ident_type_pair);

        let pos = pair.as_span().start_pos().line_col();
        let mut inner_iter = pair.into_inner();

        Param {
            pos,
            name: String::from(inner_iter.next().unwrap().as_str()),
            type_name: String::from(inner_iter.next().unwrap().as_str())
        }
    }

    fn get_pos(&self) -> (usize, usize) {
        self.pos
    }
}
