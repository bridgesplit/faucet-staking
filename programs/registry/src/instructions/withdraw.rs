use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    // Stake instance.
    registrar: ProgramAccount<'info, Registrar>,
    // Member.
    #[account(has_one = registrar, has_one = beneficiary)]
    member: ProgramAccount<'info, Member>,
    #[account(signer)]
    beneficiary: AccountInfo<'info>,
    #[account(mut, "vault.to_account_info().key == &member.balances.vault")]
    vault: CpiAccount<'info, TokenAccount>,
    #[account(
        seeds = [
            registrar.to_account_info().key.as_ref(),
            member.to_account_info().key.as_ref(),
            &[member.nonce],
        ]
    )]
    member_signer: AccountInfo<'info>,
    // Receiver.
    #[account(mut)]
    depositor: AccountInfo<'info>,
    // Misc.
    #[account("token_program.key == &token::ID")]
    token_program: AccountInfo<'info>,
}


pub fn handler(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let seeds = &[
        ctx.accounts.registrar.to_account_info().key.as_ref(),
        ctx.accounts.member.to_account_info().key.as_ref(),
        &[ctx.accounts.member.nonce],
    ];
    let signer = &[&seeds[..]];
    let cpi_accounts = Transfer {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.depositor.to_account_info(),
        authority: ctx.accounts.member_signer.clone(),
    };
    let cpi_program = ctx.accounts.token_program.clone();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

    token::transfer(cpi_ctx, amount).map_err(Into::into)

}