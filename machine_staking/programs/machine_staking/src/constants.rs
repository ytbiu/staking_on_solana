use anchor_lang::prelude::*;

pub const ANCHOR_PREFIX: usize = 8;

#[constant]
pub const ONE_DAY: u64 = 60 * 60 * 24;
pub const REWARD_DURATION: u64 = 60 * ONE_DAY;

pub const RENT_FEE_PER_SECOND: u64 = 10;
pub const BASE_CALC_POINT: u64 = 1000;

pub const MAX_NFT_COUNT_PER_MACHINE: u64 = 20;

pub const SEED_STAKING_CONFIG: &[u8] = b"staking_config";
pub const SEED_USER_STAKE: &[u8] = b"user_stake";
pub const SEED_RENT_INFO: &[u8] = b"rent_info";

pub const SEED_REWARD_TOKEN_VAULT: &[u8] = b"reward_token_vault";
pub const SEED_STAKED_TOKEN_VAULT: &[u8] = b"staked_token_vault";
pub const SEED_STAKED_NFT_VAULT: &[u8] = b"staked_nft_vault";
