## Overview

This example shows how you can use the implementation of
[access-control](contracts/access/access-control) and
[psp721](contracts/token/psp721) together to provide rights to mint and burn NFT tokens.

## Steps

1. You need to include `psp721`, `access-control` and `brush` in cargo file.

```markdown
[dependencies]
...

psp721 = { version = "0.3.0-rc1", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }
access-control = { version = "0.3.0-rc1", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }
brush = { version = "0.3.0-rc1", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }

[features]
default = ["std"]
std = [
 ...
   
   "psp721/std",
   "access-control/std",
   "brush/std",
]
```

2. To declare the contract you need to use `brush::contract` macro instead of `ink::contract`. Import **everything**
   from according trait modules.

```rust
#[brush::contract]
pub mod my_access_control {
    use psp721::traits::*;
    use access_control::traits::*;
```

3. Declare storage struct and declare the fields related to `PSP721Storage` and `AccessControlStorage`
   traits. Then you need to derive `PSP721Storage` and `AccessControlStorage` traits and mark according fields
   with `#[PSP721StorageField]` and `#[AccessControlStorageField]` attributes. Deriving these traits allow you to reuse
   the default implementation of `IPSP721` and `AccessControl`.

```rust
#[ink(storage)]
#[derive(Default, PSP721Storage, AccessControlStorage)]
pub struct PSP721Struct {
    #[PSP721StorageField]
    psp721: PSP721Data,
    #[AccessControlStorageField]
    access: AccessControlData,
}
```

4. After that you can inherit implementation of `IPSP721` and `AccessControl` traits. You can customize(override) some
   methods there.

```rust
impl IPSP721 for PSP721Struct {}

impl AccessControl for PSP721Struct {}
```

5. Now you only need to define constructor and your basic version of `IPSP721` contract is ready.

```rust
impl PSP721Struct {
    #[ink(constructor)]
    pub fn new() -> Self {
        Self::default()
    }
}
```

6. Let's customize it. We will implement `IPSP721Mint` trait. It will use modifier `only_minter`(it verifies that caller
   has minter role). Also, we need to update constructor to grant minter role to caller by default.

```rust
// ::ink_lang_ir::Selector::new("MINTER".as_ref()).as_bytes()
const MINTER: RoleType = 0xfd9ab216;

#[brush::modifier_definition]
pub fn only_minter<T: AccessControl>(instance: &mut T, body: impl FnOnce(&mut T)) {
    instance._check_role(&MINTER, &T::env().caller());
    body(instance)
}

impl PSP721Struct {
    #[ink(constructor)]
    pub fn new() -> Self {
        let mut instance = Self::default();
        let caller = instance.env().caller();
        instance._init_with_admin(caller);
        // We grant minter role to caller in constructor, so he can mint/burn tokens
        instance.grant_role(MINTER, caller);
        instance
    }
}

impl IPSP721 for PSP721Struct {}

impl AccessControl for PSP721Struct {}

impl IPSP721Mint for PSP721Struct {
    #[ink(message)]
    #[modifiers(only_minter)]
    fn mint(&mut self, id: Id) {
        self._mint(id);
    }

    #[ink(message)]
    #[modifiers(only_minter)]
    fn burn(&mut self, id: Id) {
        self._burn(id);
    }
}
```