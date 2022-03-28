
#[derive(Accounts)]
pub struct ExpireReward<'info> {
    // Staking instance globals.
    registrar: ProgramAccount<'info, Registrar>,
    // Vendor.
    #[account(mut, has_one = registrar, has_one = vault, has_one = expiry_receiver)]
    vendor: ProgramAccount<'info, RewardVendor>,
    #[account(mut)]
    vault: CpiAccount<'info, TokenAccount>,
    #[account(
        seeds = [
            registrar.to_account_info().key.as_ref(),
            vendor.to_account_info().key.as_ref(),
            &[vendor.nonce],
        ]
    )]
    vendor_signer: AccountInfo<'info>,
    // Receiver.
    #[account(signer)]
    expiry_receiver: AccountInfo<'info>,
    #[account(mut)]
    expiry_receiver_token: AccountInfo<'info>,
    // Misc.
    #[account("token_program.key == &token::ID")]
    token_program: AccountInfo<'info>,
    clock: Sysvar<'info, Clock>,
}




pub fn expire_reward(ctx: Context<ExpireReward>) -> Result<()> {
    if ctx.accounts.clock.unix_timestamp < ctx.accounts.vendor.expiry_ts {
        return Err(ErrorCode::VendorNotYetExpired.into());
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
        token::Transfer {
            to: ctx.accounts.expiry_receiver_token.to_account_info(),
            from: ctx.accounts.vault.to_account_info(),
            authority: ctx.accounts.vendor_signer.to_account_info(),
        },
        signer,
    );
    token::transfer(cpi_ctx, ctx.accounts.vault.amount)?;

    // Burn the vendor.
    let vendor = &mut ctx.accounts.vendor;
    vendor.expired = true;

    Ok(())
    }