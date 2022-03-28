
#[derive(Accounts)]
pub struct ClaimRewardLocked<'info> {
    cmn: ClaimRewardCommon<'info>,
    registry: ProgramState<'info, Registry>,
    #[account("lockup_program.key == &registry.lockup_program")]
    lockup_program: AccountInfo<'info>,
}


    #[access_control(reward_eligible(&ctx.accounts.cmn))]
    pub fn claim_reward_locked<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, ClaimRewardLocked<'info>>,
        nonce: u8,
    ) -> Result<()> {
        let (start_ts, end_ts, period_count) = match ctx.accounts.cmn.vendor.kind {
            RewardVendorKind::Unlocked => return Err(ErrorCode::ExpectedLockedVendor.into()),
            RewardVendorKind::Locked {
                start_ts,
                end_ts,
                period_count,
            } => (start_ts, end_ts, period_count),
        };

        // Reward distribution.
        let spt_total =
            ctx.accounts.cmn.balances.spt.amount + ctx.accounts.cmn.balances_locked.spt.amount;
        let reward_amount = spt_total
            .checked_mul(ctx.accounts.cmn.vendor.total)
            .unwrap()
            .checked_div(ctx.accounts.cmn.vendor.pool_token_supply)
            .unwrap();
        assert!(reward_amount > 0);

        // Specify the vesting account's realizor, so that unlocks can only
        // execute once completely unstaked.
        let realizor = Some(Realizor {
            program: *ctx.program_id,
            metadata: *ctx.accounts.cmn.member.to_account_info().key,
        });

        // CPI: Create lockup account for the member's beneficiary.
        let seeds = &[
            ctx.accounts.cmn.registrar.to_account_info().key.as_ref(),
            ctx.accounts.cmn.vendor.to_account_info().key.as_ref(),
            &[ctx.accounts.cmn.vendor.nonce],
        ];
        let signer = &[&seeds[..]];
        let mut remaining_accounts: &[AccountInfo] = ctx.remaining_accounts;
        let cpi_program = ctx.accounts.lockup_program.clone();
        let cpi_accounts =
            CreateVesting::try_accounts(ctx.accounts.lockup_program.key, &mut remaining_accounts, &[])?;
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        lockup::cpi::create_vesting(
            cpi_ctx,
            ctx.accounts.cmn.member.beneficiary,
            reward_amount,
            nonce,
            start_ts,
            end_ts,
            period_count,
            realizor,
        )?;

        // Make sure this reward can't be processed more than once.
        let member = &mut ctx.accounts.cmn.member;
        member.rewards_cursor = ctx.accounts.cmn.vendor.reward_event_q_cursor + 1;

        Ok(())
    }