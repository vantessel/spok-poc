use crate::{
    constants::{
        MAX_TARGET, MAX_TARGET_ADJ, MINTS_PER_TARGET_PERIOD, MIN_TARGET_ADJ,
        SLOTS_PER_SUBSIDY_PERIOD, SLOTS_PER_TARGET_PERIOD,
    },
    errors::SpokError,
    state::Spok,
};
use anchor_lang::{
    prelude::*,
    solana_program::keccak::{self, Hash},
};
use anchor_spl::token::{mint_to, Mint, MintTo, Token, TokenAccount};
use ethnum::{AsU256, U256};

#[derive(Accounts)]
#[instruction(nonce: Vec<u8>)]
pub struct Mine<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut, associated_token::mint = spok.mint, associated_token::authority = payer)]
    pub payer_ta: Account<'info, TokenAccount>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(mut, seeds = [b"spok"], bump = spok.bump)]
    pub spok: Account<'info, Spok>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<Mine>, nonce: Vec<u8>) -> Result<()> {
    let spok = &mut ctx.accounts.spok;
    let mint = &ctx.accounts.mint;
    let payer_ta = &ctx.accounts.payer_ta;
    let token_program = &ctx.accounts.token_program;

    let mut val = nonce;
    val.extend_from_slice(&payer_ta.key().to_bytes());
    val.push(spok.mints);

    let target = Hash(spok.target);
    let input_hash = keccak::extend_and_hash(&target, &val);

    if input_hash < target {
        mint_to(
            CpiContext::new_with_signer(
                token_program.to_account_info(),
                MintTo {
                    mint: mint.to_account_info(),
                    to: payer_ta.to_account_info(),
                    authority: spok.to_account_info(),
                },
                &[&[b"spok", &[spok.bump]]],
            ),
            spok.subsidy,
        )?;
    } else {
        return Err(SpokError::InvalidHash.into());
    }

    spok.mints += 1;

    let current_slot = Clock::get()?.slot;

    // adjust difficulty
    if spok.mints == MINTS_PER_TARGET_PERIOD {
        let target_adj = ((current_slot - spok.last_target_slot) as f64
            / SLOTS_PER_TARGET_PERIOD as f64)
            .min(MAX_TARGET_ADJ)
            .max(MIN_TARGET_ADJ);

        let new_target = (U256::from_be_bytes(spok.target).as_f64() * target_adj)
            .as_u256()
            .min(U256::from_be_bytes(MAX_TARGET));

        spok.last_target_slot = current_slot;
        spok.target = new_target.to_be_bytes();
        spok.mints = 0;
        msg!("ADJUSTMENT! Target adjusted by {}", target_adj);
    } else {
        msg!(
            "{} mints left before adjustment",
            MINTS_PER_TARGET_PERIOD - spok.mints
        );
    }

    // adjust subsidy
    if current_slot > spok.last_halvening_slot + SLOTS_PER_SUBSIDY_PERIOD {
        spok.last_halvening_slot = current_slot;
        spok.subsidy /= 2;
        msg!("HALVENING! New subsidy is {}", spok.subsidy);
    }

    Ok(())
}
