use anchor_lang::prelude::*;


#[derive(Accounts)]
pub struct Deposit<'info> {
    // Member.
    #[account(has_one = beneficiary)]
    member: ProgramAccount<'info, Member>,
    #[account(signer)]
    beneficiary: AccountInfo<'info>,
    #[account(mut, "vault.to_account_info().key == &member.balances.vault")]
    vault: CpiAccount<'info, TokenAccount>,
    // Depositor.
    #[account(mut)]
    depositor: AccountInfo<'info>,
    #[account(signer, "depositor_authority.key == &member.beneficiary")]
    depositor_authority: AccountInfo<'info>,
    // Misc.
    #[account("token_program.key == &token::ID")]
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
    token::transfer(ctx.accounts.into(), amount).map_err(Into::into)
}
