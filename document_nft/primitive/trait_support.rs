#![cfg_attr(not(feature = "std"), no_std, no_main)]
use ink::prelude::vec;
use ink::primitives::AccountId;
use scale::{Decode, Encode};
use scale_info::TypeInfo;

pub trait Erc721Helper<A, T, R, E>
where
    A: AsRef<[u8]>,
    T: Sized + Encode + Decode,
{
    fn add_erc721_to(&mut self, to: &A, token_id: T) -> Result<R, E>;
    // fn approve_for_all(&mut self, to: &A, approved: bool) -> Result<R, E>;
    // fn approve_for(&mut self, to: &A, token_id: T) -> Result<R, E>;
    // fn clear_approval(&mut self, token_id: T);
    // fn get_owned_token_count_or_zero(&self, of: &A) -> u32;
    // fn approved_for_all(&self, owner: A, operator: A) -> bool;
    // fn approved_or_owner(&self, from: A, token_id: T, owner: A) -> bool;
}
