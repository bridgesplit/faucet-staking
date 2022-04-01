
use anchor_lang::prelude::*;
use anchor_spl::token::*;

use crate::{state::*, errors::*, utils::*};

#[derive(Accounts)]
pub struct EndUnstake<'info> {
    registrar: Account<'info, Registrar>,

    #[account(has_one = registrar, has_one = beneficiary)]
    member: Box<Account<'info, Member>>,
    #[account(signer)]
    beneficiary: AccountInfo<'info>,
    #[account(mut, has_one = registrar, has_one = member, 
        constraint = !pending_withdrawal.burned)]
    pending_withdrawal: Box<Account<'info, PendingWithdrawal>>,

    // If we had ordered maps implementing Accounts we could do a constraint like
    // balances.get(pending_withdrawal.balance_id).vault == vault.key.
    //
    // Note: we do the constraints check in the handler, not here.
    #[account(mut)]
    vault: AccountInfo<'info>,
    #[account(mut)]
    vault_pw: AccountInfo<'info>,

    #[account(
        seeds = [
            registrar.to_account_info().key.as_ref(),
            member.to_account_info().key.as_ref(),
            SIGNER_SEED
        ],
        bump
    )]
    member_signer: AccountInfo<'info>,

    clock: Sysvar<'info, Clock>,
    #[account("token_program.key == &anchor_spl::token::ID")]
    token_program: AccountInfo<'info>,
}

pub fn handler(ctx: Context<EndUnstake>) -> Result<()> {
    if ctx.accounts.pending_withdrawal.end_ts > ctx.accounts.clock.unix_timestamp {
        return Err(CustomErrorCode::UnstakeTimelock.into());
    }

    // Select which balance set this affects.
    let [memVault, memVaultPw] = {
        if ctx.accounts.pending_withdrawal.locked {
            [ ctx.accounts.member.locked_vault,
             ctx.accounts.member.locked_vault_pw
            ]
        } else {
            [ ctx.accounts.member.vault,
             ctx.accounts.member.vault_pw
            ]
        }
    };
    // Check the vaults given are corrrect.
    if memVault != *ctx.accounts.vault.key {
        return Err(CustomErrorCode::InvalidVault.into());
    }
    if memVaultPw != *ctx.accounts.vault_pw.key {
        return Err(CustomErrorCode::InvalidVault.into());
    }

    // Transfer tokens between vaults.
    {
        let seeds = &[
            ctx.accounts.registrar.to_account_info().key.as_ref(),
            ctx.accounts.member.to_account_info().key.as_ref(),
            &SIGNER_SEED[..],
            &get_bump_in_seed_form(ctx.bumps.get("member_signer").unwrap())
        ];
        let signer = &[&seeds[..]];
         let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.clone(),
            Transfer {
                from: ctx.accounts.vault_pw.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
                authority: ctx.accounts.member_signer.clone(),
            },
            signer,
        );
        transfer(cpi_ctx, ctx.accounts.pending_withdrawal.amount)?;
    }

    // Burn the pending withdrawal receipt.
    let pending_withdrawal = &mut ctx.accounts.pending_withdrawal;
    pending_withdrawal.burned = true;

    Ok(())
}