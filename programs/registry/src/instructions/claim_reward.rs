use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ClaimReward<'info> {
    cmn: ClaimRewardCommon<'info>,
    // Account to send reward to.
    #[account(mut)]
    to: AccountInfo<'info>,
}


#[access_control(reward_eligible(&ctx.accounts.cmn))]
    pub fn claim_reward(ctx: Context<ClaimReward>) -> Result<()> {
    if RewardVendorKind::Unlocked != ctx.accounts.cmn.vendor.kind {
        return Err(ErrorCode::ExpectedUnlockedVendor.into());
    }
    // Reward distribution.
    let spt_total =
        ctx.accounts.cmn.balances.spt.amount + ctx.accounts.cmn.balances_locked.spt.amount;
    let reward_amount = spt_total
        .checked_mul(ctx.accounts.cmn.vendor.total)
        .unwrap()
        .checked_div(ctx.accounts.cmn.vendor.pool_token_supply)
        .unwrap();
    assert!(reward_amount > 0);

    // Send reward to the given token account.
    let seeds = &[
        ctx.accounts.cmn.registrar.to_account_info().key.as_ref(),
        ctx.accounts.cmn.vendor.to_account_info().key.as_ref(),
        &[ctx.accounts.cmn.vendor.nonce],
    ];
    let signer = &[&seeds[..]];
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.cmn.token_program.clone(),
        token::Transfer {
            from: ctx.accounts.cmn.vault.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.cmn.vendor_signer.to_account_info(),
        },
        signer,
    );
    token::transfer(cpi_ctx, reward_amount)?;

    // Update member as having processed the reward.
    let member = &mut ctx.accounts.cmn.member;
    member.rewards_cursor = ctx.accounts.cmn.vendor.reward_event_q_cursor + 1;

    Ok(())
    
    }