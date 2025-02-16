use std::{cmp::{max, min}, u64};
use anchor_lang::prelude::*;

use crate::{
    transfer_with_seed, StakingConfig, UserStakeInfo, ANCHOR_PREFIX, BASE_CALC_POINT, MAX_NFT_COUNT_PER_MACHINE, REWARD_DURATION, SEED_REWARD_TOKEN_VAULT, SEED_STAKED_NFT_VAULT, SEED_STAKED_TOKEN_VAULT, SEED_STAKING_CONFIG, SEED_USER_STAKE
};
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, Token2022,TokenAccount}};
use super::transfer;
use crate::error::ErrorCode;

#[derive(Accounts)]
#[instruction(machine_id: String)]
pub struct Stake<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [SEED_STAKING_CONFIG],
        bump = staking_config.bump,
        has_one = reward_token_account,
        has_one = nft_mint,
        has_one = stake_token_mint
    )]
    pub staking_config: Account<'info, StakingConfig>,

    #[account(
        init_if_needed,
        payer = signer,
        space = ANCHOR_PREFIX + UserStakeInfo::INIT_SPACE,
        seeds = [SEED_USER_STAKE, signer.key.as_ref(), machine_id.as_bytes()],
        bump,
    )]
    pub user_stake_info: Account<'info, UserStakeInfo>,

    #[account(mint::token_program = token_program)]
    pub reward_token_mint: InterfaceAccount<'info, Mint>,
    #[account(mint::token_program = token_program)]
    pub stake_token_mint: InterfaceAccount<'info, Mint>,
    #[account(mint::token_program = token_program)]
    pub nft_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = reward_token_mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub reward_token_account: InterfaceAccount<'info, TokenAccount>,


    #[account( 
        mut,
        associated_token::mint = stake_token_account, 
        associated_token::authority = signer,
        associated_token::token_program = token_program,
    )]
    pub stake_token_account: InterfaceAccount<'info, TokenAccount>, 

    #[account( 
        mut,
        associated_token::mint = nft_mint, 
        associated_token::authority = signer,
        associated_token::token_program = token_program,
    )]
    pub nft_token_account: InterfaceAccount<'info, TokenAccount>, 

    #[account(
        seeds = [SEED_STAKED_TOKEN_VAULT],
        bump = staking_config.bump_staked_token_vault,
        token::mint = stake_token_account,
        token::authority = staked_token_vault,
    )]
    pub staked_token_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds = [SEED_STAKED_NFT_VAULT],
        bump = staking_config.bump_staked_nft_vault,
        token::mint = nft_mint,
        token::authority = staked_nft_vault,     
    )]
    pub staked_nft_vault: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn stake(mut ctx: Context<Stake>,machine_id: String, staked_token_amount: u64, staked_nft_amount: u64,stake_seconds: i64) -> Result<()> {
    require!(staked_nft_amount >MAX_NFT_COUNT_PER_MACHINE, ErrorCode::InvalidNFTCount);
    let calc_point = calculate_calc_point(staked_token_amount, staked_nft_amount); 
    let config = &mut ctx.accounts.staking_config;

    config.total_machine_calc_point = config.total_machine_calc_point.saturating_add(calc_point);
    config.total_stake_coin_amount = config.total_stake_coin_amount.saturating_add(staked_token_amount);
    config.total_machine_count = config.total_machine_count.saturating_add(1);

    if config.total_machine_count >= config.reward_start_machine_count_threshold{
        config.reward_start_time = Clock::get()?.unix_timestamp as u64;
        config.reward_end_time = config.reward_start_time + REWARD_DURATION;

    }

    set_user_stake_info(&mut ctx, machine_id, staked_token_amount, staked_nft_amount, stake_seconds,calc_point)?;
    transfer(
        &ctx.accounts.stake_token_account,
        &ctx.accounts.staked_token_vault,
        &ctx.accounts.stake_token_mint,
        staked_token_amount,
        &ctx.accounts.signer,
        &ctx.accounts.token_program,
    )?;

    transfer(
        &ctx.accounts.nft_token_account,
        &ctx.accounts.staked_nft_vault,
        &ctx.accounts.nft_mint,
        staked_nft_amount,
        &ctx.accounts.signer,
        &ctx.accounts.token_program,
    )?;
    Ok(())
        
}

fn set_user_stake_info(ctx: &mut Context<Stake>, machine_id: String, staked_token_amount: u64, staked_nft_amount: u64, stake_seconds: i64,calc_point: u64) -> Result<()> {
    let clock = Clock::get()?;
    let user_stake_info = &mut ctx.accounts.user_stake_info;
    
    user_stake_info.set_inner(UserStakeInfo {
        authority: *ctx.accounts.signer.key,
        machine_id,
        start_time: clock.unix_timestamp,
        end_time: clock.unix_timestamp + stake_seconds,
        last_claim_time: clock.unix_timestamp,
        staked_token_amount,
        staked_nft_amount,
        locked_claimed_reward: 0,
        locked_time: 0,
        unlocked_time:0,
        is_rented:false,
        calc_point:calc_point,
        total_locked_reward: 0,
        total_claimed_reward:0,
        stake_token_account: ctx.accounts.stake_token_account.key(),
        nft_token_account: ctx.accounts.nft_token_account.key(),
        reward_token_account: ctx.accounts.reward_token_account.key(),
        bump: ctx.bumps.user_stake_info,
    });



    Ok(())
}

fn calculate_calc_point(staked_token_amount: u64, staked_nft_amount: u64) -> u64 {
    let v = staked_token_amount.saturating_div(BASE_CALC_POINT);
    BASE_CALC_POINT.saturating_mul(staked_nft_amount.saturating_add(v))
}

#[derive(Accounts)]
#[instruction(machine_id: String)]
pub struct Claim<'info>{
    #[account(mut)]
    signer: Signer<'info>,

    #[account(
        seeds = [SEED_USER_STAKE, signer.key.as_ref(), machine_id.as_bytes()],
        bump = user_stake_info.bump,
        has_one = reward_token_account
    )]
    pub user_stake_info: Account<'info, UserStakeInfo>,

    #[account(mint::token_program = token_program)]
    pub reward_token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        associated_token::mint = reward_token_mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub reward_token_account: InterfaceAccount<'info, TokenAccount>,
    
    #[account(
        mut,
        seeds = [SEED_STAKING_CONFIG],
        bump = staking_config.bump,
        has_one = reward_token_mint,
    )]
    pub staking_config: Account<'info, StakingConfig>,

    #[account(
        seeds = [SEED_REWARD_TOKEN_VAULT],
        bump = staking_config.bump_reward_token_vault,
    )]
    pub reward_token_vault: InterfaceAccount<'info, TokenAccount>,


    system_program: Program<'info, System>,
    token_program: Program<'info, Token2022>,
    associated_token_program: Program<'info, AssociatedToken>,
}

pub fn claim(mut ctx:  Context<Claim>) -> Result<()> {
    if let Some(reward_amount)=  get_reward_amount(&mut ctx){
        let now = Clock::get()?.unix_timestamp;
        let lock_reward = get_lock_reward(reward_amount);
        let released_reward = get_total_release_reward_after_lock(&mut ctx,lock_reward,now as u64);
        let avaliable_reward = released_reward.saturating_sub(lock_reward).saturating_add(released_reward);
        let user_stake_info = &mut ctx.accounts.user_stake_info;


        ctx.accounts.staking_config.total_distributed_reward_amount = ctx.accounts.staking_config.total_distributed_reward_amount.saturating_add(reward_amount);
        user_stake_info.last_claim_time = now;
        user_stake_info.total_claimed_reward = user_stake_info.total_claimed_reward.saturating_add(reward_amount);
        user_stake_info.locked_claimed_reward = user_stake_info.locked_claimed_reward.saturating_add(reward_amount);
        

        let seeds = &[
            SEED_REWARD_TOKEN_VAULT,
            &[ctx.accounts.staking_config.bump_reward_token_vault],
        ];
        let signer_seeds = [&seeds[..]];

        transfer_with_seed(
            &ctx.accounts.reward_token_vault,
                &ctx.accounts.reward_token_account,
                &ctx.accounts.reward_token_mint, 
                avaliable_reward,
                &ctx.accounts.reward_token_vault,
                &ctx.accounts.token_program,
                &signer_seeds,
            )?
    }

    Ok(())
}

pub fn get_reward_amount(ctx: &mut Context<Claim>) -> Option<u64>{
    let start_time = ctx.accounts.user_stake_info.start_time as u64;
    let end_time = ctx.accounts.user_stake_info.end_time as u64;

    let reward_start_time = ctx.accounts.staking_config.reward_start_time;
    let reward_end_time = ctx.accounts.staking_config.reward_end_time;

    let start_at = max(start_time, reward_start_time);
    let mut end_at = min(end_time, reward_end_time);

    let now = Clock::get().unwrap().unix_timestamp as u64;
    end_at = min(now ,end_at);

    let reward_duration = end_at.checked_sub(start_at)?;

    let reward_amount_per_second = ctx.accounts.staking_config.init_reward_amount.checked_div(REWARD_DURATION)?;

    reward_duration.checked_mul(reward_amount_per_second)
}

fn get_total_release_reward_after_lock(ctx: &mut Context<Claim>,lock_reward: u64, now: u64) -> u64{
    let user_stake_info = &mut ctx.accounts.user_stake_info;
    user_stake_info.total_locked_reward = user_stake_info.total_locked_reward.saturating_add(lock_reward);

    let released_reward = (user_stake_info.total_locked_reward.saturating_mul(now as u64-user_stake_info.locked_time as u64)).
    saturating_div(
        (user_stake_info.unlocked_time as u64).saturating_sub(user_stake_info.locked_time as u64)
    ).saturating_sub(user_stake_info.total_claimed_reward); 

    user_stake_info.total_claimed_reward = user_stake_info.total_claimed_reward.saturating_add(released_reward);

    released_reward
}

fn get_lock_reward(reward_amount: u64) -> u64{
    reward_amount*1/10
}
#[derive(Accounts)]
#[instruction(machine_id: String)]
pub struct EndStake<'info>{
    #[account(mut)]
    signer: Signer<'info>,

    #[account(
        seeds = [SEED_USER_STAKE, signer.key.as_ref(), machine_id.as_bytes()],
        bump = user_stake_info.bump,
        has_one = stake_token_account,
        has_one = nft_token_account
    )]
    pub user_stake_info: Account<'info, UserStakeInfo>,

    #[account(mint::token_program = token_program)]
    pub stake_token_mint: InterfaceAccount<'info, Mint>,

    #[account(mint::token_program = token_program)]
    pub nft_mint: InterfaceAccount<'info, Mint>,

    #[account(
        associated_token::mint = stake_token_mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub stake_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        associated_token::mint = nft_mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub nft_token_account: InterfaceAccount<'info, TokenAccount>,
    
    #[account(
        mut,
        seeds = [SEED_STAKING_CONFIG],
        bump = staking_config.bump,
        has_one = stake_token_mint,
    )]
    pub staking_config: Account<'info, StakingConfig>,

    #[account(
        seeds = [SEED_STAKED_TOKEN_VAULT],
        bump = staking_config.bump_staked_token_vault,
    )]
    pub staked_token_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds = [SEED_STAKED_NFT_VAULT],
        bump = staking_config.bump_staked_nft_vault,
    )]
    pub staked_nft_vault: InterfaceAccount<'info, TokenAccount>,

    system_program: Program<'info, System>,
    token_program: Program<'info, Token2022>,
    associated_token_program: Program<'info, AssociatedToken>,
}

pub fn end_stake(ctx: Context<EndStake>) -> Result<()> {
    let now = Clock::get().unwrap().unix_timestamp;
    let user_stake_info = &mut ctx.accounts.user_stake_info;
    require!(now as u64 > user_stake_info.end_time as u64, ErrorCode::InvalidStakeEndTime);

    let config = &mut ctx.accounts.staking_config;
    config.total_machine_calc_point = config.total_machine_calc_point.saturating_sub(user_stake_info.staked_token_amount);
    config.total_machine_count = config.total_machine_count.saturating_sub(config.total_machine_count);
    config.total_stake_coin_amount = config.total_stake_coin_amount.saturating_sub(user_stake_info.staked_token_amount);
    
    let seeds = &[
        SEED_STAKED_TOKEN_VAULT,
        &[ctx.accounts.staking_config.bump_staked_token_vault],
    ];
    let signer_seeds = [&seeds[..]];

    transfer_with_seed(
        &ctx.accounts.staked_token_vault,
        &ctx.accounts.stake_token_account,
        &ctx.accounts.stake_token_mint,
        user_stake_info.staked_token_amount,
        &ctx.accounts.staked_token_vault,
        &ctx.accounts.token_program,
        &signer_seeds,
    )?;

    transfer_with_seed(
        &ctx.accounts.staked_nft_vault,
        &ctx.accounts.nft_token_account,
        &ctx.accounts.nft_mint,
        user_stake_info.staked_nft_amount,
        &ctx.accounts.staked_nft_vault,
        &ctx.accounts.token_program,
        &signer_seeds,
    )?;
   
    Ok(())
}
