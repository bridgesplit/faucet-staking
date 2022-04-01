use anchor_lang::prelude::*;

    #[account]
    pub struct Registry {
        pub lockup_program: Pubkey,
    }

