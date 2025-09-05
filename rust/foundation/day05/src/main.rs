use crate::defi::models::{Lamports,TokenAccount,Pubkey};
use crate::defi::inplace;

mod defi;
fn main() {

    let mut accounts = vec![
        TokenAccount {
            owner: Pubkey([1; 32]),
            mint: Pubkey([9; 32]),
            amount: Lamports(100),
        },
        TokenAccount {
            owner: Pubkey([2; 32]),
            mint: Pubkey([9; 32]),
            amount: Lamports(200),
        },
        TokenAccount {
            owner: Pubkey([3; 32]),
            mint: Pubkey([9; 32]),
            amount: Lamports(300),
        },
    ];

    let fromi = Pubkey([1;32]);
    let toj = Pubkey([3;32]);
    let (i,j) = inplace::get_index(&accounts, fromi, toj);

    inplace::apply_transfer_inplace(&mut TokenAccount {
            owner: Pubkey([1; 32]),
            mint: Pubkey([9; 32]),
            amount: Lamports(100),
        },
         &mut TokenAccount {
            owner: Pubkey([3; 32]),
            mint: Pubkey([9; 32]),
            amount: Lamports(300),
        },
         Lamports(100),

         Pubkey([9;32]));
    

    inplace::apply_transfer_by_index(&mut accounts, i, j, Lamports(100), Pubkey([2;32]));

    let summary = inplace::account_summary(& TokenAccount {
            owner: Pubkey([1; 32]),
            mint: Pubkey([9; 32]),
            amount: Lamports(100),
        });

        println!("{}", summary);



}




