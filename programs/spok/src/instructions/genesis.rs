use crate::{
    constants::{DECIMALS, INITIAL_SUBSIDY, MAX_TARGET},
    state::Spok,
};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

#[derive(Accounts)]
pub struct Genesis<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(init, payer = payer, mint::decimals = DECIMALS, mint::authority = spok )]
    pub mint: Account<'info, Mint>,

    #[account(init, payer = payer, seeds = [b"spok"], bump, space = Spok::LEN)]
    pub spok: Account<'info, Spok>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<Genesis>) -> Result<()> {
    let spok = &mut ctx.accounts.spok;

    let current_slot = Clock::get()?.slot;

    spok.mint = ctx.accounts.mint.key();
    spok.bump = *ctx.bumps.get("spok").unwrap();

    spok.mints = 0;

    spok.last_target_slot = current_slot;
    spok.target = MAX_TARGET;

    spok.last_halvening_slot = current_slot;
    spok.subsidy = INITIAL_SUBSIDY;

    Ok(())
}
