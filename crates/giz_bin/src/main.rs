use bevy_ecs::prelude::World;
use cgmath::{Deg, Vector3};
use giz_core::{components::Name, resources::Root, Application};
use giz_input::InputPlugin;
use giz_math::{
    Children, DefaultBundle, GlobalTransform, MathPlugin, Parent, Transform,
};
use giz_render::{
    components::{Camera, CurrentCamera, Mesh, Vertex},
    Material, RenderPlugin, StateResource,
};
use giz_scripting::{Behavior, ScriptingPlugin};

const SHADER: &str = r#"

struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

struct CameraMatrices {   
    projection_matrix: mat4x4<f32>,
    view_matrix: mat4x4<f32>
}

@group(0) @binding(0)
var<uniform> camera_matrices: CameraMatrices;

@group(1) @binding(0)
var<uniform> model_matrix: mat4x4<f32>;

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

    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.3, 0.1, 0.5, 1.0);
}

"#;

fn test(world: &mut World) {
    let state = world.get_resource_mut::<StateResource>().unwrap().0.clone();

    let material = Material::new(&state, SHADER).unwrap();
    const VERTICES: &[Vertex] = &[
        Vertex {
            position: [0.5, 0.5, 0.0],
        },
        Vertex {
            position: [0.5, -0.5, 0.0],
        },
        Vertex {
            position: [-0.5, -0.5, 0.0],
        },
        Vertex {
            position: [-0.5, 0.5, 0.0],
        },
    ];
    const INDICES: &[u32] = &[0, 2, 1, 0, 3, 2];

    let root = world
        .spawn_empty()
        .insert(Name("Root".into()))
        .insert(Transform::new())
        .insert(GlobalTransform::new())
        .id();

    world.insert_resource(Root(root.clone()));

    for x in 0..10 {
        for y in 0..10 {
            let mut transform = Transform::new();
            transform.scale = Vector3::new(0.1, 0.1, 0.1);

            transform.position.x = (x as f32) / 21.0;
            transform.position.y = (y as f32) / 15.0;

            world
                .spawn_empty()
                .insert(Name("Mesh".into()))
                .insert(Parent(root))
                .insert(Children(vec![]))
                .insert(transform)
                .insert(GlobalTransform::new())
                .insert(Behavior::new("mesh".into(), "A".into()))
                .insert(Mesh::new(
                    &state,
                    &material,
                    VERTICES.to_vec(),
                    INDICES.to_vec(),
                ));
        }
    }

    let mut transform = Transform::new();
    transform.set_position(Vector3::new(0.0, 0.0, -4.0));
    world
        .spawn_empty()
        .insert(DefaultBundle {
            transform,
            name: Name("Camera".into()),
            parent: Parent(root.clone()),
            children: Children(vec![]),
            global_transform: GlobalTransform::new(),
        })
        .insert(Camera::new_perspective(
            &state.lock(),
            Deg(50.0),
            0.1,
            1000.0,
        ))
        .insert(CurrentCamera);
}

fn main() {
    pretty_env_logger::init();

    let mut app = Application::new()
        .with_plugin(RenderPlugin)
        .with_plugin(ScriptingPlugin)
        .with_plugin(MathPlugin)
        .with_plugin(InputPlugin);

    test(&mut app.world);

    app.run();
}
