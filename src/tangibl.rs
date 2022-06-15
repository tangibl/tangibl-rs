use crate::{
    ast::{
        BooleanMethod, BooleanMethodKind, Command, Condition, Conditional, ConditionalKind, Flow,
        FlowKind, IntegerMethod, IntegerMethodKind, Start, Value,
    },
    parser::Parser,
};
use std::collections::VecDeque;
use topcodes::TopCode;

pub fn parse(topcodes: Vec<TopCode>) -> Option<Start> {
    Parser::new(&topcodes).parse()
}

pub fn start() -> TangiblStartBuilder {
    TangiblStartBuilder::default()
}

pub fn flow() -> TangiblFlowBuilder {
    TangiblFlowBuilder::default()
}

#[derive(Default, Debug)]
pub struct TangiblStartBuilder {
    /// The main flow for the program. As if it were the 'main' function of many common languages.
    flow_builder: TangiblFlowBuilder,
}

impl TangiblStartBuilder {
    pub fn with_command(&mut self, command: Command) -> &mut Self {
        self.flow_builder.with_command(command);
        self
    }

    pub fn with_conditional(
        &mut self,
        conditional_kind: ConditionalKind,
        alternate: Option<Flow>,
    ) -> &mut Self {
        self.flow_builder.with_conditional(Conditional {
            kind: conditional_kind,
            alternate: alternate.map(Box::new),
        });
        self
    }

    pub fn with_boolean_method(
        &mut self,
        boolean_method_kind: BooleanMethodKind,
        condition: Option<Condition>,
        body: Option<Flow>,
    ) -> &mut Self {
        self.flow_builder
            .with_boolean_method(boolean_method_kind, condition, body);
        self
    }

    pub fn with_integer_method(
        &mut self,
        integer_method_kind: IntegerMethodKind,
        value: Option<Value>,
        body: Option<Flow>,
    ) -> &mut Self {
        self.flow_builder
            .with_integer_method(integer_method_kind, value, body);
        self
    }

    pub fn build(&mut self) -> Start {
        Start {
            next: self.flow_builder.build(),
        }
    }
}

#[derive(Default, Debug)]
pub struct TangiblFlowBuilder {
    nodes: VecDeque<Flow>,
}

impl TangiblFlowBuilder {
    pub fn with_command(&mut self, command: Command) -> &mut Self {
        self.with_flow(Flow::new(FlowKind::Command(command)));
        self
    }

    pub fn with_conditional(&mut self, conditional: Conditional) -> &mut Self {
        self.with_flow(Flow::new(FlowKind::Conditional(conditional)));
        self
    }

    pub fn with_boolean_method(
        &mut self,
        boolean_method_kind: BooleanMethodKind,
        condition: Option<Condition>,
        body: Option<Flow>,
    ) -> &mut Self {
        self.with_flow(Flow::new(FlowKind::BooleanMethod(BooleanMethod {
            kind: boolean_method_kind,
            condition,
            body: body.map(Box::new),
        })));
        self
    }

    pub fn with_integer_method(
        &mut self,
        integer_method_kind: IntegerMethodKind,
        value: Option<Value>,
        body: Option<Flow>,
    ) -> &mut Self {
        self.with_flow(Flow::new(FlowKind::IntegerMethod(IntegerMethod {
            kind: integer_method_kind,
            value,
            body: body.map(Box::new),
        })));
        self
    }

    pub fn build(&mut self) -> Option<Flow> {
        let mut current = None;
        while let Some(mut node) = self.nodes.pop_back() {
            match current {
                None => current = Some(node),
                Some(_) => {
                    node.next = current.map(Box::new);
                    current = Some(node);
                }
            }
        }
        current
    }

    fn with_flow(&mut self, flow: Flow) -> &mut Self {
        self.nodes.push_back(flow);
        self
    }
}
