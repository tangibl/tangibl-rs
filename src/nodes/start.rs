use std::{borrow::Cow, rc::Rc};

use crate::nodes::{AstNode, FlowNode, Node};
use crate::Visitor;

#[derive(Default, Debug)]
pub struct Start {
    next: Option<Rc<Box<dyn FlowNode>>>,
}

impl Node for Start {
    fn name(&self) -> Cow<'_, str> {
        Cow::Borrowed("start")
    }
}

impl AstNode for Start {
    fn apply<V: Visitor + ?Sized>(&self, visitor: &mut V, args: V::Args) -> V::Args {
        visitor.visit_start(self, args)
    }
}
