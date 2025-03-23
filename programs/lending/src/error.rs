use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("Insufficient funds to complete the transaction")]
  InsufficientFunds,
}