use anchor_lang::prelude::*;
use anchor_spl::token::*;

use crate::state::*; 
use crate::utils::*;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    // Stake instance.
    registrar: Box<Account<'info, Registrar>>,
    // Member.
    #[account(has_one = registrar, has_one = beneficiary)]
    member: Box<Account<'info, Member>>,
    #[account(signer)]
    beneficiary: AccountInfo<'info>,
    #[account(mut,
        constraint = vault.to_account_info().key == &member.vault)]
    vault: Box<Account<'info, TokenAccount>>,
    #[account(
        seeds = [
            registrar.to_account_info().key.as_ref(),
            member.to_account_info().key.as_ref(),
            SIGNER_SEED
        ],
        bump
    )]
    member_signer: AccountInfo<'info>,
    // Receiver.
    #[account(mut)]
    depositor: AccountInfo<'info>,
    // Misc.
    #[account(
        constraint = token_program.key == &anchor_spl::token::ID
    )]
    token_program: AccountInfo<'info>,
}


pub fn handler(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let seeds = &[
        ctx.accounts.registrar.to_account_info().key.as_ref(),
        ctx.accounts.member.to_account_info().key.as_ref(),
        &SIGNER_SEED[..],
        &get_bump_in_seed_form(ctx.bumps.get("member_signer").unwrap())
    ];
    let signer = &[&seeds[..]];
    let cpi_accounts = Transfer {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.depositor.to_account_info(),
        authority: ctx.accounts.member_signer.clone(),
    };
    let cpi_program = ctx.accounts.token_program.clone();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

    transfer(cpi_ctx, amount).map_err(Into::into)

}