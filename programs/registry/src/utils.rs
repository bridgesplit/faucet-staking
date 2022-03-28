use anchor_lang::prelude::*;

use crate::{
    errors::CustomErrorCode,
    state::{, RewardsFunctionType, StakeAccount, StakeManager, StakeStatus},
};
use std::{convert::TryInto, result::Result as ResultGeneric};

pub fn get_bump_in_seed_form<'info>(bump: &u8) -> 
[u8; 1] {
    let bump_val = *bump;
    return [bump_val];

}

pub fn check_hash_in_manager<'info>(hash_bytes: [u8; 64], registrar: &Account<Registrar>) -> bool{

    if registrar.nft_hash_0.eq(&hash) {
        return true;
    } else if registrar.nft_hash_1.eq(&hash) {
        return true;
    } else if registrar.nft_hash_2.eq(&hash) {
        return true;
    } else if registrar.nft_hash_3.eq(&hash) {
        return true;
    } else if registrar.nft_hash_4.eq(&hash) {
        return true;
    }
    return false;


}



// Asserts the user calling the `Stake` instruction has no rewards available
// in the reward queue.
pub fn no_available_rewards<'info>(
    reward_q: &ProgramAccount<'info, RewardQueue>,
    member: &ProgramAccount<'info, Member>,
    balances: &BalanceSandboxAccounts<'info>,
    balances_locked: &BalanceSandboxAccounts<'info>,
) -> Result<()> {
    let mut cursor = member.rewards_cursor;

    // If the member's cursor is less then the tail, then the ring buffer has
    // overwritten those entries, so jump to the tail.
    let tail = reward_q.tail();
    if cursor < tail {
        cursor = tail;
    }

    while cursor < reward_q.head() {
        let r_event = reward_q.get(cursor);
        if member.last_stake_ts < r_event.ts {
            if balances.spt.amount > 0 || balances_locked.spt.amount > 0 {
                return Err(ErrorCode::RewardsNeedsProcessing.into());
            }
        }
        cursor += 1;
    }

    Ok(())
}