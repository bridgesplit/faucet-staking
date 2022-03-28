use anchor_lang::prelude::*;

use crate::{state::*};


#[derive(Accounts)]
pub struct UpdateRegistrar<'info> {
    #[account(mut, has_one = authority)]
    registrar: Account<'info, Registrar>,
    #[account(signer)]
    authority: AccountInfo<'info>,
}

    pub fn handler(
        ctx: Context<UpdateRegistrar>,
        new_authority: Option<Pubkey>,
        withdrawal_timelock: Option<i64>,
    ) -> Result<()> {
        let registrar = &mut ctx.accounts.registrar;

        if let Some(new_authority) = new_authority {
            registrar.authority = new_authority;
        }

        if let Some(withdrawal_timelock) = withdrawal_timelock {
            registrar.withdrawal_timelock = withdrawal_timelock;
        }

        Ok(())
    }