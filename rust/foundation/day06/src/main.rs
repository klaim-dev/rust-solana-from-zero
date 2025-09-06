use crate::errors::err::TransferError;
use crate::defi::models::{Lamports,Pubkey,TokenAccount};
use crate::defi::validate;
use crate::defi::inplace_checked;

mod errors;
mod defi;

fn main() {


        let mut  from = TokenAccount{
            owner: Pubkey::new_random(),
            mint: Pubkey([2;32]),
            amount: Lamports(100),
        };
        let mut to = TokenAccount {
            owner: Pubkey::new_random(),
            mint: Pubkey([2;32]),
            amount: Lamports(300),
        };

        let amount = Lamports(100);
        let mint = Pubkey([3;32]);

        let result = inplace_checked::apply_transfer_inplace_checked(&mut from, &mut to, amount, mint);
        match result {
            Ok(()) => println!("Transfer succsess"),
            Err(e) => println!("{}", e),
        };


}