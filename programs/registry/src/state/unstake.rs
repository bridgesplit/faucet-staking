use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct StartUnstake<'info> {
    // Stake instance globals.
    #[account(has_one = reward_event_q, has_one = pool_mint)]
    registrar: ProgramAccount<'info, Registrar>,
    reward_event_q: ProgramAccount<'info, RewardQueue>,
    #[account(mut)]
    pool_mint: AccountInfo<'info>,

    // Member.
    #[account(init)]
    pending_withdrawal: ProgramAccount<'info, PendingWithdrawal>,
    #[account(has_one = beneficiary, has_one = registrar)]
    member: ProgramAccount<'info, Member>,
    #[account(signer)]
    beneficiary: AccountInfo<'info>,
    #[account("BalanceSandbox::from(&balances) == member.balances")]
    balances: BalanceSandboxAccounts<'info>,
    #[account("BalanceSandbox::from(&balances_locked) == member.balances_locked")]
    balances_locked: BalanceSandboxAccounts<'info>,

    // Programmatic signers.
    #[account(
        seeds = [
            registrar.to_account_info().key.as_ref(),
            member.to_account_info().key.as_ref(),
            &[member.nonce],
        ]
    )]
    member_signer: AccountInfo<'info>,

    // Misc.
    #[account("token_program.key == &token::ID")]
    token_program: AccountInfo<'info>,
    clock: Sysvar<'info, Clock>,
    rent: Sysvar<'info, Rent>,
}

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