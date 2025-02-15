use anchor_lang::prelude::*;

use anchor_spl::{
    token_2022::Token2022,
    token_interface::{transfer_checked, Mint, TokenAccount, TransferChecked},
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
