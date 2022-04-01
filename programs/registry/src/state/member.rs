
use anchor_lang::prelude::*;
use anchor_spl::token::*;
use crate::state::reward::*;



#[derive(Accounts)]
pub struct IsRealized<'info> {
    #[account(
        "&member.balances.spt == member_spt.to_account_info().key",
        "&member.balances_locked.spt == member_spt_locked.to_account_info().key"
    )]
    pub member: Account<'info, Member>,
    pub member_spt: Account<'info, TokenAccount>,
    pub member_spt_locked: Account<'info, TokenAccount>,
}


#[account]
pub struct Member {
    /// Registrar the member belongs to.
    pub registrar: Pubkey,
    /// The effective owner of the Member account.
    pub beneficiary: Pubkey,
    /// Arbitrary metadata account owned by any program.
    pub metadata: Pubkey,
    /// Sets of balances owned by the Member.
    pub balances: BalanceSandbox,
    /// Locked balances owned by the Member.
    pub balances_locked: BalanceSandbox,
    /// Next position in the rewards event queue to process.
    pub rewards_cursor: u32,
    /// The clock timestamp of the last time this account staked or switched
    /// entities. Used as a proof to reward vendors that the Member account
    /// was staked at a given point in time.
    pub last_stake_ts: i64,
    pub spt: Pubkey,
    pub locked_spt: Pubkey,

    pub vault: Pubkey,

    pub vault_stake: Pubkey,

    pub vault_pw: Pubkey,

    pub locked_vault: Pubkey,

    pub locked_vault_stake: Pubkey,

    pub locked_vault_pw: Pubkey
}

