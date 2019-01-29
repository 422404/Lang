use super::node::FromPair;
use super::param::Param;
use super::statement::Statement;
use super::super::parser::Rule;
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub struct Closure {
    pos: (usize, usize),
    params: Vec<Param>,
    return_type: String,
    statements: Vec<Statement>,
}

impl Closure {
    pub fn get_params(&mut self) -> &mut Vec<Param> {
        &mut self.params
    }

    pub fn get_return_type(&self) -> &str {
        &self.return_type
    }

    pub fn get_statements(&mut self) ->&mut Vec<Statement> {
        &mut self.statements
    }

    pub fn new(pos: (usize, usize), params: Vec<Param>, return_type: String,
            statements: Vec<Statement>) -> Self {
        Closure {
            pos,
            params,
            return_type,
            statements,
        }
    }
}

impl<'a> FromPair<'a> for Closure {
    fn from_pair<'b>(pair: Pair<'b, Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::closure);

        let pos = pair.as_span().start_pos().line_col();
        let mut params: Vec<Param> = vec![];
        let mut return_type = String::new();
        let mut statements: Vec<Statement> = vec![];

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::param_list => {
                    for param in pair.into_inner() {
                        params.push(Param::from_pair(param));
                    }
                },
                Rule::_type => {
                    return_type.insert_str(0, pair.as_str());
                },
                Rule::executable_body => {
                    for statement in pair.into_inner() {
                        statements.push(Statement::from_pair(statement));
                    }
                },
                _ => {}
            }
        }

        Closure {
            pos,
            params,
            return_type,
            statements
        }
    }

    fn get_pos(&self) -> (usize, usize) {
        self.pos
    }
}
