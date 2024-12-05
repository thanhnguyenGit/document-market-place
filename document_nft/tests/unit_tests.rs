#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[cfg(test)]
mod tests {
    use document_nft::document_nft::DocumentNft;

    #[ink::test]
    fn default_works() {
        let document_nft = DocumentNft::default();
        assert_eq!(document_nft.get(), false);
    }

    /// We test a simple use case of our contract.
    #[ink::test]
    fn it_works() {
        let mut document_nft = DocumentNft::new(false);
        assert_eq!(document_nft.get(), false);
        document_nft.flip();
        assert_eq!(document_nft.get(), true);
    }
}
