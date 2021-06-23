use ::ink_env::{
    Environment,
    DefaultEnvironment,
};

pub type AccountId = <DefaultEnvironment as Environment>::AccountId;
pub type Balance = <DefaultEnvironment as Environment>::Balance;
pub type EnvAccess = ::ink_lang::EnvAccess<'static, DefaultEnvironment>;

pub trait InkStorage {
    fn env() -> EnvAccess {
        Default::default()
    }
}