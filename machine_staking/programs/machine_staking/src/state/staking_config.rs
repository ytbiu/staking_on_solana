use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct StakingConfig {
    pub reward_start_time: u64,
    pub reward_end_time: u64,
    pub reward_start_machine_count_threshold: u64,
    pub total_distributed_reward_amount: u64,
    pub init_reward_amount: u64,
    pub total_machine_calc_point: u64,
    pub total_machine_count: u64,
    pub total_stake_coin_amount: u64,

    pub reward_token_mint_account: Pubkey,
    pub nft_mint_account: Pubkey,
    pub token_staked_pool_account: Pubkey,
    pub nft_staked_pool_account: Pubkey,
    pub token_reward_pool_account: Pubkey,

    pub bump_reward_token_mint_account: u8,
    pub bump_nft_mint_account: u8,
    pub bump_token_staked_pool_account: u8,
    pub bump_nft_staked_pool_account: u8,
    pub bump_token_reward_pool_account: u8,
}
