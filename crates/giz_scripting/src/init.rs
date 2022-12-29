use crate::resources::{ScriptingExtensions, ScriptingResource};
use bevy_ecs::prelude::*;

use crate::components::Behavior;

const BOOTSTRAP_SOURCE: &str = include_str!("js/bootstrap.js");

pub fn create_runtime(world: &mut World) {
    let mut extensions_resource = world.resource_mut::<ScriptingExtensions>();
    let extensions = extensions_resource.extensions.take().unwrap();
    world.insert_resource(ScriptingResource::new(extensions));
}

pub fn init_scripting(
    mut scripting: NonSendMut<ScriptingResource>,
    mut query: Query<(&mut Behavior, Entity)>,
) {
    let runtime = &mut scripting.runtime;
    runtime
        .execute_script("giz-bootstrap", BOOTSTRAP_SOURCE)
        .expect("scripting failed to initialize");

    for (mut behavior, entity) in &mut query {
        behavior.run(runtime, entity);
    }
}
