
use anchor_lang::prelude::*;
use anchor_spl::token::*;

pub use crate::state::*;
pub use ::lockup::Vesting;

#[derive(Accounts)]
pub struct DepositLocked<'info> {
    // Lockup whitelist relay interface.
    #[account(
        "vesting.to_account_info().owner == &registry.lockup_program",
        "vesting.beneficiary == member.beneficiary"
    )]
    vesting: Box<Account<'info, Vesting>>,
    #[account(mut, "vesting_vault.key == &vesting.vault")]
    vesting_vault: AccountInfo<'info>,
    // Note: no need to verify the depositor_authority since the SPL program
    //       will fail the transaction if it's not correct.
    #[account(signer)]
    depositor_authority: AccountInfo<'info>,
    #[account("token_program.key == &anchor_spl::token::ID")]
    token_program: AccountInfo<'info>,
    #[account(
        mut,
        "member_vault.to_account_info().key == &member.balances_locked.vault"
    )]
    member_vault: Box<Account<'info, TokenAccount>>,
    #[account(
        seeds = [
            registrar.to_account_info().key.as_ref(),
            member.to_account_info().key.as_ref(),
            &[member.nonce],
        ],
        bump
    )]
    member_signer: AccountInfo<'info>,

    // Program specific.
    registry: Box<Account<'info, Registry>>,
    registrar: Box<Account<'info, Registrar>>,
    #[account(has_one = registrar, has_one = beneficiary)]
    member: Box<Account<'info, Member>>,
    #[account(signer)]
    beneficiary: AccountInfo<'info>,
}


impl<'a, 'b, 'c, 'info> From<&mut DepositLocked<'info>>
    for CpiContext<'a, 'b, 'c, 'info, Transfer<'info>>
{
    fn from(accounts: &mut DepositLocked<'info>) -> CpiContext<'a, 'b, 'c, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: accounts.vesting_vault.clone(),
            to: accounts.member_vault.to_account_info(),
            authority: accounts.depositor_authority.clone(),
        };
        let cpi_program = accounts.token_program.clone();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}



// Deposits that can only come from the beneficiary's vesting accounts.
pub fn handler(ctx: Context<DepositLocked>, amount: u64) -> Result<()> {
    transfer(ctx.accounts.into(), amount).map_err(Into::into)
}