use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use crate::lib::rendering::State;

pub struct StateResource(pub State);
impl Deref for StateResource {
    type Target = State;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for StateResource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
