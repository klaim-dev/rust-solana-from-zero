mod errors;
mod all_models;
mod defi;
use crate::{all_models::models::{Lamports, Pubkey, TokenAccount}, defi::service::apply_transfer};
use crate::errors::error_format::{user_message, tech_message};
fn main() {

    let parse_i = defi::service::parse_index("0");
    let mut i = 0usize;
    match parse_i {
        Ok(v) => i = v,
        Err(e) => println!("{}", e),
    };

    let parse_j = defi::service::parse_index("1");
    let mut j = 1usize;
    match parse_j {
        Ok(v) => j = v,
        Err(e) => println!("{}", e),
    };
    let parse_lamports = defi::service::parse_lamports("30");
    let mut amount = Lamports(0);
    match parse_lamports {
        Ok(l) => {
            println!("Input Lamports: {}", l);
            amount = l;
        },
        Err(e) => println!("{}", e),
    };

    let parse_mint = defi::service::parse_mint("2");
    let mut mint = Pubkey([0u8; 32]);
    match parse_mint {
        Ok(m) => {
            println!("Input mint address:{}", m);
            mint = m;
        },
        Err(e) => println!(" {}", e),
    };


     let mut accounts = vec![
        TokenAccount::new_with(1, 100, mint),
        TokenAccount::new_with(2, 0, mint),
    ];




    let simulate = defi::service::simulate_transfer(&mut accounts, i, j, amount, mint);
    match simulate {
        Ok((from, to)) => println!("After transfer: from: {}, to: {}", from, to),
        Err(e) => {
            println!("{}", e);
            println!("{}", user_message(&e));
            println!("{}", tech_message(&e));
        },
    };

    let transfer = apply_transfer(&mut accounts, i, j, amount, mint);
    match transfer { 
        Ok(t) => println!("Transfer: from: {}, to: {}, mint address: {}, amount: {}", t.from, t.to, mint, t.amount),
        Err(e) => {
            println!("{}", e);
            println!("{}", user_message(&e));
            println!("{}", tech_message(&e));
        },
    };


    


}
