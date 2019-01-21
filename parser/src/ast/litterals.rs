use super::node::FromPair;
use super::super::parser::Rule;
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub struct Identifier {
    name: String,
}

impl Identifier {
    pub fn get_name(&self) -> &String {
        &self.name
    }
}

impl<'a> FromPair<'a> for Identifier {
    fn from_pair<'b>(pair: Pair<'b, Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::ident);

        Identifier {
            name: String::from(pair.as_str()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct StringLitteral {
    value: String,
}

impl StringLitteral {
    pub fn get_value(&self) -> &String {
        &self.value
    }
}

impl<'a> FromPair<'a> for StringLitteral {
    fn from_pair<'b>(pair: Pair<'b, Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::string);

        let chars = pair.into_inner().next().unwrap();
        assert_eq!(chars.as_rule(), Rule::string_chars);
        
        StringLitteral {
            value: String::from(chars.as_str()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Integer {
    value: i32,
}

impl Integer {
    pub fn get_value(&self) -> &i32 {
        &self.value
    }
}

impl<'a> FromPair<'a> for Integer {
    fn from_pair<'b>(pair: Pair<'b, Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::integer);

        Integer {
            value: pair.as_str().parse::<i32>().unwrap(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Char {
    value: char,
}

impl Char {
    pub fn get_value(&self) -> &char {
        &self.value
    }
}

impl<'a> FromPair<'a> for Char {
    fn from_pair<'b>(pair: Pair<'b, Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::character);

        Char {
            value: char::from(pair.as_str().as_bytes()[1]),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Boolean {
    value: bool,
}

impl Boolean {
    pub fn get_value(&self) -> &bool {
        &self.value
    }
}

impl<'a> FromPair<'a> for Boolean {
    fn from_pair<'b>(pair: Pair<'b, Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::boolean);

        Boolean {
            value: pair.as_str().parse::<bool>().unwrap(),
        }
    }
}
