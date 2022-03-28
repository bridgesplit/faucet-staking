use anchor_lang::prelude::*;
use anchor_spl::token::*;

use crate::{state::*, errors::*, claim_reward::*};



#[derive(Accounts)]
pub struct Stake<'info> {
    // Global accounts for the staking instance.
    #[account(has_one = pool_mint, has_one = reward_event_q)]
    registrar: Box<Account<'info, Registrar>>,
    reward_event_q: Box<Account<'info, RewardQueue>>,
    #[account(mut)]
    pool_mint: Box<Account<'info, Mint>>,

    // Member.
    #[account(mut, has_one = beneficiary, has_one = registrar)]
    member: Box<Account<'info, Member>>,
    #[account(signer)]
    beneficiary: AccountInfo<'info>,
    #[account("BalanceSandbox::from(&balances) == member.balances")]
    balances: BalanceSandboxAccounts<'info>,
    #[account("BalanceSandbox::from(&balances_locked) == member.balances_locked")]
    balances_locked: BalanceSandboxAccounts<'info>,

    // Program signers.
    #[account(
        seeds = [
            registrar.to_account_info().key.as_ref(),
            member.to_account_info().key.as_ref(),
            &[member.nonce],
        ],
        bump
    )]
    member_signer: AccountInfo<'info>,
    #[account(seeds = [registrar.to_account_info().key.as_ref(), &[registrar.nonce]],
    bump)]
    registrar_signer: AccountInfo<'info>,

    // Misc.
    clock: Sysvar<'info, Clock>,
    #[account("token_program.key == &anchor_spl::token::ID")]
    token_program: AccountInfo<'info>,
}




// Asserts the user calling the `Stake` instruction has no rewards available
// in the reward queue.
pub fn no_available_rewards<'info>(
    reward_q: &Account<'info, RewardQueue>,
    member: &Account<'info, Member>,
    balances: &BalanceSandboxAccounts<'info>,
    balances_locked: &BalanceSandboxAccounts<'info>,
) -> Result<()> {
    let mut cursor = member.rewards_cursor;

    // If the member's cursor is less then the tail, then the ring buffer has
    // overwritten those entries, so jump to the tail.
    let tail = reward_q.tail();
    if cursor < tail {
        cursor = tail;
    }

    while cursor < reward_q.head() {
        let r_event = reward_q.get(cursor);
        if member.last_stake_ts < r_event.ts {
            if balances.spt.amount > 0 || balances_locked.spt.amount > 0 {
                return Err(CustomErrorCode::RewardsNeedsProcessing.into());
            }
        }
        cursor += 1;
    }

    Ok(())
}


    #[access_control(no_available_rewards(
        &ctx.accounts.reward_event_q,
        &ctx.accounts.member,
        &ctx.accounts.balances,
        &ctx.accounts.balances_locked,
    ))]
    pub fn handler(ctx: Context<Stake>, spt_amount: u64, locked: bool) -> Result<()> {
        let balances = {
            if locked {
                &ctx.accounts.balances_locked
            } else {
                &ctx.accounts.balances
            }
        };

        // Transfer tokens into the stake vault.
        {
            let seeds = &[
                ctx.accounts.registrar.to_account_info().key.as_ref(),
                ctx.accounts.member.to_account_info().key.as_ref(),
                &[ctx.accounts.member.nonce],
            ];
            let member_signer = &[&seeds[..]];
            let cpi_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.clone(),
                Transfer {
                    from: balances.vault.to_account_info(),
                    to: balances.vault_stake.to_account_info(),
                    authority: ctx.accounts.member_signer.to_account_info(),
                },
                member_signer,
            );
            // Convert from stake-token units to mint-token units.
            let token_amount = spt_amount
                .checked_mul(ctx.accounts.registrar.stake_rate)
                .unwrap();
           transfer(cpi_ctx, token_amount)?;
        }

        // Mint pool tokens to the staker.
        {
            let seeds = &[
                ctx.accounts.registrar.to_account_info().key.as_ref(),
                &[ctx.accounts.registrar.nonce],
            ];
            let registrar_signer = &[&seeds[..]];

            let cpi_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.clone(),
                MintTo {
                    mint: ctx.accounts.pool_mint.to_account_info(),
                    to: balances.spt.to_account_info(),
                    authority: ctx.accounts.registrar_signer.to_account_info(),
                },
                registrar_signer,
            );
            mint_to(cpi_ctx, spt_amount)?;
        }

        // Update stake timestamp.
        let member = &mut ctx.accounts.member;
        member.last_stake_ts = ctx.accounts.clock.unix_timestamp;

        Ok(())
    }