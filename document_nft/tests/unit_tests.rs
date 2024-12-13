#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[cfg(test)]
mod tests {
    use document_nft::document_nft::DocumentNft;
    use ink::{
        env::test::set_caller,
        prelude::{
            string::{String, ToString},
            vec,
        },
    };
    use pendzl::contracts::psp34::burnable::PSP34Burnable;
    use pendzl::contracts::psp34::mintable::PSP34Mintable;
    use pendzl::contracts::psp34::*;
    use pendzl::traits::Balance;

    #[ink::test]
    fn mint_once_works() {
        let account = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
        let mut contract = DocumentNft::new();
        assert_eq!(contract.owner_of(Id::U8(0)), None);
        let name = String::from("Thanh Picture");
        let symbol = String::from("THP");
        let price = 11;
        contract.mint_once(name.clone(), symbol.clone(), price);
        assert_eq!(contract.nonce, 1);
        contract.mint_once(name.clone(), symbol.clone(), price);
        assert_eq!(contract.nonce, 2);
    }
    #[ink::test]
    fn mint_batch_works() {
        let account = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
        let mut contract = DocumentNft::new();
        assert_eq!(contract.owner_of(Id::U8(0)), None);
        let name1 = String::from("Thanh Picture");
        let symbol1 = String::from("THP");
        let price1 = 11;
        let name2 = String::from("Thanh King");
        let symbol2 = String::from("TKING");
        let price2 = 22;
        let name3 = String::from("Thanh Doc");
        let symbol3 = String::from("TDOC");
        let price3 = 0;
        let mut data = Vec::default();
        data.push((name1, symbol1, price1));
        data.push((name2, symbol2, price2));
        data.push((name3, symbol3, price3));
        contract.batch_mint(data);

        assert_eq!(
            contract
                .metadata
                .attributes
                .get((Id::U8(0), "name".to_string()))
                .unwrap(),
            String::from("Thanh Picture")
        );

        assert_eq!(
            contract
                .metadata
                .attributes
                .get((Id::U8(1), "name".to_string()))
                .unwrap(),
            String::from("Thanh King")
        );

        assert_eq!(
            contract
                .metadata
                .attributes
                .get((Id::U8(2), "name".to_string()))
                .unwrap(),
            String::from("Thanh Doc")
        );
    }
    #[ink::test]
    fn transfer_worl() {
        let account = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
        let mut contract = DocumentNft::new();
        let name = String::from("Thanh Picture");
        let symbol = String::from("THP");
        let price = 11;
        assert_eq!(contract.mint_once(name, symbol, price), Ok(()));
        assert_eq!(contract.balance_of(account.alice), 1);
        assert_eq!(contract.balance_of(account.bob), 0);
        assert_eq!(
            contract.transfer(account.bob, Id::U8(0), Vec::default()),
            Ok(())
        );
        assert_eq!(contract.balance_of(account.bob), 1);
        assert_eq!(contract.balance_of(account.alice), 0);
    }
    #[ink::test]
    fn burn_work() {
        let account = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
        let mut contract = DocumentNft::new();
        let name = String::from("Thanh Picture");
        let symbol = String::from("THP");
        let price = 11;
        assert_eq!(contract.mint_once(name, symbol, price), Ok(()));
        assert_eq!(contract.balance_of(account.alice), 1);
        assert_eq!(contract.balance_of(account.bob), 0);
        assert_eq!(
            contract.burn(account.bob, Id::U8(0)),
            Err(PSP34Error::NotApproved)
        );
        assert_eq!(contract.burn(account.alice, Id::U8(0)), Ok(()))
    }
    #[ink::test]
    fn approval_work() {
        let account = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
        let mut contract = DocumentNft::new();
        let name = String::from("Thanh Picture");
        let symbol = String::from("THP");
        let price = 11;
        assert_eq!(contract.mint_once(name, symbol, price), Ok(()));
        assert_eq!(contract.balance_of(account.alice), 1);
        assert_eq!(contract.approve(account.bob, Some(Id::U8(0)), true), Ok(()));
        set_caller::<ink::env::DefaultEnvironment>(account.bob);
        assert_eq!(contract.burn(account.bob, Id::U8(0)), Ok(()));
    }
}
