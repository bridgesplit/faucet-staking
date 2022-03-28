
#[derive(Accounts)]
pub struct DropReward<'info> {
    // Staking instance.
    #[account(has_one = reward_event_q, has_one = pool_mint)]
    registrar: ProgramAccount<'info, Registrar>,
    #[account(mut)]
    reward_event_q: ProgramAccount<'info, RewardQueue>,
    pool_mint: CpiAccount<'info, Mint>,
    // Vendor.
    #[account(init)]
    vendor: ProgramAccount<'info, RewardVendor>,
    #[account(mut)]
    vendor_vault: CpiAccount<'info, TokenAccount>,
    // Depositor.
    #[account(mut)]
    depositor: AccountInfo<'info>,
    #[account(signer)]
    depositor_authority: AccountInfo<'info>,
    // Misc.
    #[account("token_program.key == &token::ID")]
    token_program: AccountInfo<'info>,
    clock: Sysvar<'info, Clock>,
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
        .map_err(|_| ErrorCode::InvalidNonce)?;
        if vendor_signer != ctx.accounts.vendor_vault.owner {
            return Err(ErrorCode::InvalidVaultOwner.into());
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
            return Err(ErrorCode::InsufficientReward.into());
        }
        if ctx.accounts.clock.unix_timestamp >= expiry_ts {
            return Err(ErrorCode::InvalidExpiry.into());
        }
        if ctx.accounts.registrar.to_account_info().key == &fida_registrar::ID {
            if ctx.accounts.vendor_vault.mint != fida_mint::ID {
                return Err(ErrorCode::InvalidMint.into());
            }
            if total < FIDA_MIN_REWARD {
                return Err(ErrorCode::InsufficientReward.into());
            }
        } else if ctx.accounts.registrar.to_account_info().key == &srm_registrar::ID
            || ctx.accounts.registrar.to_account_info().key == &msrm_registrar::ID
        {
            if ctx.accounts.vendor_vault.mint != srm_mint::ID {
                return Err(ErrorCode::InvalidMint.into());
            }
            if total < SRM_MIN_REWARD {
                return Err(ErrorCode::InsufficientReward.into());
            }
        } else {
            // TODO: in a future major version upgrade. Add the amount + mint
            //       to the registrar so that one can remove the hardcoded
            //       variables.
            solana_program::msg!("Reward amount not constrained. Please open a pull request.");
        }
        if let RewardVendorKind::Locked {
            start_ts,
            end_ts,
            period_count,
        } = kind
        {
            if !lockup::is_valid_schedule(start_ts, end_ts, period_count) {
                return Err(ErrorCode::InvalidVestingSchedule.into());
            }
        }

        // Transfer funds into the vendor's vault.
        token::transfer(ctx.accounts.into(), total)?;

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