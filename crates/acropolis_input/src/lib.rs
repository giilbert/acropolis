use deno_core::{op, serde_json, Extension};
use acropolis_core::Plugin;
use acropolis_render::StateResource;
use acropolis_scripting::{ScriptingExtensions, SCRIPTING_WORLD};

#[op]
fn op_get_key_down(key: String) -> bool {
    let world = unsafe { &mut *SCRIPTING_WORLD.unwrap() };
    let state = world.resource::<StateResource>().lock();
    state.keys.contains(&serde_json::from_str(&key).unwrap())
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&mut self, app: &mut acropolis_core::Application) {
        let mut extensions = app.world.resource_mut::<ScriptingExtensions>();
        extensions.add_extension(
            Extension::builder().ops(vec![op_get_key_down::decl()]),
        )
    }
}
