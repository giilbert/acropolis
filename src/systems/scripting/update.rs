use bevy_ecs::{prelude::*, query::QueryIter};

use crate::{components::Behavior, resources::scripting::ScriptingResource};

pub fn scripting_update_system(world: &mut World) {
    let scripting_resource = world.non_send_resource_mut::<ScriptingResource>();
    let mut behaviors = world.query::<&Behavior>();
    for behavior in behaviors.iter_mut(world) {}
}
