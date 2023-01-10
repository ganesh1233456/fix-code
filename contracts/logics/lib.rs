#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::wrapper]
pub type BaseRef = dyn Base;

#[openbrush::trait_definition]
pub trait Base {
    #[ink(message)]
    // fn execute_function(&mut self, function_name: String, parameters: String)
    fn execute_function(&mut self)
        -> Result<(), ()>;
}
