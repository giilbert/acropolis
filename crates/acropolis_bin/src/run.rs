use std::path::PathBuf;

use acropolis_core::Application;
use acropolis_input::InputPlugin;
use acropolis_loader::LoaderPlugin;
use acropolis_math::MathPlugin;
use acropolis_render::RenderPlugin;
use acropolis_scripting::ScriptingPlugin;
use walkdir::DirEntry;

#[derive(serde::Deserialize)]
struct AcropolisJson {
    main: PathBuf,
}

fn filter_behavior_files(entry: &DirEntry) -> bool {
    if entry.file_type().is_dir() {
        return false;
    }

    let path = entry.path().clone().strip_prefix(".").unwrap();

    // if it starts with a .acropolis or node_modules
    if let Some(name) = path.to_str() {
        if name.starts_with(".acropolis") || name.starts_with("node_modules") {
            return false;
        }
    }

    if let Some(ext) = path.extension() {
        if ext == "ts" || ext == "js" {
            return true;
        }
    }

    return false;
}

pub fn command() {
    let behavior_paths = walkdir::WalkDir::new(".")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(filter_behavior_files)
        .map(|e| {
            e.path()
                .to_path_buf()
                .strip_prefix(".")
                .unwrap()
                .to_path_buf()
        })
        .collect::<Vec<_>>();

    acropolis_build::create_dot_acropolis(".".into());
    acropolis_build::build(acropolis_build::BuildParameters {
        project_root: ".".into(),
        behavior_paths,
    });

    let mut app = Application::new()
        .with_plugin(LoaderPlugin)
        .with_plugin(RenderPlugin)
        .with_plugin(ScriptingPlugin)
        .with_plugin(MathPlugin)
        .with_plugin(InputPlugin);

    let acropolis_json: AcropolisJson = serde_json::from_reader(
        std::fs::File::open("./acropolis.json").unwrap(),
    )
    .unwrap();

    acropolis_loader::load_from_file(&mut app, ".".into(), acropolis_json.main)
        .unwrap();

    app.run();
}
