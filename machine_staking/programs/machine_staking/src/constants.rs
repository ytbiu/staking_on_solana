use anchor_lang::prelude::*;

pub const ANCHOR_PREFIX: usize = 8;

#[constant]
pub const SEED_STAKING_CONFIG: &[u8] = b"staking_config";
pub const SEED_REWARD_TOKEN_MINT: &[u8] = b"reward_token_mint";
pub const SEED_USER_STAKE: &[u8] = b"user_stake";
pub const SEED_REWARD_TOKEN_POOL: &[u8] = b"reward_token_pool";
pub const SEED_STAKED_TOKEN_POOL: &[u8] = b"staked_token_pool";
pub const SEED_STAKED_NFT_POOL: &[u8] = b"staked_nft_pool";
pub const REWARD_TOKEN_MINT_DECIMALS: u8 = 9;
