use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct MyNode;

#[godot_api]
impl INode for MyNode {
    fn init(_base: Base<Node>) -> Self {
        Self
    }
}

#[godot_api]
impl MyNode {}

