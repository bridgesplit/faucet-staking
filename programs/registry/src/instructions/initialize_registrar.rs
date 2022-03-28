use anchor_lang::prelude::*;
use anchor_spl::token::*;
use anchor_lang::solana_program::program_option::COption;

use crate::{state::*, errors::*};


#[derive(Accounts)]
pub struct InitializeRegistrar<'info> {
    #[account(mut)]
    initializer: Signer<'info>,
    #[account(init, payer = initializer, space = 8 + std::mem::size_of::<Registrar>())]
    registrar: Account<'info, Registrar>,
    #[account(init, payer = initializer, space = 8 + std::mem::size_of::<RewardQueue>() + 256,
    seeds = [registrar.key().as_ref()], bump)]
    reward_event_q: Account<'info, RewardQueue>,
    #[account("pool_mint.decimals == 0")]
    pool_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    rent: Sysvar<'info, Rent>,

}


impl<'info> InitializeRegistrar<'info> {
    fn accounts(ctx: &Context<InitializeRegistrar<'info>>, nonce: u8) -> Result<()> {
        let registrar_signer = Pubkey::create_program_address(
            &[
                ctx.accounts.registrar.to_account_info().key.as_ref(),
                &[nonce],
            ],
            ctx.program_id,
        )
        .map_err(|_| CustomErrorCode::InvalidNonce)?;
        if ctx.accounts.pool_mint.mint_authority != COption::Some(registrar_signer) {
            return Err(CustomErrorCode::InvalidPoolMintAuthority.into());
        }
        assert!(ctx.accounts.pool_mint.supply == 0);
        Ok(())
    }
}






#[access_control(InitializeRegistrar::accounts(&ctx, nonce))]
pub fn handler(
    ctx: Context<InitializeRegistrar>,
    nft_hash_0: [u8; 64],
    nft_hash_1: [u8; 64],
    nft_hash_2: [u8; 64],
    nft_hash_3: [u8; 64],
    nft_hash_4: [u8; 64], 
    authority: Pubkey,
    nonce: u8,
    withdrawal_timelock: i64,
    stake_rate: u64,
    reward_q_len: u32,
) -> Result<()> {
    let registrar = &mut ctx.accounts.registrar;

    registrar.authority = authority;
    registrar.nonce = nonce;
    registrar.nft_hash_0 = nft_hash_0;
    registrar.nft_hash_1 = nft_hash_1;
    registrar.nft_hash_2 = nft_hash_2;
    registrar.nft_hash_3 = nft_hash_3;
    registrar.nft_hash_4 = nft_hash_4;
    registrar.pool_mint = *ctx.accounts.pool_mint.to_account_info().key;
    registrar.stake_rate = stake_rate;
    registrar.reward_event_q = *ctx.accounts.reward_event_q.to_account_info().key;
    registrar.withdrawal_timelock = withdrawal_timelock;

    let reward_q = &mut ctx.accounts.reward_event_q;
    reward_q
        .events
        .resize(reward_q_len as usize, Default::default());

    Ok(())
}


