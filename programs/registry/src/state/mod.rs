pub mod deposit;
pub mod lockup;
pub mod member;
pub mod registrar;
pub mod registry;
pub mod reward;
pub mod stake;
pub mod unstake;
pub mod withdraw;



// Native units.
pub const SRM_MIN_REWARD: u64 = 500_000_000;
pub const FIDA_MIN_REWARD: u64 = 900_000_000;

pub mod srm_registrar {
    solana_program::declare_id!("5vJRzKtcp4fJxqmR7qzajkaKSiAb6aT9grRsaZKXU222");
}
pub mod msrm_registrar {
    solana_program::declare_id!("7uURiX2DwCpRuMFebKSkFtX9v5GK1Cd8nWLL8tyoyxZY");
}
pub mod fida_registrar {
    solana_program::declare_id!("5C2ayX1E2SJ5kKEmDCA9ue9eeo3EPR34QFrhyzbbs3qh");
}
pub mod srm_mint {
    solana_program::declare_id!("SRMuApVNdxXokk5GT7XD5cUUgXMBCoAz2LHeuAoKWRt");
}
pub mod fida_mint {
    solana_program::declare_id!("EchesyfXePKdLtoiZSL8pBe8Myagyy8ZRqsACNCFGnvp");
}



pub use deposit;
pub use lockup;
pub use member;
pub use registrar;
pub use registry;
pub use reward;
pub use stake;
pub use unstake;
pub use withdraw;
pub use srm_registrar;
pub use msrm_registrar;
pub use fida_registrar;
pub use srm_mint;
pub use fida_mint;