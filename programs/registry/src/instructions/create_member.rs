use anchor_lang::prelude::*;

use crate::{state::*, errors::*, utils::*};
use std::convert::TryInto;


#[derive(Accounts)]
pub struct CreateMember<'info> {
    // Stake instance.
    registrar: Account<'info, Registrar>,
    // Member.
    #[account(init, payer = beneficiary,
    space = 8 + std::mem::size_of::<Member>())]
    member: Account<'info, Member>,
    #[account(mut, signer)]
    beneficiary: AccountInfo<'info>,
    #[account(
        "&balances.spt.owner == member_signer.key",
        "balances.spt.mint == registrar.pool_mint",
        "balances.vault.mint == registrar.reward_mint"
    )]
    balances: BalanceSandboxAccounts<'info>,
    #[account(
        "&balances_locked.spt.owner == member_signer.key",
        "balances_locked.spt.mint == registrar.pool_mint",
        "balances_locked.vault.mint == registrar.reward_mint"
    )]
    balances_locked: BalanceSandboxAccounts<'info>,
    member_signer: AccountInfo<'info>,
    // Misc.
    #[account("token_program.key == &anchor_spl::token::ID")]
    token_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    rent: Sysvar<'info, Rent>,
}


impl<'info> CreateMember<'info> {
    fn accounts(ctx: &Context<CreateMember>, nonce: u8) -> Result<()> {
        let seeds = &[
            ctx.accounts.registrar.to_account_info().key.as_ref(),
            ctx.accounts.member.to_account_info().key.as_ref(),
            &[nonce],
        ];
        let member_signer = Pubkey::create_program_address(seeds, ctx.program_id)
            .map_err(|_| CustomErrorCode::InvalidNonce)?;
        if &member_signer != ctx.accounts.member_signer.to_account_info().key {
            return Err(CustomErrorCode::InvalidMemberSigner.into());
        }

        Ok(())
    }
}
   
#[access_control(CreateMember::accounts(&ctx, nonce))]
pub fn create_member(ctx: Context<CreateMember>, nonce: u8, hash: String) -> Result<()> {
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
    member.balances = (&ctx.accounts.balances).into();
    member.balances_locked = (&ctx.accounts.balances_locked).into();
    member.nonce = nonce;
    Ok(())
}