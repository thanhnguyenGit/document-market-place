#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[pendzl::implementation(PSP34, PSP34Metadata, PSP34Burnable)]
#[ink::contract]
pub mod document_nft {
    use ink::{
        prelude::{
            string::{String, ToString},
            vec,
        },
        storage::traits::AutoStorableHint,
    };
    use pendzl::contracts::psp34::mintable::PSP34Mintable;
    use pendzl::contracts::psp34::{Id, PSP34Error, *};
    use pendzl::contracts::{access_control::RoleType, psp34::metadata::PSP34Metadata};
    type DocumentResult<T> = Result<T, PSP34Error>;

    #[ink(storage)]
    #[derive(Default, StorageFieldGetter)]
    pub struct DocumentNft {
        #[storage_field]
        pub document: PSP34Data,
        #[storage_field]
        pub metadata: PSP34MetadataData,
        pub nonce: u8,
    }

    impl DocumentNft {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }
        #[ink(message)]
        pub fn mint_once(
            &mut self,
            name: String,
            symbol: String,
            price: Balance,
        ) -> DocumentResult<()> {
            let caller = self.env().caller();
            self.mint(caller, name, symbol, price)
        }
        #[ink(message)]
        pub fn batch_mint(&mut self, data: Vec<(String, String, Balance)>) -> DocumentResult<()> {
            let caller = self.env().caller();
            for (name, symbol, price) in data {
                self.mint(caller, name, symbol, price)?
            }
            Ok(())
        }
        #[ink(message)]
        pub fn set_base_uri(&mut self, uri: String, id: Id) -> DocumentResult<()> {
            self._set_attribute(&id, &String::from("baseURI"), &uri);
            Ok(())
        }
        #[ink(message)]
        pub fn burn(&mut self, id: Id) -> DocumentResult<()> {
            if let Some(owner) = self._owner_of(&id) {
                let caller = Self::env().caller();
                if caller == owner || self._allowance(&owner, &caller, &Some(id.clone())) {
                    self._burn_from(&caller, &id)
                } else {
                    Err(PSP34Error::NotApproved)
                }
            } else {
                Err(PSP34Error::TokenNotExists)
            }
        }
        fn token_exists(&self, id: Id) -> DocumentResult<()> {
            self._owner_of(&id).ok_or(PSP34Error::TokenNotExists)?;
            Ok(())
        }
        fn mint(
            &mut self,
            caller: AccountId,
            name: String,
            symbol: String,
            price: Balance,
        ) -> DocumentResult<()> {
            let id = Id::U8(self.nonce);
            let symbol_key = String::from("symbol");
            let name_key = String::from("name");
            let price_key = String::from("price");
            self._set_attribute(&id, &name_key, &name);
            self._set_attribute(&id, &symbol_key, &symbol);
            self._set_attribute(&id, &price_key, &price.to_string());
            self._mint_to(&caller, &id)?;
            self.nonce = self.nonce.checked_add(1).unwrap();
            Ok(())
        }
    }
}
