use std::path::PathBuf;

use giz_core::Application;
use giz_input::InputPlugin;
use giz_math::MathPlugin;
use giz_render::RenderPlugin;
use giz_scripting::ScriptingPlugin;

fn main() {
    pretty_env_logger::init();

    let mut app = Application::new()
        .with_plugin(RenderPlugin)
        .with_plugin(ScriptingPlugin)
        .with_plugin(MathPlugin)
        .with_plugin(InputPlugin);

    let mut test_world = std::env::current_dir().unwrap();
    test_world.push("test-world");
    // get the cwd
    giz_loader::load_from_file(&mut app, test_world, "test-world.json")
        .unwrap();

    app.run();
}
