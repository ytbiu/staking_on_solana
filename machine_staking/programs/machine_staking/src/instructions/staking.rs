use anchor_lang::prelude::*;

use crate::{
     StakingConfig, UserStakeInfo, ANCHOR_PREFIX, SEED_REWARD_TOKEN_MINT, SEED_STAKING_CONFIG, SEED_USER_STAKE,SEED_STAKED_TOKEN_POOL,SEED_STAKED_NFT_POOL
};
use anchor_spl::token_interface::{Mint, Token2022,TokenAccount,transfer_checked,TransferChecked};

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [SEED_STAKING_CONFIG],
        bump,
        has_one = reward_token_mint_account,
        has_one = nft_mint_account,
    )]
    pub config_account: Account<'info, StakingConfig>,

    #[account(
        init_if_needed,
        payer = authority,
        space = ANCHOR_PREFIX + UserStakeInfo::INIT_SPACE,
        seeds = [SEED_USER_STAKE, authority.key().as_ref()],
        bump,
    )]
    pub user_stake_info: Account<'info, UserStakeInfo>,

    #[account(
        seeds = [SEED_REWARD_TOKEN_MINT],
        bump,
    )]
    pub reward_token_mint_account: InterfaceAccount<'info, Mint>,

    #[account(
        seeds = [SEED_REWARD_TOKEN_MINT],
        bump,
    )]
    pub nft_mint_account: InterfaceAccount<'info, Mint>,

    #[account( 
        mut,
        associated_token::mint = reward_token_mint_account, 
        associated_token::authority = authority,
        associated_token::token_program = token_program,
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>, 

    #[account( 
        mut,
        associated_token::mint = nft_mint_account, 
        associated_token::authority = authority,
        associated_token::token_program = token_program,
    )]
    pub user_nft_token_account: InterfaceAccount<'info, TokenAccount>, 

    #[account(
        seeds = [SEED_STAKED_TOKEN_POOL],
        bump,
        token::mint = reward_token_mint_account,
        token::authority = token_staked_pool_account,
    )]
    pub token_staked_pool_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds = [SEED_STAKED_NFT_POOL],
        bump,
        token::mint = nft_mint_account,
        token::authority = nft_staked_pool_account,     
    )]
    pub nft_staked_pool_account: InterfaceAccount<'info, TokenAccount>,

    
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

pub fn stake(ctx: Context<Stake>,machine_id: String, staked_token_amount: u64, staked_nft_amount: u64,stake_seconds: i64) -> Result<()> {
    let user_stake_info = &mut ctx.accounts.user_stake_info;
    let clock = Clock::get()?;
    user_stake_info.set_inner(UserStakeInfo {
        authority: *ctx.accounts.authority.key,
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
        nft_token_account: ctx.accounts.user_nft_token_account.key(),
        reward_token_account: ctx.accounts.user_token_account.key(),
    });

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let transfer_cpi_accounts = TransferChecked {
        from: ctx.accounts.user_token_account.to_account_info(),
        mint: ctx.accounts.reward_token_mint_account.to_account_info(),
        to: ctx.accounts.token_staked_pool_account.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    let cpi_context = CpiContext::new(cpi_program, transfer_cpi_accounts);

    transfer_checked(cpi_context, staked_token_amount, ctx.accounts.reward_token_mint_account.decimals)?;
      

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let transfer_cpi_accounts = TransferChecked {
        from: ctx.accounts.user_nft_token_account.to_account_info(),
        mint: ctx.accounts.nft_mint_account.to_account_info(),
        to: ctx.accounts.nft_staked_pool_account.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    let cpi_context = CpiContext::new(cpi_program, transfer_cpi_accounts);

    transfer_checked(cpi_context, staked_token_amount, ctx.accounts.reward_token_mint_account.decimals)?;
    Ok(())
        
}