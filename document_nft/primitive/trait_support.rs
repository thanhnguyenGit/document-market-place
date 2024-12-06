#![cfg_attr(not(feature = "std"), no_std, no_main)]
use core::fmt::Debug;

use ink::prelude::vec;
use ink::primitives::AccountId;
use scale::{Decode, Encode};
use scale_info::TypeInfo;

pub trait Erc721Helper<A, T, R, E>
where
    A: AsRef<[u8]>,
    T: Sized,
    R: Sized,
    E: Sized,
{
    fn add_erc721_to(&mut self, to: &A, token_id: T) -> Result<R, E>;
    fn transfer_erc721_from(&mut self, caller: &A, from: &A, to: &A, token_id: T) -> Result<R, E>;
    fn remove_erc721_from(&mut self, from: &A, token_id: T) -> Result<R, E>;
    fn approve_for_all(&mut self, caller: &A, to: &A, approved: bool) -> Result<R, E>;
    fn approve_for(&mut self, caller: &A, to: &A, token_id: T) -> Result<R, E>;
    fn clear_approval(&mut self, token_id: T);
    fn get_owned_token_count_or_zero(&self, of: &A) -> u32;
    fn approved_for_all(&self, owner: A, operator: A) -> bool;
    fn approved_or_owner(&self, from: A, token_id: T, owner: A) -> bool;
    fn owner_of(&self, id: T) -> Option<A>;
    fn is_null_account(&self, account: &A) -> bool;
}
