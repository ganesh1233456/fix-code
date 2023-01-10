#[openbrush::trait_definition]
pub trait Base {
    #[ink(message)]
    fn execute_function(&mut self, function_name: String, parameters: String)
        -> Result<(), ()>;
}
