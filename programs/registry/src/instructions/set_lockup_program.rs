use anchor_lang::prelude::*;

impl Registry {
    pub fn new(ctx: Context<Ctor>) -> Result<Self> {
        Ok(Registry {
            lockup_program: *ctx.accounts.lockup_program.key,
        })
    }

    impl<'info> RealizeLock<'info, IsRealized<'info>> for Registry {
        fn is_realized(ctx: Context<IsRealized>, v: Vesting) -> ProgramResult {
            if let Some(realizor) = &v.realizor {
                if &realizor.metadata != ctx.accounts.member.to_account_info().key {
                    return Err(ErrorCode::InvalidRealizorMetadata.into());
                }
                assert!(ctx.accounts.member.beneficiary == v.beneficiary);
                let total_staked =
                    ctx.accounts.member_spt.amount + ctx.accounts.member_spt_locked.amount;
                if total_staked != 0 {
                    return Err(ErrorCode::UnrealizedReward.into());
                }
            }
            Ok(())
        }
    }


    pub fn handler(
        &mut self,
        ctx: Context<SetLockupProgram>,
        lockup_program: Pubkey,
    ) -> Result<()> {
        // Hard code the authority because the first version of this program
        // did not set an authority account in the global state.
        //
        // When removing the program's upgrade authority, one should remove
        // this method first, redeploy, then remove the upgrade authority.
        let expected: Pubkey = "HUgFuN4PbvF5YzjDSw9dQ8uTJUcwm2ANsMXwvRdY4ABx"
            .parse()
            .unwrap();
        if ctx.accounts.authority.key != &expected {
            return Err(ErrorCode::InvalidProgramAuthority.into());
        }

        self.lockup_program = lockup_program;

        Ok(())
    }
}