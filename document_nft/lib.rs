#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[pendzl::implementation(PSP34, PSP34Metadata, PSP34Burnable)]
#[ink::contract]
pub mod document_nft {
    use ink::prelude::string::*;
    use pendzl::contracts::psp34::*;
    use pendzl::contracts::psp34::{burnable::PSP34Burnable, metadata::PSP34Metadata, PSP34Error};
    type DocumentResult<T> = Result<T, PSP34Error>;
    #[ink(storage)]
    #[derive(Default, StorageFieldGetter)]
    pub struct DocumentNft {
        #[storage_field]
        document: PSP34Data,
        #[storage_field]
        metadata: PSP34MetadataData,
        // #[storage_field]
        // ownable: OwnableData,
        // #[storage_field]
        // burnable: PSP34BurnableData,
        next_id: u8,
    }

    impl DocumentNft {
        #[ink(constructor)]
        pub fn new() -> Self {
            let caller = Self::env().caller();
            let mut instance = Self::default();
            // instance._update_owner(&Some(caller));
            instance
        }
        #[ink(message)]
        pub fn mint_token(&mut self) -> DocumentResult<()> {
            let caller = self.env().caller();
            let next_id = Id::U8(self.next_id);
            self._mint_to(&caller, &next_id)?;
            self.next_id = self.next_id.checked_add(1).unwrap();
            Ok(())
        }

        #[ink(message)]
        pub fn mint(&mut self, id: Id, name: String, symbol: String) -> DocumentResult<()> {
            let caller = self.env().caller();
            let name_key = String::from("name");
            let symbol_key = String::from("symbol");
            self._set_attribute(&id.clone(), &name_key, &name);
            self._set_attribute(&id.clone(), &symbol_key, &symbol);
            Ok(())
        }
    }
}
