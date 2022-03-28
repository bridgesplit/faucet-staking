//! A relatively advanced example of a staking program. If you're new to Anchor,
//! it's suggested to start with the other examples.




use anchor_lang::prelude::*;


pub mod errors;
mod instructions;
pub mod state;
pub mod utils;

use instructions::*;


declare_id!("7CGVhsk3d4Fu2Ua8SsePqGquC2YpSkTFyZWvEEagT7r6");

#[program]
mod registry {
    use super::*;


}

