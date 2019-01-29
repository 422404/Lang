use super::node::FromPair;
use super::attribute::Attribute;
use super::param::Param;
use super::statement::Statement;
use super::expression::Expression;
use super::super::parser::Rule;
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub struct Function {
    pos: (usize, usize),
    attributes: Vec<Attribute>,
    name: String,
    params: Vec<Param>,
    return_type: String,
    statements: Vec<Statement>,
}

#[derive(Clone, Debug)]
pub struct FunctionCall {
    pos: (usize, usize),
    name: String,
    param_exprs: Vec<Expression>,
}

impl Function {
    pub fn get_attributes(&mut self) -> &mut Vec<Attribute> {
        &mut self.attributes
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_params(&mut self) -> &mut Vec<Param> {
        &mut self.params
    }

    pub fn get_return_type(&self) -> &str {
        &self.return_type
    }

    pub fn get_statements(&mut self) ->&mut Vec<Statement> {
        &mut self.statements
    }

    pub fn new(pos: (usize, usize), attributes: Vec<Attribute>, name: String, params: Vec<Param>,
            return_type: String, statements: Vec<Statement>) -> Self {
        Function {
            pos,
            attributes,
            name,
            params,
            return_type,
            statements,
        }
    }
}

impl<'a> FromPair<'a> for Function {
    fn from_pair<'b>(pair: Pair<'b, Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::method_decl);

        let mut attributes: Vec<Attribute> = vec![];
        let mut name = String::new();
        let mut params: Vec<Param> = vec![];
        let mut return_type = String::new();
        let mut statements: Vec<Statement> = vec![];
        let pos = pair.as_span().start_pos().line_col();

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::attribute_list => {
                    for attribute in pair.into_inner() {
                        attributes.push(Attribute::from_pair(attribute));
                    }
                },
                Rule::ident | Rule::operator => {
                    name.insert_str(0, pair.as_str());
                },
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

        Function {
            pos,
            attributes,
            name,
            params,
            return_type,
            statements
        }
    }

    fn get_pos(&self) -> (usize, usize) {
        self.pos
    }
}

impl FunctionCall {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_param_exprs(&mut self) -> &mut Vec<Expression> {
        &mut self.param_exprs
    }

    pub fn new(pos: (usize, usize), name: String, param_exprs: Vec<Expression>) -> Self {
        FunctionCall {
            pos,
            name,
            param_exprs,
        }
    }
}

impl<'a> FromPair<'a> for FunctionCall {
    fn from_pair<'b>(pair: Pair<'b, Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::method_call);

        let pos = pair.as_span().start_pos().line_col();
        let mut inner_pair = pair.into_inner();
        let name = String::from(inner_pair.next().unwrap().as_str());
        let mut param_exprs: Vec<Expression> = vec![];

        match inner_pair.next() {
            Some(pair) => {
                for expr in pair.into_inner() {
                    param_exprs.push(Expression::from_pair(expr));
                }
            },
            None => {}
        }

        FunctionCall {
            pos,
            name,
            param_exprs,
        }
    }

    fn get_pos(&self) -> (usize, usize) {
        self.pos
    }
}
