use super::node::FromPair;
use super::super::parser::Rule;
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub struct Identifier {
    pos: (usize, usize),
    name: String,
}

impl Identifier {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn new(pos: (usize, usize), name: String) -> Self {
        Identifier {
            pos,
            name,
        }
    }
}

impl<'a> FromPair<'a> for Identifier {
    fn from_pair<'b>(pair: Pair<'b, Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::ident);

        Identifier {
            pos: pair.as_span().start_pos().line_col(),
            name: String::from(pair.as_str()),
        }
    }

    fn get_pos(&self) -> (usize, usize) {
        self.pos
    }
}

#[derive(Clone, Debug)]
pub struct StringLitteral {
    pos: (usize, usize),
    value: String,
}

impl StringLitteral {
    pub fn get_value(&self) -> &str {
        &self.value
    }

    pub fn new(pos: (usize, usize), value: String) -> Self {
        StringLitteral {
            pos,
            value,
        }
    }
}

impl<'a> FromPair<'a> for StringLitteral {
    fn from_pair<'b>(pair: Pair<'b, Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::string);

        let pos = pair.as_span().start_pos().line_col();
        let chars = pair.into_inner().next().unwrap();
        assert_eq!(chars.as_rule(), Rule::string_chars);
        
        StringLitteral {
            pos,
            value: String::from(chars.as_str()),
        }
    }

    fn get_pos(&self) -> (usize, usize) {
        self.pos
    }
}

#[derive(Clone, Debug)]
pub struct Integer {
    pos: (usize, usize),
    value: i32,
}

impl Integer {
    pub fn get_value(&self) -> &i32 {
        &self.value
    }

    pub fn new(pos: (usize, usize), value: i32) -> Self {
        Integer {
            pos,
            value,
        }
    }
}

impl<'a> FromPair<'a> for Integer {
    fn from_pair<'b>(pair: Pair<'b, Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::integer);

        Integer {
            pos: pair.as_span().start_pos().line_col(),
            value: pair.as_str().parse::<i32>().unwrap(),
        }
    }

    fn get_pos(&self) -> (usize, usize) {
        self.pos
    }
}

#[derive(Clone, Debug)]
pub struct Char {
    pos: (usize, usize),
    value: char,
}

impl Char {
    pub fn get_value(&self) -> &char {
        &self.value
    }

    pub fn new(pos: (usize, usize), value: char) -> Self {
        Char {
            pos,
            value,
        }
    }
}

impl<'a> FromPair<'a> for Char {
    fn from_pair<'b>(pair: Pair<'b, Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::character);

        Char {
            pos: pair.as_span().start_pos().line_col(),
            value: char::from(pair.as_str().as_bytes()[1]),
        }
    }

    fn get_pos(&self) -> (usize, usize) {
        self.pos
    }
}

#[derive(Clone, Debug)]
pub struct Boolean {
    pos: (usize, usize),
    value: bool,
}

impl Boolean {
    pub fn get_value(&self) -> &bool {
        &self.value
    }

    pub fn new(pos: (usize, usize), value: bool) -> Self {
        Boolean {
            pos,
            value,
        }
    }
}

impl<'a> FromPair<'a> for Boolean {
    fn from_pair<'b>(pair: Pair<'b, Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::boolean);

        Boolean {
            pos: pair.as_span().start_pos().line_col(),
            value: pair.as_str().parse::<bool>().unwrap(),
        }
    }

    fn get_pos(&self) -> (usize, usize) {
        self.pos
    }
}
