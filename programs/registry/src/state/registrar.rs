use anchor_lang::prelude::*;


#[account]
pub struct Registrar {
    /// Priviledged account.
    pub authority: Pubkey,
    /// Nonce to derive the program-derived address owning the vaults.
    pub nonce: u8,
    /// Number of seconds that must pass for a withdrawal to complete.
    pub withdrawal_timelock: i64,
    /// Global event queue for reward vendoring.
    pub reward_event_q: Pubkey,
    ///hash of the nft collection that can be staked
    pub nft_hash_0: [u8; 64],
    ///hash of the nft collection that can be staked
    pub nft_hash_1: [u8; 64],
    ///hash of the nft collection that can be staked
    pub nft_hash_2: [u8; 64],
    ///hash of the nft collection that can be staked
    pub nft_hash_3: [u8; 64],
    ///hash of the nft collection that can be staked
    pub nft_hash_4: [u8; 64],
    /// Staking pool token mint.
    pub pool_mint: Pubkey,
    /// The amount of tokens (not decimal) that must be staked to get a single
    /// staking pool token.
    pub stake_rate: u64,
}
