use std::{ops::Deref, sync::Arc};

use bevy_ecs::prelude::*;
use deno_core::parking_lot::Mutex;
use rapier2d::prelude::*;

pub struct PhysicsInner {
    pub pipeline: PhysicsPipeline,
    pub gravity: Vector<f32>,
    pub integration_parameters: IntegrationParameters,
    pub island_manager: IslandManager,
    pub broad_phase: BroadPhase,
    pub narrow_phase: NarrowPhase,
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_joint_set: MultibodyJointSet,
    pub ccd_solver: CCDSolver,
}

#[derive(Resource, Clone)]
pub struct PhysicsResource(pub Arc<Mutex<PhysicsInner>>);

impl PhysicsResource {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(PhysicsInner {
            pipeline: PhysicsPipeline::new(),
            gravity: vector![0.0, -9.81],
            integration_parameters: IntegrationParameters::default(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
        })))
    }
}

impl Deref for PhysicsResource {
    type Target = Mutex<PhysicsInner>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
