
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateMember<'info> {
    #[account(mut, has_one = beneficiary)]
    member: ProgramAccount<'info, Member>,
    #[account(signer)]
    beneficiary: AccountInfo<'info>,
}


#[derive(Accounts)]
pub struct IsRealized<'info> {
    #[account(
        "&member.balances.spt == member_spt.to_account_info().key",
        "&member.balances_locked.spt == member_spt_locked.to_account_info().key"
    )]
    member: ProgramAccount<'info, Member>,
    member_spt: CpiAccount<'info, TokenAccount>,
    member_spt_locked: CpiAccount<'info, TokenAccount>,
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
    /// Signer nonce.
    pub nonce: u8,
}

