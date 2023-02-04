mod components;
mod plugin;
mod systems;

pub use components::*;
pub use plugin::MathPlugin;

// TODO: find permenant home
use bevy_ecs::prelude::Bundle;
use giz_core::components::Name;

#[derive(Bundle)]
pub struct DefaultBundle {
    pub transform: components::Transform,
    pub global_transform: components::GlobalTransform,
    pub name: Name,
    pub parent: components::Parent,
    pub children: components::Children,
}
