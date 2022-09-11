use bevy_ecs::{prelude::*, query::QueryIter};

use crate::{components::Behavior, resources::scripting::ScriptingResource};

pub fn scripting_update_system(world: &mut World) {
    let mut scripting_resource =
        world.non_send_resource_mut::<ScriptingResource>();
    scripting_resource
        .runtime
        .execute_script("<giz::scripting::update>", "runOnce()")
        .expect("error updating scripting");
}
