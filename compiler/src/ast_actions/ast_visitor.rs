extern crate lang_parser;

use lang_parser::ast::*;

#[allow(dead_code)]
pub trait AstVisitor {
    fn visit_file(&mut self, n: &mut File) -> () {
        for entity in n.get_entities() {
            match entity {
                FirstClassEntity::Function(f) => {
                    self.visit_function(f);
                },
                FirstClassEntity::Class(c) => {
                    self.visit_class(c);
                },
            }
        }
    }

    fn visit_attribute(&mut self, n: &mut Attribute) -> () {}

    fn visit_function(&mut self, n: &mut Function) -> () {
        for attribute in n.get_attributes() {
            self.visit_attribute(attribute);
        }
        for param in n.get_params() {
            self.visit_param(param);
        }
        for stmt in n.get_statements() {
            self.visit_statement(stmt);
        }
    }

    fn visit_function_call(&mut self, n: &mut FunctionCall) -> () {
        for param_epxr in n.get_param_exprs() {
            self.visit_expression(param_epxr);
        }
    }

    fn visit_param(&mut self, n: &mut Param) -> () {}

    fn visit_operation(&mut self, n: &mut Operation) -> () {
        self.visit_expression(n.get_lval());
        self.visit_expression(n.get_rval());
    }

    fn visit_expression(&mut self, n: &mut Expression) -> () {
        match n {
            Expression::Operation(o) => {
                self.visit_operation(o);
            },
            Expression::Expr(q) => {
                self.visit_qualified_expression(q);
            },
        }
    }

    fn visit_qualified_expression(&mut self, n: &mut QualifiedExpression) -> () {
        for part in n.get_parts() {
            self.visit_qualified_expression_part(part);
        }
    }

    fn visit_qualified_expression_part(&mut self, n: &mut QualifiedExpressionPart) -> () {
        match n {
            QualifiedExpressionPart::MethodCall(f) => {
                self.visit_function_call(f);
            },
            QualifiedExpressionPart::Closure(c) => {
                self.visit_closure(c);
            },
            QualifiedExpressionPart::ParenExpr(e) => {
                self.visit_expression(e);
            },
            QualifiedExpressionPart::Identifier(i) => {
                self.visit_identifier(i);
            },
            QualifiedExpressionPart::Integer(i) => {
                self.visit_integer(i);
            },
            QualifiedExpressionPart::StringLitteral(s) => {
                self.visit_string_litteral(s);
            },
            QualifiedExpressionPart::Char(c) => {
                self.visit_char(c);
            },
            QualifiedExpressionPart::Boolean(b) => {
                self.visit_boolean(b);
            },
            _ => {}
        }
    }

    fn visit_statement(&mut self, n: &mut Statement) -> () {
        match n {
            Statement::ReturnStatement(e) => {
                self.visit_expression(e);
            },
            Statement::Declaration(d) => {
                self.visit_variable_declaration(d);
            },
            Statement::Affectation(a) => {
                self.visit_variable_affectation(a);
            },
            Statement::QualifiedExpression(q) => {
                self.visit_qualified_expression(q);
            },
        }
    }

    fn visit_closure(&mut self, n: &mut Closure) -> () {
        for param in n.get_params() {
            self.visit_param(param);
        }
        for stmt in n.get_statements() {
            self.visit_statement(stmt);
        }
    }

    fn visit_variable_declaration(&mut self, n: &mut VariableDeclaration) -> () {
        match n.get_value() {
            Some(e) => {
                self.visit_expression(e);
            },
            None => {}
        }
    }

    fn visit_variable_affectation(&mut self, n: &mut VariableAffectation) -> () {
        self.visit_qualified_expression(n.get_receiver());
        self.visit_expression(n.get_value());
    }

    fn visit_class(&mut self, n: &mut Class) -> () {
        for attribute in n.get_attributes() {
            self.visit_attribute(attribute);
        }
        for member in n.get_members() {
            self.visit_class_member(member);
        }
    }

    fn visit_class_member(&mut self, n: &mut ClassMember) -> () {
        match n {
            ClassMember::Field(f) => {
                self.visit_field(f);
            },
            ClassMember::Method(f) => {
                self.visit_function(f);
            },
            ClassMember::Block(b) => {
                self.visit_block(b);
            },
        }
    }

    fn visit_block(&mut self, n: &mut Block) ->() {
        for member in n.get_members() {
            self.visit_class_member(member);
        }
    }

    fn visit_field(&mut self, n: &mut Field) -> () {
        for attribute in n.get_attributes() {
            self.visit_attribute(attribute);
        }
    }

    fn visit_identifier(&mut self, n: &mut Identifier) -> () {}
    fn visit_integer(&mut self, n: &mut Integer) -> () {}
    fn visit_string_litteral(&mut self, n: &mut StringLitteral) -> () {}
    fn visit_boolean(&mut self, n: &mut Boolean) -> () {}
    fn visit_char(&mut self, n: &mut Char) -> () {}
}