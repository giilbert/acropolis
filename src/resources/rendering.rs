use std::rc::Rc;

pub struct GlResource {
    pub gl: Rc<glow::Context>,
}
