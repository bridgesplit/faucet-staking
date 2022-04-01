use anchor_lang::prelude::*;

use crate::{state::*, errors::*, utils::*};
use anchor_spl::{*, token::*};
use ::lockup::*;



#[derive(Accounts)]
pub struct ClaimReward<'info> {
    cmn: ClaimRewardCommon<'info>,
    // Account to send reward to.
    #[account(mut)]
    to: AccountInfo<'info>,
}


fn reward_eligible(cmn: &ClaimRewardCommon) -> Result<()> {
    let vendor = &cmn.vendor;
    let member = &cmn.member;
    if vendor.expired {
        return Err(CustomErrorCode::VendorExpired.into());
    }
    if member.rewards_cursor > vendor.reward_event_q_cursor {
        return Err(CustomErrorCode::CursorAlreadyProcessed.into());
    }
    if member.last_stake_ts > vendor.start_ts {
        return Err(CustomErrorCode::NotStakedDuringDrop.into());
    }
    Ok(())
}




// Accounts common to both claim reward locked/unlocked instructions.
#[derive(Accounts)]
pub struct ClaimRewardCommon<'info> {
    // Stake instance.
    registrar: Account<'info, Registrar>,
    // Member.
    #[account(mut, has_one = registrar, has_one = beneficiary)]
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
    // Vendor.
    #[account(
        has_one = registrar, 
        constraint = vendor.vault == *vesting_vault.key
    )]
    vendor: Box<Account<'info, RewardVendor>>,
    #[account(mut)]
    vesting_vault: AccountInfo<'info>,
    #[account(
        seeds = [
            registrar.to_account_info().key.as_ref(),
            vendor.to_account_info().key.as_ref(),
            SIGNER_SEED
        ],
        bump

    )]
    vendor_signer: AccountInfo<'info>,
    // Misc.
    #[account(
        constraint = token_program.key == &anchor_spl::token::ID)]
    token_program: AccountInfo<'info>,
    clock: Sysvar<'info, Clock>,
}


#[access_control(reward_eligible(&ctx.accounts.cmn))]
    pub fn unlocked_handler(ctx: Context<ClaimReward>) -> Result<()> {
    if RewardVendorKind::Unlocked != ctx.accounts.cmn.vendor.kind {
        return Err(CustomErrorCode::ExpectedUnlockedVendor.into());
    }
    // Reward distribution.
    let spt_total =
        ctx.accounts.cmn.spt.amount + ctx.accounts.cmn.locked_spt.amount;
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
        &SIGNER_SEED[..],
        &get_bump_in_seed_form(ctx.bumps.get("vendor_signer").unwrap())
    ];
    let signer = &[&seeds[..]];
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.cmn.token_program.clone(),
        token::Transfer {
            from: ctx.accounts.cmn.vesting_vault.to_account_info(),
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



    use crate::{errors::CustomErrorCode, state::*, utils::*};


#[derive(Accounts)]
pub struct ClaimRewardLocked<'info> {
    cmn: ClaimRewardCommon<'info>,
    registry: Account<'info, Registry>,
    #[account(constraint = *lockup_program.key == registry.lockup_program)]
    lockup_program: AccountInfo<'info>,
}


    #[access_control(reward_eligible(&ctx.accounts.cmn))]
    pub fn locked_handler<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, ClaimRewardLocked<'info>>,
        nonce: u8,
    ) -> Result<()> { 
        let (start_ts, end_ts, period_count) = match ctx.accounts.cmn.vendor.kind {
            RewardVendorKind::Unlocked => return Err(CustomErrorCode::ExpectedLockedVendor.into()),
            RewardVendorKind::Locked {
                start_ts,
                end_ts,
                period_count,
            } => (start_ts, end_ts, period_count),
        };

        // Reward distribution.
        let spt_total =
            ctx.accounts.cmn.spt.amount + ctx.accounts.cmn.locked_spt.amount;
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
            &SIGNER_SEED[..],
            &get_bump_in_seed_form(ctx.bumps.get("vendor_signer").unwrap())
        ];
        let signer = &[&seeds[..]];

        let mut remaining_accounts: &[AccountInfo] = ctx.remaining_accounts;
        let cpi_program = ctx.accounts.lockup_program.clone();
        let vesting_accounts =
            CreateVesting::try_accounts(ctx.accounts.lockup_program.key, &mut remaining_accounts, &[], &mut ctx.bumps.clone())?;

        let cpi_accounts = 
        ::lockup::cpi::accounts::CreateVesting {
            vesting: vesting_accounts.vesting.to_account_info(),
            vault: vesting_accounts.vault.to_account_info(),
            depositor: vesting_accounts.depositor,
            depositor_authority: vesting_accounts.depositor_authority,
            token_program: vesting_accounts.token_program,
            rent: vesting_accounts.rent.to_account_info(),
            system_program: vesting_accounts.system_program.to_account_info(),
            clock: vesting_accounts.clock.to_account_info()
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
            ::lockup::cpi::create_vesting(
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