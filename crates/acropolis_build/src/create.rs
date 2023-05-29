use std::path::PathBuf;

use include_dir::{include_dir, Dir};

cfg_if::cfg_if! {
    if #[cfg(not(target_os = "wasm32"))] {
        const ACROPOLIS_LIBRARY: Dir<'static> =
            include_dir!("$CARGO_MANIFEST_DIR/../../crates/acropolis_scripting/js");

        const BUILD: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/../../crates/acropolis_build/js");
    }
}

pub fn create_dot_acropolis(path: PathBuf) {
    use std::fs::{create_dir, remove_dir_all};
    // create the .acropolis directory from scratch
    let _ = remove_dir_all(&path.join(".acropolis"));
    create_dir(&path.join(".acropolis")).unwrap();

    BUILD.extract(path.join(".acropolis")).unwrap();
    ACROPOLIS_LIBRARY.extract(path.join(".acropolis")).unwrap();
}
