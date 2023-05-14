use giz_core::Application;
use giz_input::InputPlugin;
use giz_loader::LoaderPlugin;
use giz_math::MathPlugin;
use giz_render::RenderPlugin;
use giz_scripting::ScriptingPlugin;

fn main() {
    pretty_env_logger::init();

    let now = std::time::Instant::now();

    giz_build::create_dot_giz("test-world".into());
    let out = giz_build::build(giz_build::BuildParameters {
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
    giz_loader::load_from_file(&mut app, test_world, "test-world.json")
        .unwrap();

    app.run();
}
