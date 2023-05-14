use bevy_ecs::prelude::*;

use crate::resources::ScriptingResource;

pub fn scripting_update_system(world: &mut World) {
    let now = std::time::Instant::now();

    let mut scripting_resource =
        world.non_send_resource_mut::<ScriptingResource>();

    scripting_resource
        .runtime
        .execute_script("<acropolis::scripting::update>", "__acropolis__.runOnce()")
        .expect("error updating scripting");

    let elapsed = now.elapsed();
    log::trace!("scripting update: {:?}", elapsed);
}
