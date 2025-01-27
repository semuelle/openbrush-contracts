#![feature(min_specialization)]
#[cfg(feature = "psp34")]
#[brush::contract]
mod psp34_mintable {
    use brush::test_utils::accounts;
    use contracts::psp34::{
        extensions::mintable::*,
        Id,
    };
    use ink_lang as ink;

    #[derive(Default, PSP34Storage)]
    #[ink(storage)]
    pub struct PSP34Struct {
        #[PSP34StorageField]
        psp34: PSP34Data,
        // field for testing _before_token_transfer
        return_err_on_before: bool,
        // field for testing _after_token_transfer
        return_err_on_after: bool,
    }

    impl PSP34Internal for PSP34Struct {
        fn _before_token_transfer(
            &mut self,
            _from: Option<&AccountId>,
            _to: Option<&AccountId>,
            _id: &Id,
        ) -> Result<(), PSP34Error> {
            if self.return_err_on_before {
                return Err(PSP34Error::Custom(String::from("Error on _before_token_transfer")))
            }
            Ok(())
        }

        fn _after_token_transfer(
            &mut self,
            _from: Option<&AccountId>,
            _to: Option<&AccountId>,
            _id: &Id,
        ) -> Result<(), PSP34Error> {
            if self.return_err_on_after {
                return Err(PSP34Error::Custom(String::from("Error on _after_token_transfer")))
            }
            Ok(())
        }
    }

    impl PSP34 for PSP34Struct {}
    impl PSP34Mintable for PSP34Struct {}

    impl PSP34Struct {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        pub fn change_state_err_on_before(&mut self) {
            self.return_err_on_before = !self.return_err_on_before;
        }

        pub fn change_state_err_on_after(&mut self) {
            self.return_err_on_after = !self.return_err_on_after;
        }
    }

    #[ink::test]
    fn mint_works() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        // Token 1 does not _exists.
        assert_eq!(nft.owner_of(Id::U8(1u8)), None);
        // Alice does not owns tokens.
        assert_eq!(nft.balance_of(accounts.alice), 0);
        // Create token Id 1.
        assert!(nft.mint(accounts.alice, Id::U8(1u8)).is_ok());
        // Alice owns 1 token.
        assert_eq!(nft.balance_of(accounts.alice), 1);
    }

    #[ink::test]
    fn mint_existing_should_fail() {
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        // Create token Id 1.
        assert!(nft.mint(accounts.alice, Id::U8(1u8)).is_ok());
        // Alice owns 1 token.
        assert_eq!(nft.balance_of(accounts.alice), 1);
        // Alice owns token Id 1.
        assert_eq!(nft.owner_of(Id::U8(1u8)), Some(accounts.alice));
        // Cannot create token Id if it _exists.
        assert_eq!(nft.mint(accounts.alice, Id::U8(1u8)), Err(PSP34Error::TokenExists));
        assert_eq!(nft.mint(accounts.bob, Id::U8(1u8)), Err(PSP34Error::TokenExists));
    }

    #[ink::test]
    fn before_token_transfer_should_fail_mint() {
        // Constructor works.
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        // Can mint token to Alice
        assert!(nft.mint(accounts.alice, Id::U8(1u8)).is_ok());
        assert_eq!(nft.balance_of(accounts.alice), 1);
        // Turn on error on _before_token_transfer
        nft.change_state_err_on_before();
        // Alice gets an error on _before_token_transfer
        assert_eq!(
            nft.mint(accounts.alice, Id::U8(4u8)),
            Err(PSP34Error::Custom(String::from("Error on _before_token_transfer")))
        );
    }

    #[ink::test]
    fn after_token_transfer_should_fail_mint() {
        // Constructor works.
        let accounts = accounts();
        // Create a new contract instance.
        let mut nft = PSP34Struct::new();
        // Can mint token to Alice
        assert!(nft.mint(accounts.alice, Id::U8(1u8)).is_ok());
        assert_eq!(nft.balance_of(accounts.alice), 1);
        // Turn on error on _after_token_transfer
        nft.change_state_err_on_after();
        // Alice gets an error on _after_token_transfer
        assert_eq!(
            nft.mint(accounts.alice, Id::U8(4u8)),
            Err(PSP34Error::Custom(String::from("Error on _after_token_transfer")))
        );
    }
}
