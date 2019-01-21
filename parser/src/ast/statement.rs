use super::expression::{Expression, QualifiedExpression};
use super::node::FromPair;
use super::variable::{VariableDeclaration, VariableAffectation};
use super::super::parser::Rule;
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub enum Statement {
    ReturnStatement(Expression),
    Declaration(VariableDeclaration),
    Affectation(VariableAffectation),
    QualifiedExpression(QualifiedExpression),
}

impl<'a> FromPair<'a> for Statement {
    fn from_pair<'b>(pair: Pair<'b, Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::statement);

        let inner_pair = pair.into_inner().next().unwrap();
        match inner_pair.as_rule() {
            Rule::return_stmt => {
                let expr = inner_pair.into_inner().next().unwrap();
                Statement::ReturnStatement(Expression::from_pair(expr))
            },
            Rule::declaration => {
                Statement::Declaration(VariableDeclaration::from_pair(inner_pair))
            },
            Rule::affectation => {
                Statement::Affectation(VariableAffectation::from_pair(inner_pair))
            },
            Rule::qualified_expression => {
                Statement::QualifiedExpression(QualifiedExpression::from_pair(inner_pair))
            },
            _ => unreachable!()
        }
    }

    fn get_pos(&self) -> (usize, usize) {
        match self {
            Statement::ReturnStatement(r)     => r.get_pos(),
            Statement::Declaration(d)         => d.get_pos(),
            Statement::Affectation(a)         => a.get_pos(),
            Statement::QualifiedExpression(e) => e.get_pos(),
        }
    }
}
