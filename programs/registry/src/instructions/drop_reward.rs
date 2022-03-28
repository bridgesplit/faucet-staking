
use anchor_lang::prelude::*;
use anchor_spl::token::*;

use crate::{state::*, errors::*};


#[derive(Accounts)]
pub struct DropReward<'info> {
    // Staking instance.
    #[account(has_one = reward_event_q, has_one = pool_mint)]
    registrar: Box<Account<'info, Registrar>>,
    #[account(mut)]
    reward_event_q: Box<Account<'info, RewardQueue>>,
    pool_mint: Box<Account<'info, Mint>>,
    // Vendor.
    #[account(init,
    payer = depositor_authority, space = 8 + std::mem::size_of::<RewardVendor>())]
    vendor: Box<Account<'info, RewardVendor>>,
    #[account(mut)]
    vendor_vault: Box<Account<'info, TokenAccount>>,
    // Depositor.
    #[account(mut)]
    depositor: AccountInfo<'info>,
    #[account(mut, signer)]
    depositor_authority: AccountInfo<'info>,
    // Misc.
    #[account("token_program.key == &anchor_spl::token::ID")]
    token_program: AccountInfo<'info>,
    clock: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
    rent: Sysvar<'info, Rent>,
}


impl<'a, 'b, 'c, 'info> From<&mut DropReward<'info>>
    for CpiContext<'a, 'b, 'c, 'info, Transfer<'info>>
{
    fn from(accounts: &mut DropReward<'info>) -> CpiContext<'a, 'b, 'c, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: accounts.depositor.clone(),
            to: accounts.vendor_vault.to_account_info(),
            authority: accounts.depositor_authority.clone(),
        };
        let cpi_program = accounts.token_program.clone();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

impl<'info> DropReward<'info> {
    fn accounts(ctx: &Context<DropReward>, nonce: u8) -> Result<()> {
        let vendor_signer = Pubkey::create_program_address(
            &[
                ctx.accounts.registrar.to_account_info().key.as_ref(),
                ctx.accounts.vendor.to_account_info().key.as_ref(),
                &[nonce],
            ],
            ctx.program_id,
        )
        .map_err(|_| CustomErrorCode::InvalidNonce)?;
        if vendor_signer != ctx.accounts.vendor_vault.owner {
            return Err(CustomErrorCode::InvalidVaultOwner.into());
        }

        Ok(())
    }
}


#[access_control(DropReward::accounts(&ctx, nonce))]
    pub fn handler(
        ctx: Context<DropReward>,
        kind: RewardVendorKind,
        total: u64,
        expiry_ts: i64,
        expiry_receiver: Pubkey,
        nonce: u8,
    ) -> Result<()> {
        if total < ctx.accounts.pool_mint.supply {
            return Err(CustomErrorCode::InsufficientReward.into());
        }
        if ctx.accounts.clock.unix_timestamp >= expiry_ts {
            return Err(CustomErrorCode::InvalidExpiry.into());
        }

        if total < ctx.accounts.registrar.minimum_reward_amount {
            return Err(CustomErrorCode::InsufficientReward.into());
        }
        if ctx.accounts.vendor_vault.mint != ctx.accounts.registrar.reward_mint{
            return Err(CustomErrorCode::InvalidMint.into());
        }
        if let RewardVendorKind::Locked {
            start_ts,
            end_ts,
            period_count,
        } = kind
        {
            if !lockup::is_valid_schedule(start_ts, end_ts, period_count) {
                return Err(CustomErrorCode::InvalidVestingSchedule.into());
            }
        }

        // Transfer funds into the vendor's vault.
        transfer(ctx.accounts.into(), total)?;

        // Add the event to the reward queue.
        let reward_q = &mut ctx.accounts.reward_event_q;
        let cursor = reward_q.append(RewardEvent {
            vendor: *ctx.accounts.vendor.to_account_info().key,
            ts: ctx.accounts.clock.unix_timestamp,
            locked: kind != RewardVendorKind::Unlocked,
        })?;

        // Initialize the vendor.
        let vendor = &mut ctx.accounts.vendor;
        vendor.registrar = *ctx.accounts.registrar.to_account_info().key;
        vendor.vault = *ctx.accounts.vendor_vault.to_account_info().key;
        vendor.mint = ctx.accounts.vendor_vault.mint;
        vendor.nonce = nonce;
        vendor.pool_token_supply = ctx.accounts.pool_mint.supply;
        vendor.reward_event_q_cursor = cursor;
        vendor.start_ts = ctx.accounts.clock.unix_timestamp;
        vendor.expiry_ts = expiry_ts;
        vendor.expiry_receiver = expiry_receiver;
        vendor.from = *ctx.accounts.depositor_authority.key;
        vendor.total = total;
        vendor.expired = false;
        vendor.kind = kind;

        Ok(())
    }