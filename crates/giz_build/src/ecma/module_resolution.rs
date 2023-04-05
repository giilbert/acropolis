use std::{
    cell::RefCell,
    path::{Path, PathBuf},
    rc::Rc,
};

use swc_core::ecma::{
    ast::{ImportDecl, Program},
    visit::{as_folder, Fold, Folder, VisitMut},
};

pub struct ModulePathTransformer {
    base_path: PathBuf,
    file_path: PathBuf,
    additional_files: Rc<RefCell<Option<Vec<PathBuf>>>>,
}

impl VisitMut for ModulePathTransformer {
    fn visit_mut_import_decl(&mut self, node: &mut ImportDecl) {
        let module_path = PathBuf::from(node.src.value.to_string());
        if module_path.starts_with("@giz/") {
            return;
        }

        let full_path =
            join_path(&self.file_path.parent().unwrap(), &module_path);

        // look for js or ts files with the same name

        let js_path = self.base_path.join(&full_path).with_extension("js");
        let ts_path = self.base_path.join(&full_path).with_extension("ts");

        let path = if js_path.exists() {
            full_path.with_extension("js")
        } else if ts_path.exists() {
            full_path.with_extension("ts")
        } else {
            // TODO: better error handling
            panic!("Could not find module: {:?}", full_path);
        };

        node.src.value = path.to_string_lossy().into();
        self.additional_files
            .borrow_mut()
            .as_mut()
            .unwrap()
            .push(path);

        println!("full_path: {:?}", full_path);
    }
}

pub fn create_module_path_transformer(
    base_path: &PathBuf,
    file_path: &PathBuf,
    additional_files: Rc<RefCell<Option<Vec<PathBuf>>>>,
) -> Folder<ModulePathTransformer> {
    as_folder(ModulePathTransformer {
        base_path: base_path.clone(),
        file_path: file_path.clone(),
        additional_files,
    })
}

fn join_path(base: &Path, path: &Path) -> PathBuf {
    let mut base = base.to_path_buf();
    let mut path = path.to_path_buf();

    if path.is_absolute() {
        return path;
    }

    if path.starts_with("./") {
        path = path.strip_prefix("./").unwrap().to_path_buf();
    }

    if path.starts_with("../") {
        let mut path_parts = path.components().collect::<Vec<_>>();
        let mut base_parts = base.components().collect::<Vec<_>>();

        while path_parts.len() > 0
            && path_parts[0] == std::path::Component::ParentDir
        {
            path_parts.remove(0);
            base_parts.pop();
        }

        base = base_parts.into_iter().collect();
        path = path_parts.into_iter().collect();
    }

    base.push(path);
    base
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn join_path_test() {
        assert_eq!(
            join_path(Path::new("home"), Path::new("file.txt")),
            PathBuf::from("home/file.txt")
        );

        assert_eq!(
            join_path(Path::new("home"), Path::new("./file.txt")),
            PathBuf::from("home/file.txt")
        );

        assert_eq!(
            join_path(Path::new("home/a/b"), Path::new("../file.txt")),
            PathBuf::from("home/a/file.txt")
        );

        assert_eq!(
            join_path(Path::new("home/a/b"), Path::new("../../file.txt")),
            PathBuf::from("home/file.txt")
        );

        assert_eq!(
            join_path(Path::new("home/a/b"), Path::new("../../c/file.txt")),
            PathBuf::from("home/c/file.txt")
        );
    }
}
