use super::node::FromPair;
use super::expression::{QualifiedExpression, Expression};
use super::super::parser::Rule;
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub struct VariableDeclaration {
    name: String,
    type_name: String,
    value: Option<Expression>
}

#[derive(Clone, Debug)]
pub struct VariableAffectation {
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
}

impl<'a> FromPair<'a> for VariableDeclaration {
    fn from_pair<'b>(pair: Pair<'b, Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::declaration);

        let mut inner_pair = pair.into_inner();
        let name = String::from(inner_pair.next().unwrap().as_str());
        let type_name = String::from(inner_pair.next().unwrap().as_str());
        let value = match inner_pair.next() {
            Some(expr_pair) => Some(Expression::from_pair(expr_pair)),
            None => None
        };

        VariableDeclaration {
            name,
            type_name,
            value
        }
    }
}

impl VariableAffectation {
    pub fn get_receiver(&self) -> &QualifiedExpression {
        &self.receiver
    }

    pub fn get_value(&self) -> &Expression {
        &self.value
    }
}

impl<'a> FromPair<'a> for VariableAffectation {
    fn from_pair<'b>(pair: Pair<'b, Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::affectation);

        let mut inner_iter = pair.into_inner();

        VariableAffectation {
            receiver: QualifiedExpression::from_pair(inner_iter.next().unwrap()),
            value: Expression::from_pair(inner_iter.next().unwrap())
        }
    }
}
