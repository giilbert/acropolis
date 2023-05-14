use acropolis_core::Application;
use acropolis_input::InputPlugin;
use acropolis_loader::LoaderPlugin;
use acropolis_math::MathPlugin;
use acropolis_render::RenderPlugin;
use acropolis_scripting::ScriptingPlugin;

fn main() {
    pretty_env_logger::init();

    let now = std::time::Instant::now();

    acropolis_build::create_dot_acropolis("test-world".into());
    let out = acropolis_build::build(acropolis_build::BuildParameters {
        project_root: "test-world".into(),
        behavior_paths: vec!["src/move.ts".into()],
    });
    // write to bundle.js
    println!("bundling took: {}ms", now.elapsed().as_millis());

    let mut app = Application::new()
        .with_plugin(LoaderPlugin)
        .with_plugin(RenderPlugin)
        .with_plugin(ScriptingPlugin)
        .with_plugin(MathPlugin)
        .with_plugin(InputPlugin);

    let mut test_world = std::env::current_dir().unwrap();
    test_world.push("test-world");
    acropolis_loader::load_from_file(&mut app, test_world, "test-world.json")
        .unwrap();

    app.run();
}
