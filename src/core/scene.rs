use crate::core::ecs::Entity;
use std::rc::Rc;

struct Scene<'a> {
    children: Vec<Rc<Entity<'a>>>,
}
