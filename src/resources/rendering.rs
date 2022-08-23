use std::{ops::Deref, rc::Rc};

use crate::{lib::rendering::Material, utils::types::Matrix4};

pub struct GlResource(pub Rc<glow::Context>);
impl Deref for GlResource {
    type Target = glow::Context;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct MaterialsResource(pub Vec<Material>);
impl Deref for MaterialsResource {
    type Target = Vec<Material>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct CurrentCameraMatrixResource {
    pub projection_matrix: Matrix4,
    pub view_matrix: Matrix4,
}
