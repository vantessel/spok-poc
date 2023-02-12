use anchor_lang::prelude::*;

#[account]
pub struct Spok {
    // Immutable
    pub mint: Pubkey,
    pub bump: u8,

    // Updated every mint
    pub mints: u8,

    // Updated every target period
    pub last_target_slot: u64,
    pub target: [u8; 32],

    // Updated every halvening period
    pub last_halvening_slot: u64,
    pub subsidy: u64,
}

impl Spok {
    pub const LEN: usize = 8 + 32 + 1 + 1 + 8 + 32 + 8 + 8;
}
