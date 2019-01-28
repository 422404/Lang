extern crate lang_parser;

use lang_parser::ast::*;

#[allow(dead_code)]
pub trait AstVisitor {
    fn visit_file(&mut self, n: File) -> () {
        for entity in n.get_entities() {
            match entity.clone() {
                FirstClassEntity::Function(f) => {
                    self.visit_function(f);
                },
                FirstClassEntity::Class(c) => {
                    self.visit_class(c);
                },
            }
        }
    }

    fn visit_attribute(&mut self, n: Attribute) -> () {}

    fn visit_function(&mut self, n: Function) -> () {
        for attribute in n.get_attributes() {
            self.visit_attribute(attribute.clone());
        }
        for param in n.get_params() {
            self.visit_param(param.clone());
        }
        for stmt in n.get_statements() {
            self.visit_statement(stmt.clone());
        }
    }

    fn visit_function_call(&mut self, n: FunctionCall) -> () {
        for param_epxr in n.get_param_exprs() {
            self.visit_expression(param_epxr.clone());
        }
    }

    fn visit_param(&mut self, n: Param) -> () {}

    fn visit_operation(&mut self, n: Operation) -> () {
        self.visit_expression(n.get_lval().clone());
        self.visit_expression(n.get_rval().clone());
    }

    fn visit_expression(&mut self, n: Expression) -> () {
        match n.clone() {
            Expression::Operation(o) => {
                self.visit_operation(o);
            },
            Expression::Expr(q) => {
                self.visit_qualified_expression(q);
            },
        }
    }

    fn visit_qualified_expression(&mut self, n: QualifiedExpression) -> () {
        for part in n.get_parts().clone() {
            self.visit_qualified_expression_part(part);
        }
    }

    fn visit_qualified_expression_part(&mut self, n: QualifiedExpressionPart) -> () {
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

    fn visit_statement(&mut self, n: Statement) -> () {
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

    fn visit_closure(&mut self, n: Closure) -> () {
        for param in n.get_params().clone() {
            self.visit_param(param);
        }
        for stmt in n.get_statements().clone() {
            self.visit_statement(stmt);
        }
    }

    fn visit_variable_declaration(&mut self, n: VariableDeclaration) -> () {
        match n.get_value().clone() {
            Some(e) => {
                self.visit_expression(e);
            },
            None => {}
        }
    }

    fn visit_variable_affectation(&mut self, n: VariableAffectation) -> () {
        self.visit_qualified_expression(n.get_receiver().clone());
        self.visit_expression(n.get_value().clone());
    }

    fn visit_class(&mut self, n: Class) -> () {
        for attribute in n.get_attributes().clone() {
            self.visit_attribute(attribute);
        }
        for member in n.get_members().clone() {
            self.visit_class_member(member);
        }
    }

    fn visit_class_member(&mut self, n: ClassMember) -> () {
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

    fn visit_block(&mut self, n: Block) ->() {
        for member in n.get_members().clone() {
            self.visit_class_member(member);
        }
    }

    fn visit_field(&mut self, n: Field) -> () {
        for attribute in n.get_attributes().clone() {
            self.visit_attribute(attribute);
        }
    }

    fn visit_identifier(&mut self, n: Identifier) -> () {}
    fn visit_integer(&mut self, n: Integer) -> () {}
    fn visit_string_litteral(&mut self, n: StringLitteral) -> () {}
    fn visit_boolean(&mut self, n: Boolean) -> () {}
    fn visit_char(&mut self, n: Char) -> () {}
}