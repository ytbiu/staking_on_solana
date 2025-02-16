pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("C2SqF49RUjhavh6g4bQBXQmChYc2GS9ogdsVU6vG6xiU");

#[program]
pub mod machine_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, reward_amount: u64) -> Result<()> {
        initialize::init(ctx, reward_amount)
    }

    pub fn user_stake(
        ctx: Context<Stake>,
        machine_id: String,
        staked_token_amount: u64,
        staked_nft_amount: u64,
        stake_seconds: i64,
    ) -> Result<()> {
        staking::stake(
            ctx,
            machine_id,
            staked_token_amount,
            staked_nft_amount,
            stake_seconds,
        )
    }

    pub fn user_unstake(ctx: Context<EndStake>) -> Result<()> {
        staking::end_stake(ctx)
    }

    pub fn user_claim(ctx: Context<Claim>) -> Result<()> {
        staking::claim(ctx)
    }

    pub fn renter_rent_machine(
        ctx: Context<RentMachine>,
        machine_id: String,
        rent_seconds: i64,
        rent_fee: u64,
    ) -> Result<()> {
        renting::rent_machine(ctx, machine_id, rent_seconds, rent_fee)
    }

    pub fn renter_end_rent_machine(ctx: Context<RentMachine>, machine_id: String) -> Result<()> {
        renting::end_rent_machine(ctx, machine_id)
    }
}
