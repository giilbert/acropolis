use acropolis_core::Plugin;
use acropolis_render::StateResource;
use acropolis_scripting::{serde_json, ScriptingExtensions, SCRIPTING_WORLD};
#[cfg(not(target_arch = "wasm32"))]
use deno_core::Extension;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = __ACROPOLIS__)]
    fn is_key_down(key: String) -> bool;
}

#[acropolis_scripting::glued_function]
pub fn op_get_key_down(key: String) -> bool {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            return is_key_down(key);
        }
    }

    let world = unsafe { &mut *SCRIPTING_WORLD.unwrap() };
    let state = world.resource::<StateResource>().lock();
    state.keys.contains(&serde_json::from_str(&key).unwrap())
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&mut self, app: &mut acropolis_core::Application) {
        let mut extensions = app.world.resource_mut::<ScriptingExtensions>();

        #[cfg(not(target_arch = "wasm32"))]
        extensions.add_extension(
            Extension::builder().ops(vec![op_get_key_down::decl()]),
        )
    }
}
