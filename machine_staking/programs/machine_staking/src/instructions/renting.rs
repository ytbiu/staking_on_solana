use anchor_lang::prelude::*;

use crate::error::ErrorCode;
use crate::{
    burn_token, RentInfo, StakingConfig, UserStakeInfo, ANCHOR_PREFIX, RENT_FEE_PER_SECOND,
    SEED_RENT_INFO, SEED_STAKING_CONFIG, SEED_USER_STAKE,
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, Token2022, TokenAccount},
};

#[derive(Accounts)]
#[instruction(machine_id: String)]
pub struct RentMachine<'info> {
    #[account(mut)]
    signer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = signer,
        space = ANCHOR_PREFIX + RentInfo::INIT_SPACE,
        seeds = [SEED_RENT_INFO, signer.key.as_ref(), machine_id.as_bytes()],
        bump,
    )]
    pub rent_info: Account<'info, RentInfo>,

    #[account(
        seeds = [SEED_USER_STAKE, signer.key.as_ref(), machine_id.as_bytes()],
        bump = user_stake_info.bump,
    )]
    pub user_stake_info: Account<'info, UserStakeInfo>,

    #[account(mint::token_program = token_program)]
    pub fee_token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        associated_token::mint = fee_token_mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub fee_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [SEED_STAKING_CONFIG],
        bump = staking_config.bump,
        constraint = staking_config.reward_token_mint == fee_token_mint.key(),
    )]
    pub staking_config: Account<'info, StakingConfig>,

    system_program: Program<'info, System>,
    token_program: Program<'info, Token2022>,
    associated_token_program: Program<'info, AssociatedToken>,
}

pub fn rent_machine(
    ctx: Context<RentMachine>,
    machine_id: String,
    rent_seconds: i64,
    rent_fee: u64,
) -> Result<()> {
    let expected_rent_fee = get_rent_fee(rent_seconds);
    require!(expected_rent_fee == rent_fee, ErrorCode::InvalidRentFee);

    let user_stake_info: &mut Account<'_, UserStakeInfo> = &mut ctx.accounts.user_stake_info;
    require!(
        user_stake_info.is_rented == false,
        ErrorCode::MachineRentingByOthers
    );

    let now = Clock::get()?.unix_timestamp;
    let rent_end_time = now + rent_seconds;
    require!(
        rent_end_time < user_stake_info.end_time,
        ErrorCode::MachineHasEndedStaking
    );
    ctx.accounts.rent_info.set_inner(RentInfo {
        renter: ctx.accounts.signer.key(),
        machine_id: machine_id,
        rent_start_time: now,
        rent_end_time: rent_end_time,
    });

    user_stake_info.is_rented = true;
    let old_calc_point = user_stake_info.calc_point;
    // add 30% calc_point
    user_stake_info.calc_point = old_calc_point.saturating_mul(13).saturating_div(10);

    ctx.accounts.staking_config.total_machine_calc_point = ctx
        .accounts
        .staking_config
        .total_machine_calc_point
        .saturating_sub(old_calc_point)
        .saturating_add(user_stake_info.calc_point);

    burn_token(
        &ctx.accounts.fee_token_account,
        &ctx.accounts.fee_token_mint,
        rent_fee,
        &ctx.accounts.signer,
        &ctx.accounts.token_program,
    )?;

    Ok(())
}

fn get_rent_fee(rent_seconds: i64) -> u64 {
    RENT_FEE_PER_SECOND.saturating_mul(rent_seconds as u64)
}

#[derive(Accounts)]
#[instruction(machine_id: String)]
pub struct EndRentMachine<'info> {
    #[account(mut)]
    signer: Signer<'info>,

    #[account(
        seeds = [SEED_RENT_INFO, signer.key.as_ref(), machine_id.as_bytes()],
        bump,
    )]
    pub rent_info: Account<'info, RentInfo>,

    #[account(
        seeds = [SEED_USER_STAKE, signer.key.as_ref(), machine_id.as_bytes()],
        bump = user_stake_info.bump,
    )]
    pub user_stake_info: Account<'info, UserStakeInfo>,

    #[account(
        mut,
        seeds = [SEED_STAKING_CONFIG],
        bump = staking_config.bump,
    )]
    pub staking_config: Account<'info, StakingConfig>,

    system_program: Program<'info, System>,
    token_program: Program<'info, Token2022>,
}

pub fn end_rent_machine(ctx: Context<RentMachine>, _machine_id: String) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    let rent_info = &mut ctx.accounts.rent_info;
    require!(
        rent_info.rent_end_time <= now,
        ErrorCode::InvalidEndRentTime
    );

    let user_stake_info = &mut ctx.accounts.user_stake_info;

    user_stake_info.is_rented = false;
    let old_calc_point = user_stake_info.calc_point;
    // sub 30% calc_point
    user_stake_info.calc_point = old_calc_point.saturating_mul(10).saturating_div(13);

    ctx.accounts.staking_config.total_machine_calc_point = ctx
        .accounts
        .staking_config
        .total_machine_calc_point
        .saturating_sub(old_calc_point)
        .saturating_add(user_stake_info.calc_point);
    Ok(())
}
