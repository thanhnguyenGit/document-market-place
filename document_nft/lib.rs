#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub mod primitive;

#[ink::contract]
pub mod document_nft {
    use crate::primitive::{enumerable::Error, event::DocumentEvent, trait_support::Erc721Helper};
    use ink::storage::{Lazy, Mapping, StorageVec};
    use scale::{Decode, Encode};
    use support;
    type DocumentId = Hash;
    type DocumentResult<T> = Result<T, Error>;
    #[ink(storage)]
    pub struct DocumentNft {
        document_owner: Mapping<DocumentId, AccountId>,
        document_approvals: Mapping<DocumentId, AccountId>,
        owned_document_count: Mapping<AccountId, u32>,
        operator_approvals: Mapping<(AccountId, AccountId), ()>,
    }

    impl Default for DocumentNft {
        fn default() -> Self {
            Self {
                document_owner: Mapping::default(),
                document_approvals: Mapping::default(),
                owned_document_count: Mapping::default(),
                operator_approvals: Mapping::default(),
            }
        }
    }

    impl DocumentNft {
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn mint_document(&mut self, document_id: DocumentId) -> DocumentResult<()> {
            let caller = self.env().caller();
            let res: DocumentResult<()> = self.add_erc721_to(&caller, document_id);
            Ok(())
        }
    }
    impl<A, T, R, E> Erc721Helper<A, T, R, E> for DocumentNft
    where
        T: Sized + Encode + Decode,
        A: AsRef<[u8]>,
    {
        fn add_erc721_to(&mut self, to: &A, token_id: T) -> Result<R, E> {
            unimplemented!()
        }
    }
}
