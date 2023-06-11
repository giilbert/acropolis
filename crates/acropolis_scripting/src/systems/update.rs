use bevy_ecs::prelude::*;
use wasm_bindgen::prelude::*;

use crate::resources::ScriptingResource;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = __ACROPOLIS__)]
    fn run_once();
}

pub fn scripting_update_system(world: &mut World) {
    let now = std::time::Instant::now();

    let mut scripting_resource =
        world.non_send_resource_mut::<ScriptingResource>();

    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            run_once();
        } else {
            scripting_resource
                .runtime
                .execute_script(
                    "<acropolis::scripting::update>",
                    "__ACROPOLIS__.scripting.runOnce()",
                )
                .expect("error updating scripting");
        }
    }

    let elapsed = now.elapsed();
    log::info!("scripting update: {:?}", elapsed);
}
