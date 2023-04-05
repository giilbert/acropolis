use std::{collections::HashMap, path::PathBuf};

use swc_common::FileName;

use crate::BuildParameters;

use super::transpile::TranspiledCode;

pub struct EcmaBundler {
    pub transpiled_modules: HashMap<FileName, TranspiledCode>,
    pub parameters: BuildParameters,
}

impl EcmaBundler {
    pub fn new(parameters: BuildParameters) -> EcmaBundler {
        EcmaBundler {
            transpiled_modules: HashMap::new(),
            parameters,
        }
    }

    pub fn add_file(&mut self, path: &PathBuf) {
        let loaded = TranspiledCode::load(&self.parameters, path);

        self.transpiled_modules
            .insert(FileName::Real(path.clone()), loaded.clone());

        loaded.additional_files.iter().for_each(|x| {
            if self
                .transpiled_modules
                .contains_key(&FileName::Real(x.clone()))
            {
                return;
            }

            self.add_file(x);
        });
    }

    pub fn generate_code(&self) -> String {
        let mut files_code = String::with_capacity(1024 * 8);

        for (_, transpiled_code) in self.transpiled_modules.iter() {
            files_code
                .push_str(&transpiled_code.output_module_declaration_code());
        }

        format!(
            r#"
((files) => {{
  const exports = {{}};

  for (const path of Object.keys(files)) {{
    exports[path] = {{}};
  }}

  for (const [path, instantiate] of Object.entries(files)) {{
    const require = (path) => exports[path];
    instantiate(require, exports[path]);
  }}

  console.log(exports);
}})({{
    {}
}});
"#,
            files_code,
        )
    }
}
