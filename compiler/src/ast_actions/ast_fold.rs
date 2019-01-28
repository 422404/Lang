extern crate lang_parser;

use lang_parser::ast::*;

pub trait AstFold {
    fn fold_file(&mut self, n: File) -> File { n }
    fn fold_attribute(&mut self, n: Attribute) -> Attribute { n }
    fn fold_function(&mut self, n: Function) -> Function { n }
    fn fold_function_call(&mut self, n: FunctionCall) -> FunctionCall { n }
    fn fold_param(&mut self, n: Param) -> Param { n }
    fn visit_operation(&mut self, n: Operation) -> Operation { n }
    fn visit_expression(&mut self, n: Expression) -> Expression { n }
    fn fold_qualified_expression(&mut self, n: QualifiedExpression) -> QualifiedExpression { n }
    fn fold_qualified_expression_part(&mut self, n: QualifiedExpressionPart) -> QualifiedExpressionPart { n }
    fn fold_statement(&mut self, n: Statement) -> Statement { n }
    fn fold_closure(&mut self, n: Closure) -> Closure { n }
    fn fold_variable_declaration(&mut self, n: VariableDeclaration) -> VariableDeclaration { n }
    fn fold_variable_affectation(&mut self, n: VariableAffectation) -> VariableAffectation { n }
    fn fold_class(&mut self, n: Class) -> Class { n }
    fn fold_class_member(&mut self, n: ClassMember) -> ClassMember { n }
    fn fold_block(&mut self, n: Block) -> Block { n }
    fn fold_field(&mut self, n: Field) -> Field { n }
    fn fold_identifier(&mut self, n: Identifier) -> Identifier { n }
    fn fold_integer(&mut self, n: Integer) -> Integer { n }
    fn fold_string_litteral(&mut self, n: StringLitteral) -> StringLitteral { n }
    fn fold_boolean(&mut self, n: Boolean) -> Boolean { n }
    fn fold_char(&mut self, n: Char) -> Char { n }
}
