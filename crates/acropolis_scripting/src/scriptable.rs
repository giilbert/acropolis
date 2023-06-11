pub trait Scriptable {
    fn set_property(&mut self, name: &str, value: String) {
        unimplemented!("set_property {name}: {value}")
    }

    fn get_property(&self, name: &str) -> String {
        unimplemented!("get_property {name}")
    }

    fn set_property_vec3(&mut self, property: u32, x: f64, y: f64, z: f64) {
        unimplemented!("set_property_vec3 {property}: {x}, {y}, {z}")
    }

    fn get_property_vec3(&self, property: u32) -> (f64, f64, f64) {
        unimplemented!("get_property_vec3 {property}")
    }
}
