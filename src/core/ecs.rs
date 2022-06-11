pub trait System {
    fn new() -> Self;
    fn init(&mut self);
}

pub trait Component {
    fn new();
    fn init(&mut self);
    fn entity_update(&self);
    fn update(&self);
    fn destroy(&mut self);
}
