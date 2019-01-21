use super::node::FromPair;
use super::closure::Closure;
use super::function::FunctionCall;
use super::litterals::{Identifier, Integer, StringLitteral, Char, Boolean};
use super::super::parser::Rule;
use pest::iterators::{Pair, Pairs};
use pest::prec_climber::{Assoc::*, Operator, PrecClimber};

lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        PrecClimber::new(vec![
            Operator::new(Rule::eq_op, Left)
            | Operator::new(Rule::ne_op, Left),
            Operator::new(Rule::gt_op, Left)
            | Operator::new(Rule::ge_op, Left)
            | Operator::new(Rule::lt_op, Left)
            | Operator::new(Rule::le_op, Left),
            Operator::new(Rule::ad_op, Left)
            | Operator::new(Rule::mn_op, Left),
            Operator::new(Rule::tm_op, Left)
            | Operator::new(Rule::dv_op, Left)
            | Operator::new(Rule::md_op, Left),
        ])
    };
}

#[derive(Clone, Debug)]
pub enum QualifiedExpressionPart {
    MethodCall(FunctionCall),
    Identifier(Identifier),
    Integer(Integer),
    StringLitteral(StringLitteral),
    Char(Char),
    Boolean(Boolean),
    Null,
    Closure(Closure),
    ParenExpr(Expression),
}

#[derive(Clone, Debug)]
pub struct QualifiedExpression {
    pos: (usize, usize),
    parts: Vec<QualifiedExpressionPart>
}

#[derive(Clone, Debug)]
pub enum Expression {
    Operation(Operation),
    Expr(QualifiedExpression),
}

#[derive(Clone, Debug)]
pub struct Operation {
    lval: Box<Expression>,
    rval: Box<Expression>,
    op: OperationType,
}

#[derive(Clone, Debug)]
pub enum OperationType {
    Eqal,
    NotEqual,
    GreaterOrEqual,
    LowerOrEqual,
    GreaterThan,
    LowerThan,
    Add,
    Minus,
    Times,
    Div,
    Mod,
}

impl Operation {
    fn new(lval: Expression, rval: Expression, op: OperationType) -> Self {
        Operation {
            lval: Box::new(lval),
            rval: Box::new(rval),
            op,
        }
    }
}

impl<'a> FromPair<'a> for QualifiedExpression {
    fn from_pair<'b>(pair: Pair<'b, Rule>) -> Self {
        if pair.as_rule() != Rule::qualified_expression
                && pair.as_rule() != Rule::qualified_ident {
            panic!("Cannot build QualifiedExpression from rule: {:#?}", pair.as_rule());
        }

        let mut parts: Vec<QualifiedExpressionPart> = vec![];
        let pos = pair.as_span().start_pos().line_col();
        for pair in pair.into_inner() {
            parts.push(QualifiedExpressionPart::from_pair(pair));
        }

        QualifiedExpression {
            pos,
            parts,
        }
    }

    fn get_pos(&self) -> (usize, usize) {
        self.pos
    }
}

impl<'a> FromPair<'a> for QualifiedExpressionPart {
    fn from_pair<'b>(pair: Pair<'b, Rule>) -> Self {
        match pair.as_rule() {
            Rule::ident       => QualifiedExpressionPart::Identifier(Identifier::from_pair(pair)),
            Rule::method_call => QualifiedExpressionPart::MethodCall(FunctionCall::from_pair(pair)),
            Rule::integer     => QualifiedExpressionPart::Integer(Integer::from_pair(pair)),
            Rule::string      =>  QualifiedExpressionPart::StringLitteral(StringLitteral::from_pair(pair)),
            Rule::character   => QualifiedExpressionPart::Char(Char::from_pair(pair)),
            Rule::boolean     => QualifiedExpressionPart::Boolean(Boolean::from_pair(pair)),
            Rule::null        => QualifiedExpressionPart::Null,
            Rule::closure     => QualifiedExpressionPart::Closure(Closure::from_pair(pair)),
            Rule::expression  => QualifiedExpressionPart::ParenExpr(Expression::from_pair(pair)),
            _                 => unreachable!()
        }
    }

    fn get_pos(&self) -> (usize, usize) {
        match self {
            QualifiedExpressionPart::Identifier(i)     => i.get_pos(),
            QualifiedExpressionPart::MethodCall(f)     => f.get_pos(),
            QualifiedExpressionPart::Integer(int)      => int.get_pos(),
            QualifiedExpressionPart::StringLitteral(s) => s.get_pos(),
            QualifiedExpressionPart::Char(c)           => c.get_pos(),
            QualifiedExpressionPart::Boolean(b)        => b.get_pos(),
            QualifiedExpressionPart::Closure(cl)       => cl.get_pos(),
            QualifiedExpressionPart::ParenExpr(e)      => e.get_pos(),
            QualifiedExpressionPart::Null              => unimplemented!(),
        }
    }
}

impl<'a> FromPair<'a> for Expression {
    fn from_pair<'b>(pair: Pair<'b, Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::expression);
        
        let inner_iter: Pairs<'b, Rule> = pair.into_inner();
        PREC_CLIMBER.climb(
            inner_iter,
            |pair: Pair<'b, Rule>| {
                assert_eq!(pair.as_rule(), Rule::qualified_expression);
                Expression::Expr(QualifiedExpression::from_pair(pair))
            },
            |lval: Expression, op: Pair<'b, Rule>, rval: Expression| {
                let operation = match op.as_rule() {
                    Rule::eq_op => OperationType::Eqal,
                    Rule::ne_op => OperationType::NotEqual,
                    Rule::gt_op => OperationType::GreaterThan,
                    Rule::ge_op => OperationType::GreaterOrEqual,
                    Rule::lt_op => OperationType::LowerThan,
                    Rule::le_op => OperationType::LowerOrEqual,
                    Rule::ad_op => OperationType::Add,
                    Rule::mn_op => OperationType::Minus,
                    Rule::tm_op => OperationType::Times,
                    Rule::dv_op => OperationType::Div,
                    Rule::md_op => OperationType::Mod,
                    _ => unreachable!()
                };
                Expression::Operation(Operation::new(lval, rval, operation))
            }
        ).clone()
    }

    fn get_pos(&self) -> (usize, usize) {
        match self {
            Expression::Expr(e)      => e.get_pos(),
            Expression::Operation(_) => unimplemented!()
        }
    }
}
