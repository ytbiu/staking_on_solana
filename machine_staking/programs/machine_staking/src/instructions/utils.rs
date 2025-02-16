use anchor_lang::prelude::*;

use anchor_spl::{
    token_2022::Token2022,
    token_interface::{burn, transfer_checked, Burn, Mint, TokenAccount, TransferChecked},
};

pub fn transfer<'info>(
    from: &InterfaceAccount<'info, TokenAccount>,
    to: &InterfaceAccount<'info, TokenAccount>,
    mint: &InterfaceAccount<'info, Mint>,
    amount: u64,
    signer: &Signer<'info>,
    token_program: &Program<'info, Token2022>,
) -> Result<()> {
    let transfer_cpi_accounts = TransferChecked {
        from: from.to_account_info(),
        to: to.to_account_info(),
        mint: mint.to_account_info(),
        authority: signer.to_account_info(),
    };
    transfer_checked(
        CpiContext::new(token_program.to_account_info(), transfer_cpi_accounts),
        amount,
        mint.decimals,
    )?;
    Ok(())
}

pub fn transfer_with_seed<'info, 'a, 'b, 'c>(
    from: &InterfaceAccount<'info, TokenAccount>,
    to: &InterfaceAccount<'info, TokenAccount>,
    mint: &InterfaceAccount<'info, Mint>,
    amount: u64,
    signer: &InterfaceAccount<'info, TokenAccount>,
    token_program: &Program<'info, Token2022>,
    signer_seeds: &'a [&'b [&'c [u8]]],
) -> Result<()> {
    let transfer_cpi_accounts = TransferChecked {
        from: from.to_account_info(),
        to: to.to_account_info(),
        authority: signer.to_account_info(),
        mint: mint.to_account_info(),
    };

    transfer_checked(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            transfer_cpi_accounts,
            signer_seeds,
        ),
        amount,
        mint.decimals,
    )?;
    Ok(())
}

pub fn burn_token<'info>(
    from: &InterfaceAccount<'info, TokenAccount>,
    mint: &InterfaceAccount<'info, Mint>,
    amount: u64,
    signer: &Signer<'info>,
    token_program: &Program<'info, Token2022>,
) -> Result<()> {
    let burn_cpi_accounts = Burn {
        from: from.to_account_info(),
        mint: mint.to_account_info(),
        authority: signer.to_account_info(),
    };
    burn(
        CpiContext::new(token_program.to_account_info(), burn_cpi_accounts),
        amount,
    )?;
    Ok(())
}
