use anchor_lang::prelude::*;
use anchor_spl::token::*;


use crate::{state::*, errors::*, claim_reward::*, stake::*};


#[derive(Accounts)]
pub struct StartUnstake<'info> {
    // Stake instance globals.
    #[account(has_one = reward_event_q, has_one = pool_mint)]
    registrar: Account<'info, Registrar>,
    reward_event_q: Account<'info, RewardQueue>,
    #[account(mut)]
    pool_mint: AccountInfo<'info>,
    // Member.
    #[account(init,
    payer = beneficiary,
    space = std::mem::size_of::<PendingWithdrawal>())]
    pending_withdrawal: Account<'info, PendingWithdrawal>,
    #[account(has_one = beneficiary, has_one = registrar)]
    member:Account<'info, Member>,
    #[account(mut)]
    beneficiary: Signer<'info>,
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
        ],
        bump
    )]
    member_signer: AccountInfo<'info>,

    // Misc.
    #[account("token_program.key == &anchor_spl::token::ID")]
    token_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    clock: Sysvar<'info, Clock>,
    rent: Sysvar<'info, Rent>,
}
    
#[access_control(no_available_rewards(
    &ctx.accounts.reward_event_q,
    &ctx.accounts.member,
    &ctx.accounts.balances,
     &ctx.accounts.balances_locked,
 ))]
pub fn handler(ctx: Context<StartUnstake>, spt_amount: u64, locked: bool) -> Result<()> {
    let balances = {
        if locked {
            &ctx.accounts.balances_locked
        } else {
            &ctx.accounts.balances
        }
    };

    // Program signer.
    let seeds = &[
        ctx.accounts.registrar.to_account_info().key.as_ref(),
        ctx.accounts.member.to_account_info().key.as_ref(),
        &[ctx.accounts.member.nonce],
    ];
    let member_signer = &[&seeds[..]];

    // Burn pool tokens.
    {
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.clone(),
            Burn {
                mint: ctx.accounts.pool_mint.to_account_info(),
                to: balances.spt.to_account_info(),
                authority: ctx.accounts.member_signer.to_account_info(),
            },
            member_signer,
        );
        burn(cpi_ctx, spt_amount)?;
    }

        // Convert from stake-token units to mint-token units.
    let token_amount = spt_amount
        .checked_mul(ctx.accounts.registrar.stake_rate)
        .unwrap();

        // Transfer tokens from the stake to pending vault.
        {
            let cpi_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.clone(),
                Transfer {
                    from: balances.vault_stake.to_account_info(),
                    to: balances.vault_pw.to_account_info(),
                    authority: ctx.accounts.member_signer.to_account_info(),
                },
                member_signer,
            );
            transfer(cpi_ctx, token_amount)?;
        }

        // Print receipt.
        let pending_withdrawal = &mut ctx.accounts.pending_withdrawal;
        pending_withdrawal.burned = false;
        pending_withdrawal.member = *ctx.accounts.member.to_account_info().key;
        pending_withdrawal.start_ts = ctx.accounts.clock.unix_timestamp;
        pending_withdrawal.end_ts =
            ctx.accounts.clock.unix_timestamp + ctx.accounts.registrar.withdrawal_timelock;
        pending_withdrawal.amount = token_amount;
        pending_withdrawal.pool = ctx.accounts.registrar.pool_mint;
        pending_withdrawal.registrar = *ctx.accounts.registrar.to_account_info().key;
        pending_withdrawal.locked = locked;

        // Update stake timestamp.
        let member = &mut ctx.accounts.member;
        member.last_stake_ts = ctx.accounts.clock.unix_timestamp;

        Ok(())
}