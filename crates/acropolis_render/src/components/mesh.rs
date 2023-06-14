use crate::{Material, StateInner};
use acropolis_loader::Context;
use acropolis_scripting::serde_json::{self, Value};
use bevy_ecs::prelude::Component;
use serde::Deserialize;
use wgpu::{util::DeviceExt, BindGroup, Buffer, RenderPipeline};

#[derive(Component, Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub render_pipeline: RenderPipeline,
    pub transformation_matrix_buffer: Buffer,
    pub bind_group: BindGroup,
}

#[derive(Deserialize)]
struct MeshData {
    pub material: String,
    pub geometry: GeometryData,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
enum GeometryData {
    RawGeometry {
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
    },
}

impl Mesh {
    pub fn from_raw_geometry(
        state: &StateInner,
        material: &Material,
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
    ) -> Self {
        let camera_bind_group_layout = state.device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("Mesh Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::all(),
                    count: None,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                }],
            },
        );

        let transformation_matrix_bind_group_layout = state
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Mesh Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::all(),
                    count: None,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                }],
            });

        let vertex_buffer = state.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Mesh Vertex Buffer"),
                contents: bytemuck::cast_slice(vertices.as_ref()),
                usage: wgpu::BufferUsages::VERTEX,
            },
        );
        let index_buffer = state.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Mesh Index Buffer"),
                contents: bytemuck::cast_slice(indices.as_ref()),
                usage: wgpu::BufferUsages::INDEX,
            },
        );

        let pipeline_layout = state.device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("Mesh Render Pipeline Layout"),
                bind_group_layouts: &[
                    &camera_bind_group_layout,
                    &transformation_matrix_bind_group_layout,
                ],
                push_constant_ranges: &[],
            },
        );

        let render_pipeline = state.device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some("Mesh Render Pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &material.module,
                    entry_point: "vs_main",
                    buffers: &[Vertex::desc()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &material.module,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: state.config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            },
        );

        let transformation_matrix_buffer = state.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Mesh Uniform Buffer"),
                #[rustfmt::skip]
                contents: bytemuck::cast_slice(&[0.0f32; 32]),
                usage: wgpu::BufferUsages::UNIFORM
                    | wgpu::BufferUsages::COPY_DST,
            },
        );

        let bind_group_layout = state.device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("Mesh Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::all(),
                    count: None,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                }],
            },
        );

        let bind_group =
            state.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Camera Bind Group"),
                layout: &bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: transformation_matrix_buffer.as_entire_binding(),
                }],
            });

        Self {
            vertices,
            indices,
            vertex_buffer,
            index_buffer,
            render_pipeline,
            transformation_matrix_buffer,
            bind_group,
        }
    }

    pub fn load(
        context: &Context,
        // assets: &HashMap<String, Arc<Mutex<Option<Box<dyn Any>>>>>,
        state: &StateInner,
        value: Value,
    ) -> Self {
        let data = serde_json::from_value::<MeshData>(value).unwrap();
        let geometry = data.geometry;
        let material = context
            .assets
            .get(&data.material)
            .unwrap()
            .lock()
            .unwrap()
            .take()
            .unwrap();
        let material = material.downcast::<Material>().unwrap();

        match geometry {
            GeometryData::RawGeometry { vertices, indices } => {
                Self::from_raw_geometry(state, &material, vertices, indices)
            }
        }
    }
}

#[derive(
    bytemuck::Zeroable, bytemuck::Pod, Copy, Clone, Debug, Deserialize,
)]
#[repr(C)]
pub struct Vertex {
    pub position: [f32; 3],
}

impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x3,
            }],
        }
    }
}
