#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[cfg(test)]
mod tests {
    use document_nft::document_nft::DocumentNft;
    use ink::prelude::{
        string::{String, ToString},
        vec,
    };
    use pendzl::traits::Balance;

    /// We test a simple use case of our contract.
    #[ink::test]
    fn mint_once_works() {
        let mut contract = DocumentNft::new();
        let name = String::from("Thanh Picture");
        let symbol = String::from("THP");
        let price = 11;
        contract.mint_once(name, symbol, price);
    }
}
