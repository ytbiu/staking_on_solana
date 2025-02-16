use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct UserStakeInfo {
    pub authority: Pubkey,

    #[max_len(50)]
    pub machine_id: String,

    pub start_time: i64,
    pub end_time: i64,
    pub last_claim_time: i64,

    pub calc_point: u64,
    pub is_rented: bool,
    pub staked_token_amount: u64,
    pub staked_nft_amount: u64,
    pub total_claimed_reward: u64,

    pub locked_time: u64,
    pub unlocked_time: u64,
    pub total_locked_reward: u64,
    pub locked_claimed_reward: u64,

    pub reward_token_account: Pubkey,
    pub stake_token_account: Pubkey,
    pub nft_token_account: Pubkey,

    pub bump: u8,
}
