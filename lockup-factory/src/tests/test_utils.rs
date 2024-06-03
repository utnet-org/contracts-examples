use std::convert::TryInto;
use unc_sdk::{env, AccountId};

pub const GENESIS_TIME_IN_DAYS: u64 = 500;
pub const YEAR: u64 = 365;
pub const SALT: [u8; 3] = [1, 2, 3];

pub fn to_ts(num_days: u64) -> u64 {
    // 2018-08-01 UTC in nanoseconds
    1533081600_000_000_000 + to_nanos(num_days)

pub fn to_nanos(num_days: u64) -> u64 {
    num_days * 86400_000_000_000
}

pub fn account_unc() -> AccountId {
    "testnet".to_string()
}

pub fn account_factory() -> AccountId {
    "lockup.testnet".to_string()
}

pub fn lockup_master_account_id() -> AccountId {
    "lockup.testnet".try_into().unwrap()
}

pub fn whitelist_account_id() -> AccountId {
    "whitelist.testnet".try_into().unwrap()
}

pub fn custom_whitelist_account_id() -> AccountId {
    "custom.whitelist.testnet".try_into().unwrap()
}

pub fn foundation_account_id() -> AccountId {
    "testnet".try_into().unwrap()
}

pub fn account_tokens_owner() -> AccountId {
    "tokenowner.testnet".try_into().unwrap()
}

pub fn ntoy(unc_amount: Balance) -> Balance {
    unc_amount * 10u128.pow(24)
}

pub fn lockup_account() -> AccountId {
    let byte_slice = env::sha256(account_tokens_owner().as_ref().as_bytes());
    let lockup_account_id = format!(
        "{}.{}",
        hex::encode(&byte_slice[..20]),
        &lockup_master_account_id().as_ref().to_string()
    );
    return lockup_account_id;
}