use anchor_lang::prelude::*;

use crate::{
     StakingConfig, UserStakeInfo, ANCHOR_PREFIX,
      SEED_STAKING_CONFIG, SEED_USER_STAKE,SEED_STAKED_TOKEN_VAULT,
      SEED_STAKED_NFT_VAULT
};
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, Token2022,TokenAccount}};
use super::transfer;

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
    set_user_stake_info(&mut ctx, machine_id, staked_token_amount, staked_nft_amount, stake_seconds)?;
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

fn set_user_stake_info(ctx: &mut Context<Stake>, machine_id: String, staked_token_amount: u64, staked_nft_amount: u64, stake_seconds: i64) -> Result<()> {
    let clock = Clock::get()?;
    let user_stake_info = &mut ctx.accounts.user_stake_info;
    user_stake_info.set_inner(UserStakeInfo {
        authority: *ctx.accounts.signer.key,
        machine_id,
        start_time: clock.unix_timestamp,
        end_time: clock.unix_timestamp + stake_seconds,
        staked_token_amount,
        staked_nft_amount,
        locked_claimed_reward: 0,
        locked_time: 0,
        unlocked_time:0,
        total_locked_reward: 0,
        total_claimed_reward:0,
        nft_token_account: ctx.accounts.nft_token_account.key(),
        reward_token_account: ctx.accounts.reward_token_account.key(),
    });

    Ok(())
}