use crate::all_models::models::{Lamports, Pubkey, TokenAccount, Transfer};
use crate::errors::parse_error::ParseError;
use crate::errors::transfer_error::TransferError;
use crate::defi::validate::validate_transfer;
use crate::defi::inplace_checked::apply_transfer_inplace_checked;

pub fn parse_lamports(s: &str) -> Result<Lamports, ParseError> {
    let parse = s.trim().parse::<u64>().map_err(|_| ParseError::InvalidAmountFormat)?;
    Ok(Lamports(parse))
}

pub fn parse_index(s: &str) -> Result<usize, ParseError> {
    let parse = s.trim().parse::<usize>().map_err(|_| ParseError::InvalidIndexFormat)?;
    Ok(parse)
}

pub fn parse_mint(s: &str) -> Result<Pubkey, ParseError> {
    let b = s.trim().parse::<u8>().map_err(|_| ParseError::InvalidMintFormat)?;
    Ok(Pubkey([b; 32]))
}


pub fn get_two_mut_checked<'a>(xs: &'a mut [TokenAccount], i: usize, j: usize) -> Result<(&'a mut TokenAccount, &'a mut TokenAccount), TransferError> {
let len = xs.len();
if i >= len || j >= len {
   return Err(TransferError::IndexOutOfBounds { len: len, i: i, j: j });
}
if i == j {
    return Err(TransferError::SameIndex);
}

let (from, to) = if i > j {
    let (left, right) = xs.split_at_mut(i);
    (&mut right[0], &mut left[j])
} else {
    let (left,right) = xs.split_at_mut(j);
    (&mut left[i], &mut right[0])
};

Ok((from, to))

}

pub fn simulate_transfer(xs: &mut [TokenAccount], i: usize, j: usize, amount: Lamports, mint: Pubkey) -> Result<(Lamports, Lamports), TransferError> {
let (from, to) = get_two_mut_checked(xs, i, j)?;
validate_transfer(&from, &to, amount, mint)?;
let from_new = from.amount.0
.checked_sub(amount.0)
.ok_or(TransferError::InsufficientFunds { have: from.amount, need: amount })?;
let to_new = to.amount.0.checked_add(amount.0)
.ok_or(TransferError::OverflowToBalance)?;

Ok((Lamports(from_new), Lamports(to_new)))

}


pub fn apply_transfer(xs: &mut [TokenAccount], i: usize, j: usize, amount: Lamports, mint: Pubkey) -> Result<Transfer, TransferError> {
    let(from, to) = get_two_mut_checked(xs, i, j)?;
    validate_transfer(&from, &to, amount, mint)?;
    apply_transfer_inplace_checked(from, to, amount, mint)?;
    let tx = Transfer { from: from.owner, to: to.owner, amount};
    Ok(tx)
 }



 #[cfg(test)]
mod tests {
    use super::*;
    use crate::all_models::models::{TokenAccount, Lamports, Pubkey};

#[test]
fn test_parse_lamports_valid() {
    let parsed = parse_lamports("123").unwrap();
    assert_eq!(parsed, Lamports(123));
}

#[test]
fn test_parse_lamports_invalid() {
    let parsed = parse_lamports("abc");
    assert!(parsed.is_err());
}

#[test]
fn test_parse_index_valid() {
    let parsed = parse_index("42").unwrap();
    assert_eq!(parsed, 42);
}

#[test]
fn test_parse_index_invalid() {
    let parsed = parse_index("-1");
    assert!(parsed.is_err());
}

#[test]
fn test_parse_mint_valid() {
    let parsed = parse_mint("5").unwrap();
    assert_eq!(parsed, Pubkey([5; 32]));
}

#[test]
fn test_parse_mint_invalid() {
    let parsed = parse_mint("abc");
    assert!(parsed.is_err());
}

#[test]
fn test_get_two_mut_checked_valid() {
    let mut accounts = vec![
        TokenAccount::new_dummy(1),
        TokenAccount::new_dummy(2),
    ];
    let result = get_two_mut_checked(&mut accounts, 0, 1);
    assert!(result.is_ok());
}

#[test]
fn test_get_two_mut_checked_same_index() {
    let mut accounts = vec![TokenAccount::new_dummy(1)];
    let result = get_two_mut_checked(&mut accounts, 0, 0);
    assert!(result.is_err());
}

#[test]
fn test_get_two_mut_checked_out_of_bounds() {
    let mut accounts = vec![TokenAccount::new_dummy(1)];
    let result = get_two_mut_checked(&mut accounts, 0, 1);
    assert!(result.is_err());
}

#[test]
fn test_simulate_transfer_ok() {
    let mint = Pubkey([1; 32]);
    let mut accounts = vec![
        TokenAccount::new_with(1, 100, mint),
        TokenAccount::new_with(2, 0, mint),
    ];
    let result = simulate_transfer(&mut accounts, 0, 1, Lamports(50), mint);
    assert_eq!(result.unwrap(), (Lamports(50), Lamports(50)));
}

#[test]
fn test_simulate_transfer_insufficient_funds() {
    let mint = Pubkey([1; 32]);
    let mut accounts = vec![
        TokenAccount::new_with(1, 10, mint),
        TokenAccount::new_with(2, 0, mint),
    ];
    let result = simulate_transfer(&mut accounts, 0, 1, Lamports(50), mint);
    assert!(result.is_err());
}

#[test]
fn test_simulate_transfer_mint_mismatch() {
    let mint1 = Pubkey([1; 32]);
    let mint2 = Pubkey([2; 32]);
    let mut accounts = vec![
        TokenAccount::new_with(1, 100, mint1),
        TokenAccount::new_with(2, 0, mint2),
    ];
    let result = simulate_transfer(&mut accounts, 0, 1, Lamports(50), mint1);
    assert!(result.is_err());
}

#[test]
fn simulate_transfer_does_not_mutate_accounts() {
    let mint = Pubkey([1; 32]);
    let  a =  TokenAccount::new_with(1, 100, mint);
    let  b = TokenAccount::new_with(2, 0, mint);
    let mut accounts = vec![a.clone(), b.clone()];
    let before = vec![a,b];

    

    let res = simulate_transfer(&mut accounts, 0, 1, Lamports(10), mint);

    assert!(res.is_ok());
    assert_eq!(before, accounts); 
}



#[test]
fn apply_transfer_success() {
    let mint = Pubkey([42; 32]);
    let mut accounts = vec![
        TokenAccount::new_with(1, 100, mint),
        TokenAccount::new_with(2, 0, mint),
    ];

    let res = apply_transfer(&mut accounts, 0, 1, Lamports(50), mint);
    assert!(res.is_ok());

    let tx = res.unwrap();
    assert_eq!(tx.amount, Lamports(50));
    assert_eq!(tx.from, Pubkey([1; 32]));
    assert_eq!(tx.to, Pubkey([2; 32]));
    assert_eq!(accounts[0].amount, Lamports(50));
    assert_eq!(accounts[1].amount, Lamports(50));
}

#[test]
fn apply_transfer_same_index_fails() {
    let mint = Pubkey([42; 32]);
    let mut accounts = vec![TokenAccount::new_with(1, 100, mint)];

    let res = apply_transfer(&mut accounts, 0, 0, Lamports(10), mint);
    assert!(matches!(res, Err(TransferError::SameIndex)));
}

#[test]
fn apply_transfer_mint_mismatch_fails() {
    let mint1 = Pubkey([1; 32]);
    let mint2 = Pubkey([2; 32]);
    let mut accounts = vec![
        TokenAccount::new_with(1, 100, mint1),
        TokenAccount::new_with(2, 0, mint2),
    ];

    let res = apply_transfer(&mut accounts, 0, 1, Lamports(10), mint1);
       dbg!(&res);
    assert!(matches!(res, Err(TransferError::MintMismatch { .. })));
}

#[test]
fn apply_transfer_overflow_fails() {
    let mint = Pubkey([42; 32]);
    let mut accounts = vec![
        TokenAccount::new_with(1, 100, mint),
        TokenAccount::new_with(2, u64::MAX, mint),
    ];

    let res = apply_transfer(&mut accounts, 0, 1, Lamports(1), mint);

    assert!(matches!(res, Err(TransferError::OverflowToBalance)));
}

}