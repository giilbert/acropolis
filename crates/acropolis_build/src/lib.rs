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

#[cfg(not(target_arch = "wasm32"))]
pub fn pack_project(base_path: PathBuf) {
    use std::{ffi::OsStr, fs::File, io::BufReader};
    use walkdir::WalkDir;
    use zip::write::{FileOptions, ZipWriter};

    let mut zip = ZipWriter::new(File::create("web/test-project.zip").unwrap());

    for entry in WalkDir::new(base_path.clone()) {
        if let Ok(entry) = entry {
            let directory_name = entry
                .path()
                .strip_prefix(&base_path)
                .unwrap()
                .iter()
                .next()
                .unwrap_or(OsStr::new(""))
                .to_string_lossy();

            if directory_name == ".acropolis"
                || directory_name == "node_modules"
            {
                continue;
            }

            if entry.file_type().is_file() {
                // add the file to the zip archive
                zip.start_file(
                    entry
                        .path()
                        .strip_prefix(&base_path)
                        .unwrap()
                        .to_string_lossy(),
                    FileOptions::default(),
                )
                .unwrap();

                let buffer = File::open(entry.path()).unwrap();
                std::io::copy(&mut BufReader::new(buffer), &mut zip).unwrap();
            }
        }
    }

    zip.start_file("bundle.js", FileOptions::default()).unwrap();
    let buffer = File::open(base_path.join(".acropolis/out.js")).unwrap();
    std::io::copy(&mut BufReader::new(buffer), &mut zip).unwrap();

    zip.finish().unwrap();
}
