use anchor_lang::prelude::*;
use anchor_spl::token::*;

use crate::state::*;

#[derive(Accounts)]
pub struct Deposit<'info> {
    // Member.
    #[account(has_one = beneficiary)]
    member: Account<'info, Member>,
    #[account(signer)]
    beneficiary: AccountInfo<'info>,
    #[account(mut, constraint = vault.to_account_info().key == &member.vault)]
    vault: Account<'info, TokenAccount>,
    // Depositor.
    #[account(mut)]
    depositor: AccountInfo<'info>,
    #[account(signer, constraint = depositor_authority.key == &member.beneficiary)]
    depositor_authority: AccountInfo<'info>,
    // Misc.
    #[account(constraint = token_program.key == &anchor_spl::token::ID)]
    token_program: AccountInfo<'info>,
}

impl<'a, 'b, 'c, 'info> From<&mut Deposit<'info>>
    for CpiContext<'a, 'b, 'c, 'info, Transfer<'info>>
{
    fn from(accounts: &mut Deposit<'info>) -> CpiContext<'a, 'b, 'c, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: accounts.depositor.clone(),
            to: accounts.vault.to_account_info(),
            authority: accounts.depositor_authority.clone(),
        };
        let cpi_program = accounts.token_program.clone();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}


// Deposits that can only come directly from the member beneficiary.
pub fn handler(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    transfer(ctx.accounts.into(), amount).map_err(Into::into)
}
