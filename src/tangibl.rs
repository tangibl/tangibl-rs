use crate::nodes::{FlowNode, Start};
use std::rc::{Rc, Weak};
use topcodes::TopCode;

pub struct Tangibl {}

impl Tangibl {
    /// TODO
    pub fn parse(topcodes: Vec<TopCode>) -> Option<()> {
        None
    }

    /// TODO
    pub fn builder() -> TangiblStartBuilder {
        TangiblStartBuilder::default()
    }
}

#[derive(Default)]
pub struct TangiblStartBuilder {
    /// The main flow for the program. As if it were the 'main' function of many common languages.
    flow: TangiblFlowBuilder,
}

impl TangiblStartBuilder {
    pub fn build(self) -> Start {
        let start = Start::default();
        start
    }
}

#[derive(Default)]
pub struct TangiblFlowBuilder {
    first_node: Option<Rc<Box<dyn FlowNode>>>,
    last_node: Weak<Box<dyn FlowNode>>,
}

impl TangiblFlowBuilder {
    pub fn build(self) -> Option<Rc<Box<dyn FlowNode>>> {
        self.first_node
    }
}
