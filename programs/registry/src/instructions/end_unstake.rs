#[derive(Accounts)]
pub struct EndUnstake<'info> {
    registrar: ProgramAccount<'info, Registrar>,

    #[account(has_one = registrar, has_one = beneficiary)]
    member: ProgramAccount<'info, Member>,
    #[account(signer)]
    beneficiary: AccountInfo<'info>,
    #[account(mut, has_one = registrar, has_one = member, "!pending_withdrawal.burned")]
    pending_withdrawal: ProgramAccount<'info, PendingWithdrawal>,

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
            &[member.nonce],
        ]
    )]
    member_signer: AccountInfo<'info>,

    clock: Sysvar<'info, Clock>,
    #[account("token_program.key == &token::ID")]
    token_program: AccountInfo<'info>,
}

pub fn handler(ctx: Context<EndUnstake>) -> Result<()> {
    if ctx.accounts.pending_withdrawal.end_ts > ctx.accounts.clock.unix_timestamp {
        return Err(ErrorCode::UnstakeTimelock.into());
    }

    // Select which balance set this affects.
    let balances = {
        if ctx.accounts.pending_withdrawal.locked {
            &ctx.accounts.member.balances_locked
        } else {
            &ctx.accounts.member.balances
        }
    };
    // Check the vaults given are corrrect.
    if &balances.vault != ctx.accounts.vault.key {
        return Err(ErrorCode::InvalidVault.into());
    }
    if &balances.vault_pw != ctx.accounts.vault_pw.key {
        return Err(ErrorCode::InvalidVault.into());
    }

    // Transfer tokens between vaults.
    {
        let seeds = &[
            ctx.accounts.registrar.to_account_info().key.as_ref(),
            ctx.accounts.member.to_account_info().key.as_ref(),
            &[ctx.accounts.member.nonce],
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
        token::transfer(cpi_ctx, ctx.accounts.pending_withdrawal.amount)?;
    }

    // Burn the pending withdrawal receipt.
    let pending_withdrawal = &mut ctx.accounts.pending_withdrawal;
    pending_withdrawal.burned = true;

    Ok(())
}