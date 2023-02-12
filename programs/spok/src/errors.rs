use anchor_lang::prelude::*;

#[error_code]
pub enum SpokError {
    #[msg("Invalid hash")]
    InvalidHash,
}
