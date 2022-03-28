use anchor_lang::prelude::*;
use anchor_spl::token::*;

use crate::{state::*, errors::*};

#[derive(Accounts)]
pub struct ExpireReward<'info> {
    // Staking instance globals.
    registrar: Box<Account<'info, Registrar>>,
    // Vendor.
    #[account(mut, has_one = registrar, has_one = vault, has_one = expiry_receiver)]
    vendor: Box<Account<'info, RewardVendor>>,
    #[account(mut)]
    vault: Box<Account<'info, TokenAccount>>,
    #[account(
        seeds = [
            registrar.to_account_info().key.as_ref(),
            vendor.to_account_info().key.as_ref(),
            &[vendor.nonce],
        ],
        bump
    )]
    vendor_signer: AccountInfo<'info>,
    // Receiver.
    #[account(signer)]
    expiry_receiver: AccountInfo<'info>,
    #[account(mut)]
    expiry_receiver_token: AccountInfo<'info>,
    // Misc.
    #[account("token_program.key == &anchor_spl::token::ID")]
    token_program: AccountInfo<'info>,
    clock: Sysvar<'info, Clock>,
}




pub fn handler(ctx: Context<ExpireReward>) -> Result<()> {
    if ctx.accounts.clock.unix_timestamp < ctx.accounts.vendor.expiry_ts {
        return Err(CustomErrorCode::VendorNotYetExpired.into());
    }

    // Send all remaining funds to the expiry receiver's token.
    let seeds = &[
        ctx.accounts.registrar.to_account_info().key.as_ref(),
        ctx.accounts.vendor.to_account_info().key.as_ref(),
        &[ctx.accounts.vendor.nonce],
    ];
    let signer = &[&seeds[..]];
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.clone(),
        Transfer {
            to: ctx.accounts.expiry_receiver_token.to_account_info(),
            from: ctx.accounts.vault.to_account_info(),
            authority: ctx.accounts.vendor_signer.to_account_info(),
        },
        signer,
    );
    transfer(cpi_ctx, ctx.accounts.vault.amount)?;

    // Burn the vendor.
    let vendor = &mut ctx.accounts.vendor;
    vendor.expired = true;

    Ok(())
    }