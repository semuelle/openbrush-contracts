/// Extension of [`PSP1155`] that allows token holders to destroy their tokens
use crate::traits::psp1155::Id;
use crate::traits::psp1155::PSP1155Error;
use brush::traits::{
    AccountId,
    Balance,
};
use ink_prelude::vec::Vec;

#[brush::wrapper]
pub type PSP1155BurnableRef = dyn PSP1155Burnable;

#[brush::trait_definition]
pub trait PSP1155Burnable {
    /// Destroys `amount` tokens of token type `id` from `from`
    ///
    /// See [`PSP1155::_burn_from`].
    #[ink(message)]
    fn burn(&mut self, from: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP1155Error>;
}
