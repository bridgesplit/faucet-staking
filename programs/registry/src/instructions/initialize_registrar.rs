use anchor_lang::prelude::*;
use anchor_spl::token::*;
use anchor_lang::solana_program::program_option::COption;

use crate::{state::*, errors::*};


#[derive(Accounts)]
#[instruction(
    nft_hash_0: [u8; 64],
    nft_hash_1: [u8; 64],
    nft_hash_2: [u8; 64],
    nft_hash_3: [u8; 64],
    nft_hash_4: [u8; 64], 
    authority: Pubkey,
    withdrawal_timelock: i64,
    stake_rate: u64,
    _reward_q_len: u32)]
pub struct InitializeRegistrar<'info> {
    #[account(mut)]
    initializer: Signer<'info>,
    #[account(init, payer = initializer, space = 8 + std::mem::size_of::<Registrar>())]
    registrar: Box<Account<'info, Registrar>>,
    #[account(init, payer = initializer, space =  8 + 4 + (_reward_q_len * std::mem::size_of::<RewardEvent>() as u32) as usize,
    seeds = [registrar.key().as_ref(), QUEUE_SEED], bump)]
    reward_event_q: Box<Account<'info, RewardQueue>>,
    #[account(
    constraint = pool_mint.decimals == 0,
    constraint = pool_mint.supply == 0,
    constraint = pool_mint.mint_authority == COption::Some(*registrar_signer.key)
    )]
    pool_mint: Box<Account<'info, Mint>>,
    reward_mint: Box<Account<'info, Mint>>,
    #[account(
        mut,
        seeds = [registrar.key().as_ref(),
        SIGNER_SEED],
        bump
    )]
    registrar_signer: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    rent: Sysvar<'info, Rent>,

}

pub fn handler(
    ctx: Context<InitializeRegistrar>,
    nft_hash_0: [u8; 64],
    nft_hash_1: [u8; 64],
    nft_hash_2: [u8; 64],
    nft_hash_3: [u8; 64],
    nft_hash_4: [u8; 64], 
    authority: Pubkey,
    withdrawal_timelock: i64,
    stake_rate: u64,
    _reward_q_len: u32,
) -> Result<()> {
    let registrar = &mut ctx.accounts.registrar;

    registrar.authority = authority;
    registrar.nft_hash_0 = nft_hash_0;
    registrar.nft_hash_1 = nft_hash_1;
    registrar.nft_hash_2 = nft_hash_2;
    registrar.nft_hash_3 = nft_hash_3;
    registrar.nft_hash_4 = nft_hash_4;
    registrar.reward_mint = ctx.accounts.reward_mint.key();
    registrar.pool_mint = *ctx.accounts.pool_mint.to_account_info().key;
    registrar.stake_rate = stake_rate;
    registrar.reward_event_q = *ctx.accounts.reward_event_q.to_account_info().key;
    ctx.accounts.reward_event_q.events = vec![RewardEvent{
        locked: false,
        ts: 0,
        vendor: Pubkey::default()
    }; _reward_q_len as usize];
    registrar.withdrawal_timelock = withdrawal_timelock;
    Ok(())
}


