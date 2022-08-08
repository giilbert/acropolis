use std::rc::Rc;

use crate::lib::rendering::Material;

pub struct GlResource(pub Rc<glow::Context>);
pub struct MaterialsResource(pub Vec<Material>);
