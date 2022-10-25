use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use cgmath::Matrix4;

use crate::lib::rendering::State;

pub struct StateResource(pub Rc<RefCell<State>>);
impl Deref for StateResource {
    type Target = Rc<RefCell<State>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for StateResource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// pub struct MaterialsResource(pub Vec<Material>);

pub struct CurrentCameraMatrixResource {
    pub projection_matrix: Matrix4<f32>,
    pub view_matrix: Matrix4<f32>,
}
