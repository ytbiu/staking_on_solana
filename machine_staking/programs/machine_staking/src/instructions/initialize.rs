use anchor_lang::prelude::*;

use crate::{
    StakingConfig, ANCHOR_PREFIX, REWARD_TOKEN_MINT_DECIMALS, SEED_REWARD_TOKEN_MINT,
    SEED_STAKED_NFT_POOL, SEED_STAKED_TOKEN_POOL, SEED_STAKING_CONFIG,
};
use anchor_spl::{token_interface::{transfer_checked,Mint, Token2022,TokenAccount,TransferChecked}};

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
    pub config_account: Account<'info, StakingConfig>,

    #[account(
        init_if_needed,
        payer = signer,
        seeds = [SEED_REWARD_TOKEN_MINT],
        bump,
        mint::decimals = REWARD_TOKEN_MINT_DECIMALS,
        mint::authority = reward_token_mint_account,
        mint::freeze_authority = reward_token_mint_account,
        mint::token_program = token_program
    )]
    pub reward_token_mint_account: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = signer,
        seeds = [SEED_REWARD_TOKEN_MINT],
        bump,
        mint::decimals = 0,
        mint::authority = nft_mint_account,
        mint::freeze_authority = nft_mint_account,
        mint::token_program = token_program
    )]
    pub nft_mint_account: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = signer,
        seeds = [SEED_STAKED_TOKEN_POOL],
        bump,
        token::mint = reward_token_mint_account,
        token::authority = token_staked_pool_account,
    )]
    pub token_staked_pool_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = signer,
        seeds = [SEED_STAKED_NFT_POOL],
        bump,
        token::mint = nft_mint_account,
        token::authority = nft_staked_pool_account,     
    )]
    pub nft_staked_pool_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = signer,
        seeds = [SEED_STAKED_TOKEN_POOL],
        bump,
        token::mint = reward_token_mint_account,
        token::authority = token_staked_pool_account,
    )]
    pub token_reward_pool_account: InterfaceAccount<'info, TokenAccount>,

    #[account( 
        mut,
        associated_token::mint = reward_token_mint_account, 
        associated_token::authority = signer,
        associated_token::token_program = token_program,
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>, 


    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

pub fn init(ctx: Context<Initialize>,reward_amount: u64) -> Result<()> {
    let config_account = &mut ctx.accounts.config_account;
    config_account.set_inner(StakingConfig {
        reward_start_time: 0,
        reward_end_time: 0,
        reward_start_machine_count_threshold: 10,
        total_distributed_reward_amount: 0,
        init_reward_amount: 0,
        total_machine_calc_point: 0,
        total_machine_count: 0,
        total_stake_coin_amount: 0,

        reward_token_mint_account: ctx.accounts.reward_token_mint_account.key(),
        nft_mint_account: ctx.accounts.nft_mint_account.key(),
        token_staked_pool_account: ctx.accounts.token_staked_pool_account.key(),
        nft_staked_pool_account: ctx.accounts.nft_staked_pool_account.key(),
        token_reward_pool_account :ctx.accounts.reward_token_mint_account.key(),

        bump_reward_token_mint_account: ctx.bumps.reward_token_mint_account,
        bump_nft_mint_account: ctx.bumps.nft_mint_account,
        bump_nft_staked_pool_account: ctx.bumps.nft_staked_pool_account,
        bump_token_staked_pool_account: ctx.bumps.nft_staked_pool_account,
        bump_token_reward_pool_account: ctx.bumps.token_reward_pool_account,
    });

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let transfer_cpi_accounts = TransferChecked {
        from: ctx.accounts.user_token_account.to_account_info(),
        mint: ctx.accounts.reward_token_mint_account.to_account_info(),
        to: ctx.accounts.token_reward_pool_account.to_account_info(),
        authority: ctx.accounts.signer.to_account_info(),
    };
    let cpi_context = CpiContext::new(cpi_program, transfer_cpi_accounts);
      

    transfer_checked(
        cpi_context,
        reward_amount,
        ctx.accounts.reward_token_mint_account.decimals,
    )?;

    Ok(())
}

