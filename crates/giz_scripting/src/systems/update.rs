use bevy_ecs::prelude::*;

use crate::resources::ScriptingResource;

pub fn scripting_update_system(world: &mut World) {
    let mut scripting_resource =
        world.non_send_resource_mut::<ScriptingResource>();
    scripting_resource
        .runtime
        .execute_script("<giz::scripting::update>", "runOnce()")
        .expect("error updating scripting");
}
