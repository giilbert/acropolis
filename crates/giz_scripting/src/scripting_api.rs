pub trait ScriptingApi {
    fn set_property(&mut self, name: &str, value: String);
    fn get_property(&self, name: &str) -> String;
}
