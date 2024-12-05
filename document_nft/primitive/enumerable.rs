#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[derive(Debug, PartialEq, Eq, Clone)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
pub enum Error {
    NotOwner,
    NotApproved,
    DocumentExists,
    DocumentNotFound,
    CannotInsert,
    CannotFetchValue,
    NotAllowed,
}
