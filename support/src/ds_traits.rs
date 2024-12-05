#![cfg_attr(not(feature = "std"), no_std, no_main)]
pub trait HashBuilder {
    type OutputType;
    fn add_buffer(self, input: &[u8]) -> Self;
    fn build(self) -> Self::OutputType;
}
