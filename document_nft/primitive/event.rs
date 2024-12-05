#![cfg_attr(not(feature = "std"), no_std, no_main)]
use ink::prelude::vec;
use ink::primitives::AccountId;
use scale::{Decode, Encode};
use scale_info::TypeInfo;
#[ink::event]
pub struct DocumentEvent {
    #[ink(topic)]
    from: Option<AccountId>,
    #[ink(topic)]
    to: Option<AccountId>,
    #[ink(topic)]
    event_type: EventType,
}

#[derive(Debug, Decode, Encode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum EventType {
    Transfer {
        document_id: Vec<u8>,
    },
    Approval {
        document_id: Vec<u8>,
    },
    ApprovalForAll {
        owner: AccountId,
        operator: AccountId,
        approved: bool,
    },
}
