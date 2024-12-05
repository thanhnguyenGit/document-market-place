#![cfg_attr(not(feature = "std"), no_std, no_main)]
// import mod,crate,..
mod ds_traits;
extern crate ink;
extern crate scale;
extern crate scale_info;
// use mod,crate,..
use ds_traits::HashBuilder;
use ink::env::hash::HashOutput;
use ink::primitives::Hash;
use ink::{env::hash::Blake2x256, prelude::vec};
use scale::{Decode, Encode};

pub fn caller_is_owner<T>(caller: T)
where
    T: AsRef<[u8]>,
{
}

#[derive(Debug, Default)]
pub struct HashAggregate {
    buffer: [u8; 32],
}
impl HashBuilder for HashAggregate {
    type OutputType = Hash;
    #[must_use]
    fn add_buffer(mut self, input: &[u8]) -> Self {
        assert!(
            !input.iter().all(|&element| element == 0),
            "The input must not be a zero array"
        );
        let mut output_hash = <Blake2x256 as HashOutput>::Type::default();
        ink::env::hash_bytes::<Blake2x256>(&self.buffer, &mut output_hash);
        self.buffer = output_hash.into();
        self
    }
    fn build(self) -> Self::OutputType {
        self.buffer.into()
    }
}

#[cfg(test)]
mod tests {
    use ink::primitives::AccountId;

    use super::*;

    #[test]
    fn build_hash_aggregation() {
        let mut hash_aggregation = HashAggregate::default();
        let input1: AccountId = [0x11; 32].into();
        let input2: AccountId = [0x12; 32].into();
        let result = hash_aggregation
            .add_buffer(input1.as_ref())
            .add_buffer(input2.as_ref())
            .build();
        assert_ne!(result, [0u8; 32].into());
        println!("result {:?}", result);
    }
}
