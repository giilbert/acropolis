use crate::{state::StateInner, StateResource};
use bevy_ecs::{prelude::Component, world::World};
use bytemuck::{Pod, Zeroable};
use cgmath::{Deg, Matrix4, Rad};
use deno_core::serde_json::{self, Value};
use serde::Deserialize;
use wgpu::{util::DeviceExt, BindGroup, Buffer};

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

#[derive(Component)]
pub struct Camera {
    pub projection_matrix: Matrix4<f32>,
    pub camera_data: CameraData,
    pub bind_group: BindGroup,
    pub projection_matrix_buffer: Buffer,
}

#[derive(Component)]
pub struct CurrentCamera;

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum CameraData {
    Perspective {
        fov: Deg<f32>,
        #[serde(skip)]
        aspect_ratio: f32,
        near: f32,
        far: f32,
    },
    Orthographic {},
}

impl Camera {
    pub fn new_perspective<T>(
        state: &StateInner,
        fov: T,
        near: f32,
        far: f32,
    ) -> Camera
    where
        T: Copy + Into<Rad<f32>>,
    {
        // create perspective matrix from fov
        let aspect_ratio =
            (state.size.width as f32) / (state.size.height as f32);
        let projection_matrix = OPENGL_TO_WGPU_MATRIX
            * cgmath::perspective(fov, aspect_ratio.into(), near, far);

        let projection_matrix_buffer = state.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Mesh Uniform Buffer"),
                #[rustfmt::skip]
                contents: bytemuck::cast_slice(&[0.0f32; 64]),
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
                    resource: projection_matrix_buffer.as_entire_binding(),
                }],
            });

        Camera {
            projection_matrix,
            camera_data: CameraData::Perspective {
                fov: fov.into().into(),
                aspect_ratio,
                near,
                far,
            },
            bind_group,
            projection_matrix_buffer,
        }
    }

    pub fn from_json(world: &mut World, value: Value) -> Self {
        let state_resource = world.get_resource::<StateResource>().unwrap();
        let state = state_resource.0.lock();

        let camera_data: CameraData = serde_json::from_value(value).unwrap();

        match camera_data {
            CameraData::Perspective { fov, near, far, .. } => {
                Self::new_perspective(&state, fov, near, far)
            }
            CameraData::Orthographic {} => todo!(),
        }
    }

    pub fn update_projection_matrix(&mut self, state: &mut StateInner) {
        let aspect_ratio =
            (state.size.width as f32) / (state.size.height as f32);
        let matrix = match &self.camera_data {
            &CameraData::Perspective { fov, near, far, .. } => {
                OPENGL_TO_WGPU_MATRIX
                    * cgmath::perspective(fov, aspect_ratio.into(), near, far)
            }
            _ => todo!(),
        };

        if let CameraData::Perspective { fov, near, far, .. } = self.camera_data
        {
            self.camera_data = CameraData::Perspective {
                fov,
                aspect_ratio,
                near,
                far,
            }
        }

        self.projection_matrix = matrix;
    }
}

#[repr(C, align(16))]
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
pub struct CameraUniform {
    pub projection_matrix: [[f32; 4]; 4],
    pub view_matrix: [[f32; 4]; 4],
}
