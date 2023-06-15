struct VertexInput {
  @location(0) position: vec3<f32>,
  @location(1) uv: vec2<f32>,
};

struct VertexOutput {
  @builtin(position) clip_position: vec4<f32>,
  @location(0) uv: vec2<f32>
};

struct CameraMatrices {   
  projection_matrix: mat4x4<f32>,
  view_matrix: mat4x4<f32>
}

@group(0) @binding(0)
var<uniform> camera_matrices: CameraMatrices;

@group(1) @binding(0)
var<uniform> model_matrix: mat4x4<f32>;

@group(2) @binding(0)
var texture: texture_2d<f32>;
@group(2) @binding(1)
var texture_sampler: sampler;

@vertex
fn vs_main(
  model: VertexInput,
) -> VertexOutput {
  var out: VertexOutput;

  out.clip_position =
    model_matrix
    * camera_matrices.projection_matrix
    * camera_matrices.view_matrix
    * vec4<f32>(model.position, 1.0);

  out.uv = model.uv;

  return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
  return textureSample(texture, texture_sampler, in.uv);
  // return vec4<f32>(0.3, 0.1, 0.5, 1.0);
}

