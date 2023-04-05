mod ecma;

use std::path::PathBuf;

use ecma::bundler::EcmaBundler;

#[derive(Clone, Debug)]
pub struct BuildParameters {
    pub base_path: PathBuf,
    pub behavior_paths: Vec<PathBuf>,
}

pub struct BuildOutput {
    pub code: String,
}

pub fn build(params: BuildParameters) -> BuildOutput {
    let mut bundler = EcmaBundler::new(params.clone());

    bundler.add_file(&params.behavior_paths[0]);

    BuildOutput {
        code: bundler.generate_code(),
    }
}
