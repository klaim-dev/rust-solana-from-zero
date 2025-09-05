
use crate::defi::models::{Pubkey,TokenAccount,Transfer, Lamports};

mod defi;

fn main() {

    let from = TokenAccount{
            owner: Pubkey::new_random(),
            mint: Pubkey([2;32]),
            amount: Lamports(100),
        };
        let to = TokenAccount {
            owner: Pubkey::new_random(),
            mint: Pubkey([2;32]),
            amount: Lamports(300),
        };


let can_tarnsfer = defi::engine::can_transfer(&from, &to, Lamports(100), Pubkey([2;32]));
println!("Can transfer :{}", can_tarnsfer);

let (from, to, transfer) = defi::engine::transfer_consume(from, to, Lamports(100), Pubkey([2;32]));
println!("from: {},to: {}, amount:{:?}", transfer.from, transfer.to, transfer.amount);

}




