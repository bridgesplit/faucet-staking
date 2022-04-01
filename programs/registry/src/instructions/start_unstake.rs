use anchor_lang::prelude::*;
use anchor_spl::token::*;


use crate::{state::*, errors::*, claim_reward::*, stake::*, utils::get_bump_in_seed_form};


#[derive(Accounts)]
pub struct StartUnstake<'info> {
    // Stake instance globals.
    #[account(has_one = reward_event_q, has_one = pool_mint)]
    registrar: Box<Account<'info, Registrar>>,
    reward_event_q: Box<Account<'info, RewardQueue>>,
    #[account(mut)]
    pool_mint: AccountInfo<'info>,
    // Member.
    #[account(
        init,
    payer = beneficiary,
    space = 8 + std::mem::size_of::<PendingWithdrawal>())]
    pending_withdrawal: Box<Account<'info, PendingWithdrawal>>,
    #[account(
        mut,
        has_one = beneficiary,
         has_one = registrar)]
    member: Box<Account<'info, Member>>,
    #[account(mut ,signer)]
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

    // Programmatic signers.
    #[account(
        mut,
        seeds = [
            registrar.to_account_info().key.as_ref(),
            member.to_account_info().key.as_ref(),
            SIGNER_SEED
        ],
        bump
    )]
    member_signer: AccountInfo<'info>,

    // Misc.
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    clock: Sysvar<'info, Clock>,
    rent: Sysvar<'info, Rent>,
}
    
#[access_control(no_available_rewards(
    &ctx.accounts.reward_event_q,
    &ctx.accounts.member,
    &ctx.accounts.spt,
    &ctx.accounts.locked_spt,
 ))]
pub fn handler(ctx: Context<StartUnstake>, spt_amount: u64, locked: bool) -> Result<()> {
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

    msg!("got vualts set up");

    // Program signer.
    let seeds = &[
        ctx.accounts.registrar.to_account_info().key.as_ref(),
        ctx.accounts.member.to_account_info().key.as_ref(),
        &SIGNER_SEED[..],
        &get_bump_in_seed_form(ctx.bumps.get("member_signer").unwrap())
    ];
    let member_signer = &[&seeds[..]];

    // Burn pool tokens.
    {
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.pool_mint.to_account_info(),
                to: spt.to_account_info(),
                authority: ctx.accounts.member_signer.to_account_info(),
            },
            member_signer,
        );
        burn(cpi_ctx, spt_amount)?;
    }

    msg!("burnt!");

        // Convert from stake-token units to mint-token units.
    let token_amount = spt_amount
        .checked_mul(ctx.accounts.registrar.stake_rate)
        .unwrap();

        msg!("tkem amt calculated");

        // Transfer tokens from the stake to pending vault.
        {
            let cpi_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: vault_stake.to_account_info(),
                    to: vault_pw.to_account_info(),
                    authority: ctx.accounts.member_signer.to_account_info(),
                },
                member_signer,
            );
            transfer(cpi_ctx, token_amount)?;
        }


        msg!("transferring....");

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