#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub mod primitive;

#[ink::contract]
pub mod document_nft {
    use core::{error::Error, fmt::Debug};

    use crate::primitive::{
        enumerable::Error as DocumentError,
        event::{DocumentEvent, EventType as DocEventType},
        trait_support::Erc721Helper,
    };
    use ink::storage::{Lazy, Mapping, StorageVec};
    use scale::{Decode, Encode};
    use support;
    type DocumentId = Hash;
    type DocumentResult<T> = Result<T, DocumentError>;
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
            self.add_erc721_to(&caller, document_id)?;
            self.env().emit_event(DocumentEvent {
                from: Some(AccountId::from([0x0; 32])),
                to: Some(caller),
                event_type: DocEventType::Transfer {
                    document_id: document_id.into(),
                },
            });

            Ok(())
        }
        pub fn burn_document(&mut self, )
    }
    impl Erc721Helper<AccountId, DocumentId, (), DocumentError> for DocumentNft {
        fn add_erc721_to(
            &mut self,
            to: &AccountId,
            token_id: DocumentId,
        ) -> Result<(), DocumentError> {
            if self.document_owner.contains(token_id) {
                return Err(DocumentError::DocumentExists);
            }
            if self.is_null_account(to) {
                return Err(DocumentError::NotAllowed);
            }
            let count = self
                .owned_document_count
                .get(to)
                .map(|count| count.checked_add(1).unwrap())
                .unwrap_or(1);
            self.owned_document_count.insert(to, &count);
            self.document_owner.insert(token_id, to);
            Ok(())
        }
        fn transfer_erc721_from(
            &mut self,
            caller: &AccountId,
            from: &AccountId,
            to: &AccountId,
            token_id: DocumentId,
        ) -> Result<(), DocumentError> {
            let owner = self
                .owner_of(token_id)
                .ok_or(DocumentError::DocumentNotFound)?;
            if !self.approved_or_owner(*caller, token_id, owner) {
                return Err(DocumentError::NotApproved);
            }
            if owner != *from {
                return Err(DocumentError::NotOwner);
            }
            self.clear_approval(token_id);
            self.remove_erc721_from(from, token_id)?;
            self.add_erc721_to(to, token_id)?;
            Ok(())
        }
        fn remove_erc721_from(
            &mut self,
            from: &AccountId,
            token_id: DocumentId,
        ) -> Result<(), DocumentError> {
            if !self.document_owner.contains(token_id) {
                return Err(DocumentError::DocumentNotFound);
            }
            let count = self
                .owned_document_count
                .get(from)
                .map(|count| count.checked_sub(1).unwrap())
                .ok_or(DocumentError::CannotFetchValue)?;
            self.owned_document_count.insert(from, &count);
            self.document_owner.remove(token_id);
            Ok(())
        }
        fn approve_for_all(
            &mut self,
            caller: &AccountId,
            to: &AccountId,
            approved: bool,
        ) -> Result<(), DocumentError> {
            if caller == to {
                return Err(DocumentError::NotAllowed);
            }
            if approved {
                self.operator_approvals.insert((caller, to), &());
            } else {
                self.operator_approvals.remove((caller, to));
            }
            Ok(())
        }
        // Approved the passed 'AccountId' to transfer the specidic document on behalf of
        // the message's sender
        fn approve_for(
            &mut self,
            caller: &AccountId,
            to: &AccountId,
            token_id: DocumentId,
        ) -> Result<(), DocumentError> {
            let owner = self
                .owner_of(token_id)
                .ok_or(DocumentError::DocumentNotFound)?;
            if !(owner == *caller || self.approved_for_all(owner, *caller)) {
                return Err(DocumentError::DocumentNotFound);
            }
            if self.is_null_account(to) {
                return Err(DocumentError::NotAllowed);
            }
            if self.document_approvals.contains(token_id) {
                return Err(DocumentError::CannotInsert);
            } else {
                self.document_approvals.insert(token_id, to);
            }
            Ok(())
        }
        fn clear_approval(&mut self, token_id: DocumentId) {
            self.document_approvals.remove(token_id)
        }
        fn get_owned_token_count_or_zero(&self, of: &AccountId) -> u32 {
            self.owned_document_count
                .get(of)
                .expect("This account does not own any Document")
        }
        fn approved_for_all(&self, owner: AccountId, operator: AccountId) -> bool {
            self.operator_approvals.contains((owner, operator))
        }
        fn approved_or_owner(
            &self,
            from: AccountId,
            token_id: DocumentId,
            owner: AccountId,
        ) -> bool {
            self.is_null_account(&from)
                && (from == owner
                    || self.document_approvals.get(token_id) == Some(from)
                    || self.approved_for_all(owner, from))
        }
        fn owner_of(&self, id: DocumentId) -> Option<AccountId> {
            self.document_owner.get(id)
        }

        fn is_null_account(&self, account: &AccountId) -> bool {
            account.eq(&AccountId::from([0x0; 32]))
        }
    }
}
