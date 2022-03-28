use anchor_lang::prelude::*;



#[derive(Accounts, Clone)]
pub struct BalanceSandboxAccounts<'info> {
    #[account(mut)]
    spt: CpiAccount<'info, TokenAccount>,
    #[account(mut, "vault.owner == spt.owner")]
    vault: CpiAccount<'info, TokenAccount>,
    #[account(
        mut,
        "vault_stake.owner == spt.owner",
        "vault_stake.mint == vault.mint"
    )]
    vault_stake: CpiAccount<'info, TokenAccount>,
    #[account(mut, "vault_pw.owner == spt.owner", "vault_pw.mint == vault.mint")]
    vault_pw: CpiAccount<'info, TokenAccount>,
}





// Accounts common to both claim reward locked/unlocked instructions.
#[derive(Accounts)]
pub struct ClaimRewardCommon<'info> {
    // Stake instance.
    registrar: ProgramAccount<'info, Registrar>,
    // Member.
    #[account(mut, has_one = registrar, has_one = beneficiary)]
    member: ProgramAccount<'info, Member>,
    #[account(signer)]
    beneficiary: AccountInfo<'info>,
    #[account("BalanceSandbox::from(&balances) == member.balances")]
    balances: BalanceSandboxAccounts<'info>,
    #[account("BalanceSandbox::from(&balances_locked) == member.balances_locked")]
    balances_locked: BalanceSandboxAccounts<'info>,
    // Vendor.
    #[account(has_one = registrar, has_one = vault)]
    vendor: ProgramAccount<'info, RewardVendor>,
    #[account(mut)]
    vault: AccountInfo<'info>,
    #[account(
        seeds = [
            registrar.to_account_info().key.as_ref(),
            vendor.to_account_info().key.as_ref(),
            &[vendor.nonce],
        ]
    )]
    vendor_signer: AccountInfo<'info>,
    // Misc.
    #[account("token_program.key == &token::ID")]
    token_program: AccountInfo<'info>,
    clock: Sysvar<'info, Clock>,
}




// BalanceSandbox defines isolated funds that can only be deposited/withdrawn
// into the program.
//
// Once controlled by the program, the associated `Member` account's beneficiary
// can send funds to/from any of the accounts within the sandbox, e.g., to
// stake.
#[derive(AnchorSerialize, AnchorDeserialize, Default, Debug, Clone, PartialEq)]
pub struct BalanceSandbox {
    // Staking pool token.
    pub spt: Pubkey,
    // Free balance (deposit) vaults.
    pub vault: Pubkey,
    // Stake vaults.
    pub vault_stake: Pubkey,
    // Pending withdrawal vaults.
    pub vault_pw: Pubkey,
}



#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum RewardVendorKind {
    Unlocked,
    Locked {
        start_ts: i64,
        end_ts: i64,
        period_count: u64,
    },
}

impl<'info> From<&BalanceSandboxAccounts<'info>> for BalanceSandbox {
    fn from(accs: &BalanceSandboxAccounts<'info>) -> Self {
        Self {
            spt: *accs.spt.to_account_info().key,
            vault: *accs.vault.to_account_info().key,
            vault_stake: *accs.vault_stake.to_account_info().key,
            vault_pw: *accs.vault_pw.to_account_info().key,
        }
    }
}

fn reward_eligible(cmn: &ClaimRewardCommon) -> Result<()> {
    let vendor = &cmn.vendor;
    let member = &cmn.member;
    if vendor.expired {
        return Err(ErrorCode::VendorExpired.into());
    }
    if member.rewards_cursor > vendor.reward_event_q_cursor {
        return Err(ErrorCode::CursorAlreadyProcessed.into());
    }
    if member.last_stake_ts > vendor.start_ts {
        return Err(ErrorCode::NotStakedDuringDrop.into());
    }
    Ok(())
}

#[account]
pub struct RewardQueue {
    // Invariant: index is position of the next available slot.
    head: u32,
    // Invariant: index is position of the first (oldest) taken slot.
    // Invariant: head == tail => queue is initialized.
    // Invariant: index_of(head + 1) == index_of(tail) => queue is full.
    tail: u32,
    // Although a vec is used, the size is immutable.
    events: Vec<RewardEvent>,
}

impl RewardQueue {
    pub fn append(&mut self, event: RewardEvent) -> Result<u32> {
        let cursor = self.head;

        // Insert into next available slot.
        let h_idx = self.index_of(self.head);
        self.events[h_idx] = event;

        // Update head and tail counters.
        let is_full = self.index_of(self.head + 1) == self.index_of(self.tail);
        if is_full {
            self.tail += 1;
        }
        self.head += 1;

        Ok(cursor)
    }

    pub fn index_of(&self, counter: u32) -> usize {
        counter as usize % self.capacity()
    }

    pub fn capacity(&self) -> usize {
        self.events.len()
    }

    pub fn get(&self, cursor: u32) -> &RewardEvent {
        &self.events[cursor as usize % self.capacity()]
    }

    pub fn head(&self) -> u32 {
        self.head
    }

    pub fn tail(&self) -> u32 {
        self.tail
    }
}


#[derive(Default, Clone, Copy, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct RewardEvent {
    vendor: Pubkey,
    ts: i64,
    locked: bool,
}

#[account]
pub struct RewardVendor {
    pub registrar: Pubkey,
    pub vault: Pubkey,
    pub mint: Pubkey,
    pub nonce: u8,
    pub pool_token_supply: u64,
    pub reward_event_q_cursor: u32,
    pub start_ts: i64,
    pub expiry_ts: i64,
    pub expiry_receiver: Pubkey,
    pub from: Pubkey,
    pub total: u64,
    pub expired: bool,
    pub kind: RewardVendorKind,
}