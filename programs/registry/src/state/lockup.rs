use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Ctor<'info> {
    pub lockup_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SetLockupProgram<'info> {
    #[account(signer)]
    authority: AccountInfo<'info>,
}
