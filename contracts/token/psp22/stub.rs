// It exports the stub implementation of all PSP22 traits.
// ink! will generate a wrapper around all methods of each trait and it will allow creating wrapped
// struct around contracts address(::ink_env::call::FromAccountId::from_account_id).
pub use self::{psp22::PSP22, psp22metadata::PSP22Metadata};

pub use self::psp22receiver::PSP22Receiver;

#[ink_lang::contract(compile_as_dependency = true)]
mod psp22 {
    use ink_prelude::vec::Vec;

    #[derive(Default)]
    #[ink(storage)]
    pub struct PSP22 {}

    impl PSP22 {
        #[ink(constructor)]
        pub fn new() -> Self {
            unimplemented!()
        }
    }

    #[ink(namespace = "PSP22")]
    impl PSP22 {
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            unimplemented!()
        }

        #[ink(message)]
        pub fn balance_of(&self, _owner: AccountId) -> Balance {
            unimplemented!()
        }

        #[ink(message)]
        pub fn transfer(&mut self, _to: AccountId, _value: Balance, _data: Vec<u8>) {
            unimplemented!()
        }

        #[ink(message)]
        pub fn allowance(&self, _owner: AccountId, _spender: AccountId) -> Balance {
            unimplemented!()
        }

        #[ink(message)]
        pub fn transfer_from(&mut self, _from: AccountId, _to: AccountId, _value: Balance, _data: Vec<u8>) {
            unimplemented!()
        }

        #[ink(message)]
        pub fn approve(&mut self, _spender: AccountId, _value: Balance) {
            unimplemented!()
        }

        #[ink(message)]
        pub fn increase_allowance(&mut self, _spender: AccountId, _delta_value: Balance) {
            unimplemented!()
        }

        #[ink(message)]
        pub fn decrease_allowance(&mut self, _spender: AccountId, _delta_value: Balance) {
            unimplemented!()
        }
    }
}

#[ink_lang::contract(compile_as_dependency = true)]
mod psp22metadata {
    use ink_prelude::string::String;

    #[derive(Default)]
    #[ink(storage)]
    pub struct PSP22Metadata {}

    impl PSP22Metadata {
        #[ink(constructor)]
        pub fn new() -> Self {
            unimplemented!()
        }
    }

    #[ink(namespace = "PSP22Metadata")]
    impl PSP22Metadata {
        #[ink(message)]
        pub fn token_name(&self) -> Option<String> {
            unimplemented!()
        }

        #[ink(message)]
        pub fn token_symbol(&self) -> Option<String> {
            unimplemented!()
        }

        #[ink(message)]
        pub fn token_decimals(&self) -> u8 {
            unimplemented!()
        }
    }
}

#[ink_lang::contract(compile_as_dependency = true)]
mod psp22wrapped {
    #[derive(Default)]
    #[ink(storage)]
    pub struct PSP22Wrapped {}

    impl PSP22Wrapped {
        #[ink(constructor)]
        pub fn new() -> Self {
            unimplemented!()
        }
    }

    #[ink(namespace = "PSP22Wrapped")]
    impl PSP22Wrapped {
        #[ink(message)]
        pub fn underlying_token(&self) -> AccountId {
            unimplemented!()
        }
    }
}

/// The user has to define their own Receiver contract with custom funds acceptance logic.
#[ink_lang::contract(compile_as_dependency = true)]
pub mod psp22receiver {
    use crate::traits::PSP22ReceiverError;
    use ink_prelude::vec::Vec;

    #[derive(Default)]
    #[ink(storage)]
    pub struct PSP22Receiver {}

    #[ink(namespace = "PSP22Receiver")]
    impl PSP22Receiver {
        #[ink(constructor)]
        pub fn new() -> Self {
            unimplemented!()
        }

        #[ink(message)]
        pub fn before_received(
            &mut self,
            operator: AccountId,
            from: AccountId,
            value: Balance,
            data: Vec<u8>,
        ) -> Result<(), PSP22ReceiverError> {
            unimplemented!()
        }
    }
}

#[ink_lang::contract(compile_as_dependency = true)]
pub mod psp22mintable {

    #[derive(Default)]
    #[ink(storage)]
    pub struct PSP22Mintable {}

    #[ink(namespace = "PSP22Mintable")]
    impl PSP22Mintable {
        #[ink(constructor)]
        pub fn new() -> Self {
            unimplemented!()
        }

        #[ink(message)]
        pub fn mint(&mut self, amount: Balance) {
            unimplemented!()
        }
    }
}

#[ink_lang::contract(compile_as_dependency = true)]
pub mod psp22burnable {

    #[derive(Default)]
    #[ink(storage)]
    pub struct PSP22Burnable {}

    #[ink(namespace = "PSP22Burnable")]
    impl PSP22Burnable {
        #[ink(constructor)]
        pub fn new() -> Self {
            unimplemented!()
        }

        #[ink(message)]
        pub fn burn(&mut self, amount: Balance) {
            unimplemented!()
        }

        #[ink(message)]
        pub fn burn_from(&mut self, account: AccountId, amount: Balance) {
            unimplemented!()
        }
    }
}

#[ink_lang::contract(compile_as_dependency = true)]
pub mod psp22wrapper {

    #[derive(Default)]
    #[ink(storage)]
    pub struct PSP22Wrapper {}

    #[ink(namespace = "PSP22Burnable")]
    impl PSP22Wrapper {
        #[ink(constructor)]
        pub fn new() -> Self {
            unimplemented!()
        }

        #[ink(message)]
        pub fn deposit_for(&mut self, account: AccountId, amount: Balance) {
            unimplemented!()
        }

        #[ink(message)]
        pub fn withdraw_to(&mut self, account: AccountId, amount: Balance) {
            unimplemented!()
        }

        pub fn _recover(&mut self, account: AccountId) {
            unimplemented!()
        }
    }
}