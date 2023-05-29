use std::path::PathBuf;

use acropolis_core::Application;
use acropolis_input::InputPlugin;
use acropolis_loader::LoaderPlugin;
use acropolis_math::MathPlugin;
use acropolis_render::RenderPlugin;
use acropolis_scripting::ScriptingPlugin;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn run(world: String) {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let mut app = Application::new()
        .with_plugin(LoaderPlugin)
        .with_plugin(RenderPlugin)
        .with_plugin(ScriptingPlugin)
        .with_plugin(MathPlugin)
        .with_plugin(InputPlugin);

    acropolis_loader::load_from_file(&mut app, "".into(), "test-world.json")
        .unwrap();

    app.run();
}
