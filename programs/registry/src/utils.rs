use anchor_lang::prelude::*;

use crate::{
    errors::CustomErrorCode,
    state::*
};
use std::{convert::TryInto, result::Result as ResultGeneric};

pub fn get_bump_in_seed_form<'info>(bump: &u8) -> 
[u8; 1] {
    let bump_val = *bump;
    return [bump_val];

}

pub fn check_hash_in_manager<'info>(hash_bytes: [u8; 64], registrar: &Account<Registrar>) -> bool{

    if registrar.nft_hash_0.eq(&hash_bytes) {
        return true;
    } else if registrar.nft_hash_1.eq(&hash_bytes) {
        return true;
    } else if registrar.nft_hash_2.eq(&hash_bytes) {
        return true;
    } else if registrar.nft_hash_3.eq(&hash_bytes) {
        return true;
    } else if registrar.nft_hash_4.eq(&hash_bytes) {
        return true;
    }
    return false;


}



