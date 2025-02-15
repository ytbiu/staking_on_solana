use anchor_lang::prelude::*;

use crate::{
    StakingConfig, ANCHOR_PREFIX, SEED_REWARD_TOKEN_VAULT, SEED_STAKED_NFT_VAULT, SEED_STAKED_TOKEN_VAULT, SEED_STAKING_CONFIG
};
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, Token2022,TokenAccount}};
use super::transfer;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = signer,
        space = ANCHOR_PREFIX + StakingConfig::INIT_SPACE,
        seeds = [SEED_STAKING_CONFIG],
        bump,
    )]
    pub staking_config: Account<'info, StakingConfig>,

    #[account(mint::token_program = token_program)]
    pub reward_token_mint: InterfaceAccount<'info, Mint>,

    #[account(mint::token_program = token_program)]
    pub stake_token_mint: InterfaceAccount<'info, Mint>,

    #[account(mint::token_program = token_program)]
    pub nft_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = signer,
        seeds = [SEED_STAKED_TOKEN_VAULT],
        bump,
        token::mint = reward_token_account,
        token::authority = staked_token_vault,
    )]
    pub staked_token_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = signer,
        seeds = [SEED_STAKED_NFT_VAULT],
        bump,
        token::mint = nft_mint,
        token::authority = staked_nft_vault,     
    )]
    pub staked_nft_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = signer,
        seeds = [SEED_REWARD_TOKEN_VAULT],
        bump,
        token::mint = reward_token_account,
        token::authority = reward_token_vault,
    )]
    pub reward_token_vault: InterfaceAccount<'info, TokenAccount>,

    #[account( 
        mut,
        associated_token::mint = reward_token_mint, 
        associated_token::authority = signer,
        associated_token::token_program = token_program,
    )]
    pub reward_token_account: InterfaceAccount<'info, TokenAccount>, 


    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,

}

pub fn init(ctx: Context<Initialize>,reward_amount: u64) -> Result<()> {
    let config_account = &mut ctx.accounts.staking_config;
    config_account.set_inner(StakingConfig {
        authority:ctx.accounts.signer.key(),
        reward_start_time: 0,
        reward_end_time: 0,
        reward_start_machine_count_threshold: 10,
        total_distributed_reward_amount: 0,
        init_reward_amount: 0,
        total_machine_calc_point: 0,
        total_machine_count: 0,
        total_stake_coin_amount: 0,

        reward_token_account: ctx.accounts.reward_token_account.key(),

        reward_token_mint: ctx.accounts.reward_token_mint.key(),
        stake_token_mint: ctx.accounts.stake_token_mint.key(),
        nft_mint: ctx.accounts.nft_mint.key(),

        staked_token_vault: ctx.accounts.staked_token_vault.key(),
        staked_nft_vault: ctx.accounts.staked_nft_vault.key(),
        reward_token_vault :ctx.accounts.reward_token_mint.key(),

        bump_staked_nft_vault: ctx.bumps.staked_nft_vault,
        bump_staked_token_vault: ctx.bumps.staked_token_vault,
        bump_reward_token_vault: ctx.bumps.reward_token_vault,
        bump: ctx.bumps.staking_config,
    });

    transfer(
        &ctx.accounts.reward_token_account,
        &ctx.accounts.reward_token_vault,
        &ctx.accounts.reward_token_mint,
        reward_amount,
        &ctx.accounts.signer,
        &ctx.accounts.token_program,
    )?;

    Ok(())
}

