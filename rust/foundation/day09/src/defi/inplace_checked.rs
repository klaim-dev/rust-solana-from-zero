use crate::errors::transfer_error::TransferError;
use crate::all_models::models::{TokenAccount, Pubkey, Lamports};
use crate::defi::validate::validate_transfer;


pub fn apply_transfer_inplace_checked(from: &mut TokenAccount, to: &mut TokenAccount, amount: Lamports, mint: Pubkey) -> Result<(), TransferError> {

    validate_transfer(&from, &to, amount, mint)?;

    let new_from = from.amount.0
    .checked_sub(amount.0)
    .ok_or_else(|| TransferError::InsufficientFunds { have: from.amount, need: amount })?;

    let new_to = to.amount.0
    .checked_add(amount.0)
    .ok_or_else(|| TransferError::OverflowToBalance)?;

    from.amount = Lamports(new_from);
    to.amount = Lamports(new_to);
    Ok(())
}
