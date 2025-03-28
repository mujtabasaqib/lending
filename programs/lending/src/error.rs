use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("Insufficient funds to complete the transaction")]
  InsufficientFunds,
  #[msg("User has overborrowed the maximum amount")]
  OverBorrowableAmount,
  #[msg("User has overpaid the maximum amount")]
  OverRepay,
  #[msg("User is not under collateralized, cannot liquidate")]
  NotUnderCollateralized,
}