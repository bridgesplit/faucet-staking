use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

use crate::{state::*, errors::*, utils::*};
use std::convert::TryInto;


#[derive(Accounts)]
pub struct CreateMember<'info> {
    // Stake instance.
    registrar: Box<Account<'info, Registrar>>,
    nft_mint: Box<Account<'info, Mint>>,
    // Member.
    #[account(init, payer = beneficiary,
        seeds = [beneficiary.key.as_ref(), nft_mint.key().as_ref()], bump,
    space = 8 + std::mem::size_of::<Member>())]
    member: Box<Account<'info, Member>>,
    #[account(mut, signer)]
    beneficiary: AccountInfo<'info>,
    #[account(mut,
        constraint = spt.mint == registrar.pool_mint,
        constraint = spt.owner == *member_signer.key
    )]
    pub spt: Box<Account<'info, TokenAccount>>,
    #[account(mut,
        constraint = locked_spt.mint == registrar.pool_mint,
        constraint = locked_spt.owner == *member_signer.key
    )]
    pub locked_spt:  Box<Account<'info, TokenAccount>>,
    #[account(
        mut, 
        constraint = vault.owner == spt.owner,
        constraint = vault.mint == nft_mint.key(),
    )]
    pub vault: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = vault_stake.owner == spt.owner,
        constraint = vault_stake.mint == vault.mint
    )]
    pub vault_stake: Box<Account<'info, TokenAccount>>,
    #[account(mut, 
        constraint =vault_pw.owner == spt.owner,
        constraint = vault_pw.mint == vault.mint
    )]
    pub vault_pw: Box<Account<'info, TokenAccount>>,
    #[account(
        mut, 
        constraint = locked_vault.owner == locked_spt.owner,
        constraint = locked_vault.mint == nft_mint.key(),
        constraint = locked_vault.owner == *member_signer.key,
    )]
    pub locked_vault: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = locked_vault_stake.owner == locked_spt.owner,
        constraint = locked_vault_stake.mint == locked_vault.mint
    )]
    pub locked_vault_stake: Box<Account<'info, TokenAccount>>,
    #[account(mut, 
        constraint = locked_vault_pw.owner == spt.owner,
        constraint = locked_vault_pw.mint == locked_vault.mint)]
    pub locked_vault_pw: Box<Account<'info, TokenAccount>>,
    #[account(mut,
    seeds = [registrar.key().as_ref(), member.key().as_ref(),
    SIGNER_SEED],
    bump)]
    member_signer: AccountInfo<'info>,
    // Misc.
    #[account(
        constraint = token_program.key == &anchor_spl::token::ID
    )]
    token_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<CreateMember>, hash: String) -> Result<()> {
    let hash_bytes: [u8; 64] = hash
    .as_bytes()
    .try_into()
    .ok()
    .ok_or(error!(CustomErrorCode::WrongHash))?;

    if !check_hash_in_manager(hash_bytes, &ctx.accounts.registrar) {
        return Err(CustomErrorCode::HashMismatch.into());
    }

    let member = &mut ctx.accounts.member;
    member.registrar = *ctx.accounts.registrar.to_account_info().key;
    member.beneficiary = *ctx.accounts.beneficiary.key;
    member.spt = ctx.accounts.spt.key();
    member.vault = ctx.accounts.vault.key();
    member.vault_stake = ctx.accounts.vault_stake.key();
    member.vault_pw = ctx.accounts.vault_pw.key();
    member.locked_vault = ctx.accounts.locked_vault.key();
    member.locked_spt = ctx.accounts.locked_spt.key();
    member.locked_vault_pw = ctx.accounts.locked_vault_pw.key();
    member.locked_vault_stake = ctx.accounts.locked_vault_stake.key();
    Ok(())
}