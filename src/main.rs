use deno_core::{op, Extension, JsRuntime, RuntimeOptions};
use serde::Deserialize;

#[derive(Deserialize)]
struct Test {
    a: i32,
    b: i8,
    c: Option<Box<Test>>
}

#[op]
fn op_test(value: Test) -> Result<i32, deno_core::error::AnyError> {
    if value.c.is_some() {
        let inner = value.c.unwrap();
        println!("{}\n{}", inner.a, inner.b);
    }

    Ok(value.a)
}

fn main() {
  // Build a deno_core::Extension providing custom ops
  let ext = Extension::builder()
    .ops(vec![
      op_test::decl(), 
    ])
    .build();

  // Initialize a runtime instance
  let mut runtime = JsRuntime::new(RuntimeOptions {
    extensions: vec![ext],
    ..Default::default()
  });

  runtime
    .execute_script(
      "<usage>",
      r#"

Deno.core.print(Deno.core.opSync("op_test", {
    a: 10,
    b: 2.2,
    c: {
        a: 20,
        b: 4.4
    }
}) + "\n");

"#,
    )
    .unwrap();
}
