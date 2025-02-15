use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct StakingConfig {
    pub authority: Pubkey,

    pub reward_start_time: u64,
    pub reward_end_time: u64,
    pub reward_start_machine_count_threshold: u64,
    pub total_distributed_reward_amount: u64,
    pub init_reward_amount: u64,
    pub total_machine_calc_point: u64,
    pub total_machine_count: u64,
    pub total_stake_coin_amount: u64,

    pub reward_token_account: Pubkey,

    pub reward_token_mint: Pubkey,
    pub stake_token_mint: Pubkey,
    pub nft_mint: Pubkey,

    pub reward_token_vault: Pubkey,
    pub staked_token_vault: Pubkey,
    pub staked_nft_vault: Pubkey,

    pub bump_reward_token_vault: u8,
    pub bump_staked_token_vault: u8,
    pub bump_staked_nft_vault: u8,

    pub bump: u8,
}
