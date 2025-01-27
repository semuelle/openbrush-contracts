#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod flash_borrower {
    use brush::contracts::psp22::extensions::flashmint::*;
    use ink_lang::codegen::Env;
    use ink_prelude::{
        string::String,
        vec::Vec,
    };

    #[ink(storage)]
    pub struct FlashBorrowerStruct {}

    impl FlashBorrowerStruct {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }
    }

    impl FlashBorrower for FlashBorrowerStruct {
        #[ink(message)]
        fn on_flashloan(
            &mut self,
            _initiator: AccountId,
            token: AccountId,
            amount: Balance,
            fee: Balance,
            _data: Vec<u8>,
        ) -> Result<(), FlashBorrowerError> {
            if PSP22Ref::approve(&token, self.env().caller(), amount + fee).is_err() {
                return Err(FlashBorrowerError::FlashloanRejected(String::from("Can't approve")))
            }
            Ok(())
        }
    }
}
