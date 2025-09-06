use crate::errors::err::TransferError;
use crate::defi::models::{Lamports,Pubkey,TokenAccount};

pub fn validate_transfer(from: &TokenAccount, to: &TokenAccount, amount: Lamports, mint: Pubkey) -> Result<(), TransferError> { 
    ensure(from.amount.0 != 0 ,TransferError::ZeroAmount)?;

    ensure(from.owner != to.owner, TransferError::SameOwner)?;
    

    ensure(from.mint == to.mint && mint == to.mint, TransferError::MintMismatch { expected: mint, from: from.mint, to: to.mint })?;
    

    ensure(from.amount >= amount, TransferError::InsufficientFunds { have: from.amount, need: amount });

    ensure(amount.0 != 0 ,TransferError::ZeroAmount)?;


    Ok(())

}

pub fn ensure(cond: bool, err: TransferError) -> Result<(), TransferError> {
    if cond {
        Ok(())
    } else {
        Err(err)
    }
}





#[cfg(test)]
mod tests {
    use std::result;

    use super::*;

    #[test]
    fn check_validate_transfer_same_owner() {
        let from = TokenAccount{
            owner: Pubkey([1;32]),
            mint: Pubkey([2;32]),
            amount: Lamports(100),
        };
        let to = TokenAccount {
            owner: Pubkey([1;32]),
            mint: Pubkey([2;32]),
            amount: Lamports(300),
        };

        let result = validate_transfer(&from, &to, Lamports(100), Pubkey([2;32]));
        assert_eq!(result, Err(TransferError::SameOwner));
    }
}

#[test]
    fn check_validate_transfer_zero_amount() {
        let from = TokenAccount{
            owner: Pubkey([1;32]),
            mint: Pubkey([2;32]),
            amount: Lamports(0),
        };
        let to = TokenAccount {
            owner: Pubkey([1;32]),
            mint: Pubkey([2;32]),
            amount: Lamports(300),
        };

        let result = validate_transfer(&from, &to, Lamports(100), Pubkey([2;32]));
        assert_eq!(result, Err(TransferError::ZeroAmount));
    }

    #[test]
    fn check_validate_transfer_mint_missing() {
        let from = TokenAccount{
            owner: Pubkey([2;32]),
            mint: Pubkey([2;32]),
            amount: Lamports(100),
        };
        let to = TokenAccount {
            owner: Pubkey([1;32]),
            mint: Pubkey([2;32]),
            amount: Lamports(300),
        };

        let result = validate_transfer(&from, &to, Lamports(100), Pubkey([3;32]));
        assert_eq!(result, Err(TransferError::MintMismatch { expected: Pubkey([3;32]), from: from.mint, to: to.mint }));
    }
