use bevy_ecs::prelude::*;

use crate::{components::Behavior, resources::scripting::ScriptingResource};

const BOOTSTRAP_SOURCE: &str = include_str!("js/bootstrap.js");

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
