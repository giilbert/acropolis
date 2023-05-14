mod create;

pub use create::create_dot_acropolis;
use std::{fs, path::PathBuf};

pub struct BuildParameters {
    pub project_root: PathBuf,
    pub behavior_paths: Vec<PathBuf>,
}

pub struct BuildOutput {}

pub fn build(parameters: BuildParameters) -> BuildOutput {
    let dot_acropolis = parameters.project_root.join(".acropolis");

    let files_js_path = dot_acropolis.join("files.js");
    let files_js = create_files_js(&parameters.behavior_paths);
    fs::write(files_js_path, &files_js).unwrap();

    println!("{}", files_js);

    // spawn node build.mjs
    let mut build_command = std::process::Command::new("node");
    let mut child = build_command
        .arg("build.mjs")
        .current_dir(dot_acropolis)
        .spawn()
        .unwrap();

    child.wait().unwrap();

    BuildOutput {}
}

fn create_files_js(files: &[PathBuf]) -> String {
    let mut imports = String::new();
    let mut export = "export const files = {\n".to_string();

    for (i, file) in files.iter().enumerate() {
        let path = file.display();

        imports += &format!("import * as FILE_{i} from \"../{path}\";\n");
        export += &format!("  \"{path}\": FILE_{i},\n");
    }

    export += "};";

    imports + &export
}
