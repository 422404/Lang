use super::node::FromPair;
use super::super::parser::Rule;
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub struct Attribute {
    pub name: String
}

impl Attribute {
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

impl<'a> FromPair<'a> for Attribute {
    fn from_pair<'b>(pair: Pair<'b, Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::attribute);
        
        let inner_pair = pair.into_inner().next().unwrap();

        Attribute {
            name: String::from(inner_pair.as_str())
        }
    }
}
