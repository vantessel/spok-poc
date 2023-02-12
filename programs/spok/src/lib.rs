use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

mod constants;
mod errors;
mod instructions;
mod state;

use instructions::*;

#[program]
pub mod spok {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn mine(ctx: Context<Mine>, nonce: Vec<u8>) -> Result<()> {
        mine::handler(ctx, nonce)
    }
}
