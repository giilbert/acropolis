use bevy_ecs::prelude::Bundle;

use super::{Children, GlobalTransform, Name, Parent, Transform};

#[derive(Bundle)]
pub struct DefaultBundle {
    pub name: Name,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub children: Children,
    pub parent: Parent,
}
