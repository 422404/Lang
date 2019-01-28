use super::node::FromPair;
use super::expression::{QualifiedExpression, Expression};
use super::super::parser::Rule;
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub struct VariableDeclaration {
    pos: (usize, usize),
    name: String,
    type_name: String,
    value: Option<Expression>,
}

#[derive(Clone, Debug)]
pub struct VariableAffectation {
    pos: (usize, usize),
    receiver: QualifiedExpression,
    value: Expression,
}

impl VariableDeclaration {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_type_name(&self) -> &str {
        &self.type_name
    }

    pub fn get_value(&self) -> &Option<Expression> {
        &self.value
    }

    pub fn new(pos: (usize, usize), name: String, type_name: String, value: Option<Expression>) -> Self {
        VariableDeclaration {
            pos,
            name,
            type_name,
            value,
        }
    }
}

impl<'a> FromPair<'a> for VariableDeclaration {
    fn from_pair<'b>(pair: Pair<'b, Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::declaration);

        let pos = pair.as_span().start_pos().line_col();
        let mut inner_pair = pair.into_inner();
        let name = String::from(inner_pair.next().unwrap().as_str());
        let type_name = String::from(inner_pair.next().unwrap().as_str());
        let value = match inner_pair.next() {
            Some(expr_pair) => Some(Expression::from_pair(expr_pair)),
            None => None
        };

        VariableDeclaration {
            pos,
            name,
            type_name,
            value
        }
    }

    fn get_pos(&self) -> (usize, usize) {
        self.pos
    }
}

impl VariableAffectation {
    pub fn get_receiver(&self) -> &QualifiedExpression {
        &self.receiver
    }

    pub fn get_value(&self) -> &Expression {
        &self.value
    }

    pub fn new(pos: (usize, usize), receiver: QualifiedExpression, value: Expression) -> Self {
        VariableAffectation {
            pos,
            receiver,
            value,
        }
    }
}

impl<'a> FromPair<'a> for VariableAffectation {
    fn from_pair<'b>(pair: Pair<'b, Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::affectation);

        let pos = pair.as_span().start_pos().line_col();
        let mut inner_iter = pair.into_inner();

        VariableAffectation {
            pos,
            receiver: QualifiedExpression::from_pair(inner_iter.next().unwrap()),
            value: Expression::from_pair(inner_iter.next().unwrap())
        }
    }

    fn get_pos(&self) -> (usize, usize) {
        self.pos
    }
}
