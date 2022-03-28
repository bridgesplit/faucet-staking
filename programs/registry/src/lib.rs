//! A relatively advanced example of a staking program. If you're new to Anchor,
//! it's suggested to start with the other examples.

use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_option::COption;
use anchor_spl::token::{self, Mint, TokenAccount, Transfer};
use lockup::{CreateVesting, RealizeLock, Realizor, Vesting};
use std::convert::Into;

declare_id!("7CGVhsk3d4Fu2Ua8SsePqGquC2YpSkTFyZWvEEagT7r6");

#[program]
mod registry {

    use super::*;


}

