use pest::iterators::Pair;
use super::super::parser::Rule;
use std::any::Any;

pub trait FromPair<'a> {
    fn from_pair<'b>(pair: Pair<'b, Rule>) -> Self;
    fn get_pos(&self) -> (usize, usize);
}

/********** unused *****************/
pub trait AstNode: ToAny {
    fn node_type(&self) -> AstNodeType;
}

pub trait ToAny {
    fn to_any(&mut self) -> &mut dyn Any;
}

impl<T: 'static + AstNode> ToAny for T  where T: Sized {
    fn to_any(&mut self) -> &mut dyn Any {
        self
    }
}

pub enum AstNodeType {}
