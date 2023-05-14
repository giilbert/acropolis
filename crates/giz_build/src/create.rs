use std::path::PathBuf;

use include_dir::{include_dir, Dir};

const GIZ_LIBRARY: Dir<'static> = include_dir!("crates/giz_scripting/js");
const BUILD: Dir<'static> = include_dir!("crates/giz_build/js");

pub fn create_dot_giz(path: PathBuf) {
    use std::fs::{create_dir, remove_dir_all};
    // create the .giz directory from scratch
    let _ = remove_dir_all(&path.join(".giz"));
    create_dir(&path.join(".giz")).unwrap();

    BUILD.extract(path.join(".giz")).unwrap();
    GIZ_LIBRARY.extract(path.join(".giz")).unwrap();
}
