use std::{cell::RefCell, path::PathBuf, rc::Rc, sync::Arc};

use swc::{
    config::{Config, JscConfig, ModuleConfig, Options},
    Compiler,
};
use swc_common::{
    comments::SingleThreadedComments,
    errors::{ColorConfig, Handler},
    FileName, Globals, SourceMap, GLOBALS,
};
use swc_core::ecma::{
    ast::EsVersion,
    parser::{Syntax, TsConfig},
    transforms::{
        base::pass::noop, module::common_js::Config as CommonJsConfig,
    },
};

use crate::BuildParameters;

use super::module_resolution::create_module_path_transformer;

#[derive(Debug, Clone)]
pub struct TranspiledCode {
    pub file_name: FileName,
    pub code: String,
    pub additional_files: Vec<PathBuf>,
    // TODO: sourcemaps
}

impl TranspiledCode {
    pub fn load(
        parameters: &BuildParameters,
        path: &PathBuf,
    ) -> TranspiledCode {
        GLOBALS.set(&Globals::new(), || {
            let cm = Arc::<SourceMap>::default();
            let handler = Handler::with_tty_emitter(
                ColorConfig::Auto,
                true,
                false,
                Some(cm.clone()),
            );
            let c = Compiler::new(cm.clone());

            let fm = cm.new_source_file(
                FileName::Real(path.clone()),
                std::fs::read_to_string(parameters.base_path.join(path))
                    .unwrap(),
            );

            let additional_files = Rc::new(RefCell::new(Some(vec![])));

            let output = c
                .process_js_with_custom_pass(
                    fm,
                    None,
                    &handler,
                    &Options {
                        config: Config {
                            module: Some(ModuleConfig::CommonJs(
                                CommonJsConfig {
                                    ..Default::default()
                                },
                            )),
                            jsc: JscConfig {
                                target: Some(EsVersion::Es2020),
                                syntax: Some(Syntax::Typescript(TsConfig {
                                    ..Default::default()
                                })),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    SingleThreadedComments::default(),
                    |_| {
                        create_module_path_transformer(
                            &parameters.base_path,
                            path,
                            additional_files.clone(),
                        )
                    },
                    |_| noop(),
                )
                .unwrap();

            let additional_files =
                additional_files.borrow_mut().take().unwrap();

            TranspiledCode {
                file_name: FileName::Real(path.clone()),
                code: output.code,
                additional_files,
            }
        })
    }

    pub fn output_module_declaration_code(&self) -> String {
        format!(
            r#"
"{file_name}": (require, exports) => {{
    {code}
}},
"#,
            file_name = self.file_name,
            code = self.code
        )
    }
}
