#![feature(min_specialization)]
#[cfg(feature = "psp22")]
#[brush::contract]
mod psp22 {
    use brush::test_utils::*;
    use contracts::psp22::*;
    use ink::{
        EmitEvent,
        Env,
    };
    use ink_lang as ink;
    use std::panic;

    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    /// Event emitted when an approval occurs that `spender` is allowed to withdraw
    /// up to the amount of `value` tokens from `owner`.
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    /// A simple PSP-20 contract.
    #[ink(storage)]
    #[derive(Default, PSP22Storage)]
    pub struct PSP22Struct {
        #[PSP22StorageField]
        psp22: PSP22Data,
    }

    type Event = <PSP22Struct as ::ink_lang::BaseEvent>::Type;

    impl PSP22Internal for PSP22Struct {
        fn _emit_transfer_event(&self, _from: Option<AccountId>, _to: Option<AccountId>, _amount: Balance) {
            self.env().emit_event(Transfer {
                from: _from,
                to: _to,
                value: _amount,
            });
        }

        fn _emit_approval_event(&self, _owner: AccountId, _spender: AccountId, _amount: Balance) {
            self.env().emit_event(Approval {
                owner: _owner,
                spender: _spender,
                value: _amount,
            });
        }

        // Override this function with an empty body to omit error (cross-contract calls are not supported in off-chain environment)
        fn _do_safe_transfer_check(
            &mut self,
            _from: AccountId,
            _to: AccountId,
            _value: Balance,
            _data: Vec<u8>,
        ) -> Result<(), PSP22Error> {
            Ok(())
        }
    }

    impl PSP22 for PSP22Struct {}

    impl PSP22Struct {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut instance = Self::default();
            assert!(instance._mint(instance.env().caller(), total_supply).is_ok());
            instance
        }
    }

    fn assert_transfer_event(
        event: &ink_env::test::EmittedEvent,
        expected_from: Option<AccountId>,
        expected_to: Option<AccountId>,
        expected_value: Balance,
    ) {
        let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..])
            .expect("encountered invalid contract event data buffer");
        if let Event::Transfer(Transfer { from, to, value }) = decoded_event {
            assert_eq!(from, expected_from, "encountered invalid Transfer.from");
            assert_eq!(to, expected_to, "encountered invalid Transfer.to");
            assert_eq!(value, expected_value, "encountered invalid Trasfer.value");
        } else {
            panic!("encountered unexpected event kind: expected a Transfer event")
        }
        let expected_topics = vec![
            encoded_into_hash(&PrefixedValue {
                value: b"PSP22Struct::Transfer",
                prefix: b"",
            }),
            encoded_into_hash(&PrefixedValue {
                prefix: b"PSP22Struct::Transfer::from",
                value: &expected_from,
            }),
            encoded_into_hash(&PrefixedValue {
                prefix: b"PSP22Struct::Transfer::to",
                value: &expected_to,
            }),
            encoded_into_hash(&PrefixedValue {
                prefix: b"PSP22Struct::Transfer::value",
                value: &expected_value,
            }),
        ];
        for (n, (actual_topic, expected_topic)) in event.topics.iter().zip(expected_topics).enumerate() {
            let topic = actual_topic
                .decode::<Hash>()
                .expect("encountered invalid topic encoding");
            assert_eq!(topic, expected_topic, "encountered invalid topic at {}", n);
        }
    }

    /// The default constructor does its job.
    #[ink::test]
    fn new_works() {
        // Constructor works.
        let _psp22 = PSP22Struct::new(100);

        // Transfer event triggered during initial construction.
        let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
        assert_eq!(1, emitted_events.len());

        assert_transfer_event(&emitted_events[0], None, Some(AccountId::from([0x01; 32])), 100);
    }

    /// The total supply was applied.
    #[ink::test]
    fn total_supply_works() {
        // Constructor works.
        let psp22 = PSP22Struct::new(100);
        // Transfer event triggered during initial construction.
        let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
        assert_transfer_event(&emitted_events[0], None, Some(AccountId::from([0x01; 32])), 100);
        // Get the token total supply.
        assert_eq!(psp22.total_supply(), 100);
    }

    /// Get the actual balance of an account.
    #[ink::test]
    fn balance_of_works() {
        // Constructor works
        let psp22 = PSP22Struct::new(100);
        // Transfer event triggered during initial construction
        let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
        assert_transfer_event(&emitted_events[0], None, Some(AccountId::from([0x01; 32])), 100);
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
        // Alice owns all the tokens on deployment
        assert_eq!(psp22.balance_of(accounts.alice), 100);
        // Bob does not owns tokens
        assert_eq!(psp22.balance_of(accounts.bob), 0);
    }

    #[ink::test]
    fn transfer_works() {
        // Constructor works.
        let mut psp22 = PSP22Struct::new(100);
        let accounts = accounts();

        assert_eq!(psp22.balance_of(accounts.bob), 0);
        // Alice transfers 10 tokens to Bob.
        assert!(psp22.transfer(accounts.bob, 10, Vec::<u8>::new()).is_ok());
        // Bob owns 10 tokens.
        assert_eq!(psp22.balance_of(accounts.bob), 10);

        let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
        assert_eq!(emitted_events.len(), 2);
        // Check first transfer event related to PSP-20 instantiation.
        assert_transfer_event(&emitted_events[0], None, Some(AccountId::from([0x01; 32])), 100);
        // Check the second transfer event relating to the actual trasfer.
        assert_transfer_event(
            &emitted_events[1],
            Some(AccountId::from([0x01; 32])),
            Some(AccountId::from([0x02; 32])),
            10,
        );
    }

    #[ink::test]
    fn invalid_transfer_should_fail() {
        // Constructor works.
        let mut psp22 = PSP22Struct::new(100);
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");

        assert_eq!(psp22.balance_of(accounts.bob), 0);
        change_caller(accounts.bob);

        // Bob fails to transfers 10 tokens to Eve.
        assert_eq!(
            psp22.transfer(accounts.eve, 10, Vec::<u8>::new()),
            Err(PSP22Error::InsufficientBalance)
        );
    }

    #[ink::test]
    fn transfer_from_fails() {
        // Constructor works.
        let mut psp22 = PSP22Struct::new(100);
        // Transfer event triggered during initial construction.
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");

        // Bob fails to transfer tokens owned by Alice.
        assert_eq!(
            psp22.transfer_from(accounts.alice, accounts.eve, 10, Vec::<u8>::new()),
            Err(PSP22Error::InsufficientAllowance)
        );
    }

    #[ink::test]
    fn transfer_from_works() {
        // Constructor works.
        let mut psp22 = PSP22Struct::new(100);
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");

        // Alice approves Bob for token transfers on her behalf.
        assert!(psp22.approve(accounts.bob, 10).is_ok());

        // The approve event takes place.
        assert_eq!(ink_env::test::recorded_events().count(), 2);

        change_caller(accounts.bob);

        // Bob transfers tokens from Alice to Eve.
        assert!(psp22
            .transfer_from(accounts.alice, accounts.eve, 10, Vec::<u8>::new())
            .is_ok());
        // Eve owns tokens.
        assert_eq!(psp22.balance_of(accounts.eve), 10);

        // Check all transfer events that happened during the previous calls:
        let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
        assert_eq!(emitted_events.len(), 4);
        assert_transfer_event(&emitted_events[0], None, Some(AccountId::from([0x01; 32])), 100);
        // The second event `emitted_events[1]` is an Approve event that we skip checking.
        assert_transfer_event(
            &emitted_events[2],
            Some(AccountId::from([0x01; 32])),
            Some(AccountId::from([0x05; 32])),
            10,
        );
    }

    #[ink::test]
    fn allowance_must_not_change_on_failed_transfer() {
        let mut psp22 = PSP22Struct::new(100);
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");

        // Alice approves Bob for token transfers on her behalf.
        let alice_balance = psp22.balance_of(accounts.alice);
        let initial_allowance = alice_balance + 2;
        assert!(psp22.approve(accounts.bob, initial_allowance).is_ok());
        change_caller(accounts.bob);

        assert_eq!(
            psp22.transfer_from(accounts.alice, accounts.eve, alice_balance + 1, Vec::<u8>::new()),
            Err(PSP22Error::InsufficientBalance)
        );
    }
}