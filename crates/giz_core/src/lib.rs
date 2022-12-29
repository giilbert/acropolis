mod application;
pub mod components;
mod plugin;
pub mod resources;
mod stage;
mod systems;

pub use application::Application;
use components::{Children, GlobalTransform, Name, Parent, Transform};
pub use plugin::Plugin;
pub use stage::Stage;

use bevy_ecs::prelude::Bundle;

#[derive(Bundle)]
pub struct DefaultBundle {
    pub name: Name,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub children: Children,
    pub parent: Parent,
}
