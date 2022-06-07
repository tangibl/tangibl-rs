use serde_json::{Map, Value as JsValue};

use crate::{
    ast::{
        BooleanMethod, BooleanMethodKind, Command, Condition, Conditional, ConditionalKind, Flow,
        FlowKind, IntegerMethod, IntegerMethodKind, Start, Value,
    },
    Visitor,
};

const NAME: &str = "name";
const NEXT: &str = "next";
const ALTERNATE: &str = "alternate";
const CONDITION: &str = "condition";
const BODY: &str = "body";
const VALUE: &str = "value";

pub struct JsonPrinter {}

impl JsonPrinter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn print(&mut self, start: &Start) -> String {
        self.visit_start(start).to_string()
    }
}

impl Visitor for JsonPrinter {
    type Result = JsValue;

    fn visit_start(&mut self, start: &Start) -> Self::Result {
        let mut map = Map::new();
        map.insert(NAME.into(), JsValue::String("start".into()));

        if let Some(flow) = &start.next {
            let next = self.visit_flow(flow);
            if next.is_object() {
                map.insert(NEXT.into(), next);
            }
        }

        JsValue::Object(map)
    }

    fn visit_command(&mut self, command: &Command) -> Self::Result {
        let mut map = Map::new();
        let name = match &command {
            Command::Shoot => "shoot",
            Command::MoveForwards => "moveForwards",
            Command::MoveBackwards => "moveBackwards",
            Command::TurnLeft => "turnLeft",
            Command::TurnRight => "turnRight",
        };
        map.insert(NAME.into(), name.into());
        JsValue::Object(map)
    }

    fn visit_conditional(&mut self, conditional: &Conditional) -> Self::Result {
        let mut map = Map::new();
        let name = match &conditional.kind {
            ConditionalKind::Blocked => "blocked",
        };
        map.insert(NAME.into(), name.into());
        if let Some(alternate_flow) = &conditional.alternate {
            let alternate = self.visit_flow(alternate_flow);
            if alternate.is_object() {
                map.insert(ALTERNATE.into(), alternate);
            }
        }
        JsValue::Object(map)
    }

    fn visit_boolean_method(&mut self, boolean_method: &BooleanMethod) -> Self::Result {
        let mut map = Map::new();
        let name = match &boolean_method.kind {
            BooleanMethodKind::While => "while",
        };
        map.insert(NAME.into(), name.into());
        if let Some(condition) = &boolean_method.condition {
            let condition = match condition {
                Condition::IsBlocked => "isBlocked",
                Condition::IsPathClear => "isPathClear",
            };
            map.insert(CONDITION.into(), condition.into());
        }
        if let Some(body_flow) = &boolean_method.body {
            let body = self.visit_flow(body_flow);
            if body.is_object() {
                map.insert(BODY.into(), body);
            }
        }
        JsValue::Object(map)
    }

    fn visit_integer_method(&mut self, integer_method: &IntegerMethod) -> Self::Result {
        let mut map = Map::new();
        let name = match &integer_method.kind {
            IntegerMethodKind::Repeat => "repeat",
        };
        map.insert(NAME.into(), name.into());
        if let Some(value) = &integer_method.value {
            let value = match value {
                Value::One => "1",
                Value::Two => "2",
                Value::Three => "3",
                Value::Four => "4",
                Value::Five => "5",
                Value::Six => "6",
                Value::Seven => "7",
                Value::Eight => "8",
                Value::Infinity => "Infinity",
            };
            map.insert(VALUE.into(), value.into());
        }
        if let Some(body_flow) = &integer_method.body {
            let body = self.visit_flow(body_flow);
            if body.is_object() {
                map.insert(BODY.into(), body);
            }
        }
        JsValue::Object(map)
    }

    fn visit_flow(&mut self, flow: &Flow) -> Self::Result {
        let mut node = match &flow.kind {
            FlowKind::Command(command) => self.visit_command(&command),
            FlowKind::BooleanMethod(boolean_method) => self.visit_boolean_method(&boolean_method),
            FlowKind::IntegerMethod(integer_method) => self.visit_integer_method(&integer_method),
            FlowKind::Conditional(conditional) => self.visit_conditional(&conditional),
        };
        if let Some(next_flow) = &flow.next {
            if let Some(node_mut) = node.as_object_mut() {
                let next = self.visit_flow(next_flow);
                if next.is_object() {
                    node_mut.insert("next".into(), next);
                }
            }
        }
        node
    }
}
