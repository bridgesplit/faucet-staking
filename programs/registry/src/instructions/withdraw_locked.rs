use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct WithdrawLocked<'info> {
    // Lockup whitelist relay interface.
    #[account(
        "vesting.to_account_info().owner == &registry.lockup_program",
        "vesting.beneficiary == member.beneficiary"
    )]
    vesting: CpiAccount<'info, Vesting>,
    #[account(mut, "vesting_vault.key == &vesting.vault")]
    vesting_vault: AccountInfo<'info>,
    #[account(signer)]
    vesting_signer: AccountInfo<'info>,
    #[account("token_program.key == &token::ID")]
    token_program: AccountInfo<'info>,
    #[account(
        mut,
        "member_vault.to_account_info().key == &member.balances_locked.vault"
    )]
    member_vault: CpiAccount<'info, TokenAccount>,
    #[account(
        seeds = [
            registrar.to_account_info().key.as_ref(),
            member.to_account_info().key.as_ref(),
            &[member.nonce],
        ]
    )]
    member_signer: AccountInfo<'info>,

    // Program specific.
    registry: ProgramState<'info, Registry>,
    registrar: ProgramAccount<'info, Registrar>,
    #[account(has_one = registrar, has_one = beneficiary)]
    member: ProgramAccount<'info, Member>,
    #[account(signer)]
    beneficiary: AccountInfo<'info>

}



pub fn handler(ctx: Context<WithdrawLocked>, amount: u64) -> Result<()> {
        let seeds = &[
            ctx.accounts.registrar.to_account_info().key.as_ref(),
            ctx.accounts.member.to_account_info().key.as_ref(),
            &[ctx.accounts.member.nonce],
        ];
        let signer = &[&seeds[..]];
        let cpi_accounts = Transfer {
            from: ctx.accounts.member_vault.to_account_info(),
            to: ctx.accounts.vesting_vault.to_account_info(),
            authority: ctx.accounts.member_signer.clone(),
        };
        let cpi_program = ctx.accounts.token_program.clone();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

        token::transfer(cpi_ctx, amount).map_err(Into::into)
    }
