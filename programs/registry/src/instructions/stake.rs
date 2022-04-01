use anchor_lang::prelude::*;
use anchor_spl::token::*;

use crate::{state::*, errors::*, claim_reward::*, utils::*};



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
    #[account(
        mut,
        constraint = spt.key() == member.spt
    )]
    pub spt: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = locked_spt.key() == member.locked_spt
       
    )]
    pub locked_spt:  Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = vault.key() == member.vault
    )]
    pub vault: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = vault_stake.key() == member.vault_stake
    )]
    pub vault_stake: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = vault_pw.key() == member.vault_pw
    )]
    pub vault_pw: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = locked_vault.key() == member.locked_vault
    )]
    pub locked_vault: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = locked_vault_stake.key() == member.locked_vault_stake
    )]
    pub locked_vault_stake: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = locked_vault_pw.key() == member.locked_vault_pw
    )]
    pub locked_vault_pw: Box<Account<'info, TokenAccount>>,

    // Program signers.
    #[account(
        mut,
        seeds = [
            registrar.key().as_ref(),
            member.key().as_ref(),
            SIGNER_SEED
        ],
        bump
    )]
    member_signer: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [registrar.key().as_ref(),
        SIGNER_SEED],
        bump
    )]
    registrar_signer: AccountInfo<'info>,

    // Misc.
    clock: Sysvar<'info, Clock>,
    #[account(constraint = token_program.key == &anchor_spl::token::ID)]
    token_program: AccountInfo<'info>,
}




// Asserts the user calling the `Stake` instruction has no rewards available
// in the reward queue.
pub fn no_available_rewards<'info>(
    reward_q: &Account<'info, RewardQueue>,
    member: &Account<'info, Member>,
    spt: &Account<'info, TokenAccount>,
    locked_spt: &Account<'info, TokenAccount>,
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
            if spt.amount > 0 || spt.amount > 0 {
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
        &ctx.accounts.spt,
        &ctx.accounts.locked_spt,
    ))]
    pub fn handler(ctx: Context<Stake>, locked: bool) -> Result<()> {
        let [spt, vault, vault_stake, vault_pw] = {
            if !locked {
                [&ctx.accounts.spt,
                &ctx.accounts.vault,
                &ctx.accounts.vault_stake,
                &ctx.accounts.vault_pw]
            } else {
                [&ctx.accounts.locked_spt,
                &ctx.accounts.locked_vault,
                &ctx.accounts.locked_vault_stake,
                &ctx.accounts.locked_vault_pw]
            }
        };

        // Transfer tokens into the stake vault.
        {
            let seeds = &[
                ctx.accounts.registrar.to_account_info().key.as_ref(),
                ctx.accounts.member.to_account_info().key.as_ref(), 
                &SIGNER_SEED[..],
                &get_bump_in_seed_form(ctx.bumps.get("member_signer").unwrap()),
            ];
            let member_signer = &[&seeds[..]];
            let cpi_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.clone(),
                Transfer {
                    from: vault.to_account_info(),
                    to: vault_stake.to_account_info(),
                    authority: ctx.accounts.member_signer.to_account_info(),
                },
                member_signer,
            );
            transfer(cpi_ctx,1);
        }

        // Mint pool tokens to the staker.
        {
            let seeds = &[
                ctx.accounts.registrar.to_account_info().key.as_ref(),
                &SIGNER_SEED[..],
                &get_bump_in_seed_form(ctx.bumps.get("registrar_signer").unwrap()),
            ];
            let registrar_signer = &[&seeds[..]];

            let cpi_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.clone(),
                MintTo {
                    mint: ctx.accounts.pool_mint.to_account_info(),
                    to: spt.to_account_info(),
                    authority: ctx.accounts.registrar_signer.to_account_info(),
                },
                registrar_signer,
            );
            mint_to(cpi_ctx, 1)?;
        }

        // Update stake timestamp.
        let member = &mut ctx.accounts.member;
        member.last_stake_ts = ctx.accounts.clock.unix_timestamp;

        Ok(())
    }