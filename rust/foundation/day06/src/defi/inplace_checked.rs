use crate::errors::err::TransferError;
use crate::defi::models::{Lamports,Pubkey,TokenAccount};
use crate::defi::validate;




pub fn get_two_accounts<'a>( accounts: &'a mut [TokenAccount], i: usize, j: usize) -> Option<(&'a mut TokenAccount, &'a mut TokenAccount)> {
    let len = accounts.len();

    if i >= len || j >= len {
        return None;
    }

    if i == j {
        return None;
    }

    
    let (from, to ) = if i > j {
        let (left, right) = accounts.split_at_mut(i);
        (&mut right[i], &mut left[0])
    } else {
        let (left, right) = accounts.split_at_mut(j);
        (&mut left[i], &mut right[0])
    };

    Some((from, to))

}

pub fn apply_transfer_inplace_checked(from:&mut TokenAccount, to: &mut TokenAccount, amount: Lamports, mint: Pubkey) -> Result<(), TransferError> {


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



    pub fn apply_transfer_by_index_checked(accounts: &mut [TokenAccount], i: usize, j: usize, amount: Lamports, mint: Pubkey,) -> Result<(), TransferError> {
    let len = accounts.len();
    let (from, to) = get_two_accounts(accounts, i, j)
    .ok_or(TransferError::IndexOutOfBounds { len: len, i: i, j: j})?;
    validate::validate_transfer(&from, &to, amount, mint)?;
    apply_transfer_inplace_checked(from, to, amount, mint)?;
    Ok(())

}


pub fn format_error(e:&TransferError) -> String {
    format!("{}", e)

}







#[cfg(test)]
mod tests {
    use super::*;
    use crate::defi::models::{TokenAccount, Lamports, Pubkey};

    fn new_account(owner: u8, mint: u8, amount: u64) -> TokenAccount {
        TokenAccount {
            owner: Pubkey([owner; 32]),
            mint: Pubkey([mint; 32]),
            amount: Lamports(amount),
        }
    }

    #[test]
    fn happy_path() {
        let mut accounts = vec![
            new_account(1, 2, 100),
            new_account(2, 2, 50),
        ];
        let result = apply_transfer_by_index_checked(
            &mut accounts,
            0,
            1,
            Lamports(30),
            Pubkey([2; 32]),
        );
        assert!(result.is_ok());
        assert_eq!(accounts[0].amount.0, 70);
        assert_eq!(accounts[1].amount.0, 80);
    }

    #[test]
    fn zero_amount_fails() {
        let mut accounts = vec![
            new_account(1, 2, 100),
            new_account(2, 2, 100),
        ];
        let result = apply_transfer_by_index_checked(
            &mut accounts,
            0,
            1,
            Lamports(0),
            Pubkey([2; 32]),
        );
        assert_eq!(result, Err(TransferError::ZeroAmount));
    }

    #[test]
    fn same_owner_fails() {
        let mut accounts = vec![
            new_account(1, 2, 100),
            new_account(1, 2, 100),
        ];
        let result = apply_transfer_by_index_checked(
            &mut accounts,
            0,
            1,
            Lamports(10),
            Pubkey([2; 32]),
        );
        assert_eq!(result, Err(TransferError::SameOwner));
    }

    #[test]
    fn mint_mismatch_fails() {
        let mut accounts = vec![
            new_account(1, 2, 100),
            new_account(2, 3, 100),
        ];
        let result = apply_transfer_by_index_checked(
            &mut accounts,
            0,
            1,
            Lamports(10),
            Pubkey([2; 32]),
        );
        assert!(matches!(result, Err(TransferError::MintMismatch { .. })));
    }

    #[test]
    fn insufficient_funds_fails() {
        let mut accounts = vec![
            new_account(1, 2, 10),
            new_account(2, 2, 0),
        ];
        let result = apply_transfer_by_index_checked(
            &mut accounts,
            0,
            1,
            Lamports(50),
            Pubkey([2; 32]),
        );
        assert!(matches!(result, Err(TransferError::InsufficientFunds { .. })));
    }

    #[test]
    fn index_out_of_bounds_fails() {
        let mut accounts = vec![
            new_account(1, 2, 100),
            new_account(2, 2, 100),
        ];
        let result = apply_transfer_by_index_checked(
            &mut accounts,
            0,
            2, // out of bounds
            Lamports(10),
            Pubkey([2; 32]),
        );
        assert!(matches!(result, Err(TransferError::IndexOutOfBounds { .. })));
    }

    #[test]
    fn overflow_to_balance_fails() {
        let mut accounts = vec![
            new_account(1, 2, 100),
            TokenAccount {
                owner: Pubkey([2; 32]),
                mint: Pubkey([2; 32]),
                amount: Lamports(u64::MAX - 50),
            },
        ];
        let result = apply_transfer_by_index_checked(
            &mut accounts,
            0,
            1,
            Lamports(100),
            Pubkey([2; 32]),
        );
        assert_eq!(result, Err(TransferError::OverflowToBalance));
    }
}


