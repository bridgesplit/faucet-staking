//! A relatively advanced example of a staking program. If you're new to Anchor,
//! it's suggested to start with the other examples.




use anchor_lang::prelude::*;


pub mod errors;
mod instructions;
pub mod state;
pub mod utils;

use instructions::*;


declare_id!("WRDwvUg93dDeMi9A1UqgHUEXrEZVnZ1aNUhf5uZwtEX");

#[program]
mod registry {
    use super::*;

    pub fn claim_locked_reward<'a, 'b, 'c, 'info>(ctx:Context<'a, 'b, 'c, 'info, ClaimRewardLocked<'info>>, nonce: u8) -> Result<()> {
        instructions::claim_reward::locked_handler(ctx, nonce)
    }

    pub fn claim_unlocked_reward(ctx: Context<ClaimReward>) -> Result<()> {
        instructions::claim_reward::unlocked_handler(ctx)
    }

    pub fn create_member(ctx: Context<CreateMember>, nonce: u8, hash: String) -> Result<()> {
        instructions::create_member::handler(ctx, nonce, hash)
    }
    pub fn deposit_locked(ctx: Context<DepositLocked>, amount: u64) -> Result<()> {
        instructions::deposit_locked::handler(ctx, amount)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::deposit::handler(ctx, amount)
    }

    pub fn drop_reward(ctx: Context<DropReward>, kind: RewardVendorKind, total: u64, expiry_ts: i64, expiry_receiver: Pubkey, nonce: u8) -> Result<()> {
        instructions::drop_reward::handler(ctx, kind, total, expiry_ts, expiry_receiver, nonce)
    }

    pub fn end_unstake(ctx: Context<EndUnstake>) -> Result<()> {
        instructions::end_unstake::handler(ctx)
    }

    pub fn expire_reward(ctx: Context<ExpireReward>) -> Result<()> {
        instructions::expire_reward::handler(ctx)
    }

    pub fn initialize_registrar(ctx: Context<InitializeRegistrar>,
        nft_hash_0: [u8; 64],
        nft_hash_1: [u8; 64],
        nft_hash_2: [u8; 64],
        nft_hash_3: [u8; 64],
        nft_hash_4: [u8; 64], 
        authority: Pubkey,
        nonce: u8,
        withdrawal_timelock: i64,
        stake_rate: u64,
        reward_q_len: u32) -> Result<()> {

        instructions::initialize_registrar::handler(ctx, nft_hash_0, nft_hash_1, nft_hash_2, nft_hash_3, nft_hash_4, authority, nonce, withdrawal_timelock, stake_rate, reward_q_len)
    }

    pub fn stake(ctx: Context<Stake>, spt_amount: u64, locked: bool) -> Result<()> {
        instructions::stake::handler(ctx, spt_amount, locked)
    }

    pub fn start_unstake(ctx: Context<StartUnstake>, spt_amount: u64, locked: bool) -> Result<()> {
        instructions::start_unstake::handler(ctx, spt_amount, locked)
    }

    pub fn update_member(ctx: Context<UpdateMember>, metadata: Option<Pubkey>) -> Result<()> {
        instructions::update_member::handler(ctx, metadata)
    }

    pub fn update_registrar(ctx: Context<UpdateRegistrar>, new_authority: Option<Pubkey>, withdrawal_timelock: Option<i64>) -> Result<()> {
        instructions::update_registrar::handler(ctx, new_authority, withdrawal_timelock)
    }

    pub fn withdraw_locked(ctx: Context<WithdrawLocked>, amount: u64) -> Result<()> {
        instructions::withdraw_locked::handler(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        instructions::withdraw::handler(ctx, amount)
    }







}

