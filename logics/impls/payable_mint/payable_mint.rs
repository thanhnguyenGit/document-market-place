use extensions::metadata;
use openbrush::contracts::ownable;
use openbrush::contracts::psp34::psp34;
use openbrush::traits::{DefaultEnv, Storage};
use openbrush::{
    contracts::psp34::*,
    traits::{AccountId, String},
};

use crate::impls::payable_mint::types::Data;

#[openbrush::trait_definition]
pub trait PayableMintImpl:
    Storage<Data>
    + Storage<psp34::Data>
    + Storage<ownable::Data>
    + Storage<metadata::Data>
    + Internal
    + psp34::InternalImpl
{
    #[ink(message, payable)]
    fn mint(&mut self, to: AccountId, mint_amount: u64) -> Result<(), PSP34Error> {
        self.check_value(Self::env().transferred_value(), mint_amount)?;
        self.check_amount(mint_amount)?;

        let next_to_mint = self.data::<Data>().last_token_id + 1;
        let mint_offset = next_to_mint + mint_amount;

        for mint_id in next_to_mint..mint_offset {
            psp34::InternalImpl::_mint_to(self, to, Id::U64(mint_id))?;
            self.data::<Data>().last_token_id += 1;
        }
        Ok(())
    }
}

pub trait Internal: Storage<Data> + psp34::Internal {
    fn check_value(&self, transferred_value: u128, mint_amount: u64) -> Result<(), PSP34Error> {
        if let Some(value) = (mint_amount as u128).checked_mul(self.data::<Data>().price_per_mint) {
            if transferred_value == value {
                return Ok(());
            }
        }
        return Err(PSP34Error::Custom(String::from("BadMintValue")));
    }
    fn check_amount(&self, mint_amount: u64) -> Result<(), PSP34Error> {
        if mint_amount == 0 {
            return Err(PSP34Error::Custom(String::from("CannotMintZeroTokens")));
        }
        if let Some(amount) = self.data::<Data>().last_token_id.checked_add(mint_amount) {
            if amount <= self.data::<Data>().max_supply {
                return Ok(());
            }
        }
        return Err(PSP34Error::Custom(String::from("CollectionIsFull")));
    }
}
