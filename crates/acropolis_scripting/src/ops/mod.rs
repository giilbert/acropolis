mod fast_vec3;
mod generic;
mod method_call;

#[cfg(not(target_arch = "wasm32"))]
pub fn deno_get_all_props() -> Vec<deno_core::OpDecl> {
    vec![
        generic::op_get_component_prop::decl(),
        generic::op_set_component_prop::decl(),
        fast_vec3::op_set_component_vec3_prop::decl(),
        fast_vec3::op_get_component_vec3_prop::decl(),
        method_call::op_call_component_method_mut::decl(),
    ]
}
