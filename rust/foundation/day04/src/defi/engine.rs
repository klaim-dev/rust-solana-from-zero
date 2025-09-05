use crate::defi::models::{Pubkey,TokenAccount,Transfer, Lamports};

pub fn can_transfer(from: &TokenAccount, to: &TokenAccount, amount: Lamports, mint: Pubkey) -> bool {
    from.amount >= amount
    && from.mint == mint
    &&to.mint == mint
    &&from.owner != to.owner
}

pub fn transfer_consume(from: TokenAccount, to: TokenAccount, amount: Lamports, mint: Pubkey ) -> (TokenAccount, TokenAccount, Transfer) {
    let new_from = TokenAccount {
        owner: from.owner,
        mint: from.mint,
        amount: Lamports(from.amount.0 - amount.0),
    };

    let new_to = TokenAccount {
        owner: to.owner,
        mint: to.mint,
        amount: Lamports(to.amount.0 + amount.0),
    };
        let transfer = Transfer{
        from: from.owner,
        to: to.owner,
        mint,
        amount,
    };

    (new_from, new_to, transfer)
}

pub fn account_summary(acc: &TokenAccount) -> String {
    format!("Owner: {:?}, mint: {:?}, amount: {:?}", acc.owner, acc.mint, acc.amount)
}







#[cfg(test)]
mod tests {
    use std::result;

    use super::*;


    #[test]
    fn check_can_transfer_happy() {
        let from = TokenAccount{
            owner: Pubkey([1;32]),
            mint: Pubkey([2;32]),
            amount: Lamports(100),
        };
        let to = TokenAccount {
            owner: Pubkey([2;32]),
            mint: Pubkey([2;32]),
            amount: Lamports(300),
        };
        let result = can_transfer(&from, &to, Lamports(100), Pubkey([2;32]));
        assert_eq!(result, true);
    }

    #[test]
    fn check_can_transfer_bigger_amount() {
        let from = TokenAccount{
            owner: Pubkey([1;32]),
            mint: Pubkey([2;32]),
            amount: Lamports(100),
        };
        let to = TokenAccount {
            owner: Pubkey([2;32]),
            mint: Pubkey([2;32]),
            amount: Lamports(300),
        };
        let result = can_transfer(&from, &to, Lamports(200), Pubkey([2;32]));
        assert_eq!(result, false);
    }

    #[test]
    fn check_can_transfer_miss_mint() {
        let from = TokenAccount{
            owner: Pubkey([1;32]),
            mint: Pubkey([2;32]),
            amount: Lamports(100),
        };
        let to = TokenAccount {
            owner: Pubkey([2;32]),
            mint: Pubkey([2;32]),
            amount: Lamports(300),
        };
        let result = can_transfer(&from, &to, Lamports(200), Pubkey([3;32]));
        assert_eq!(result, false);
    }

    #[test]
    fn check_can_transfer_same_owner() {
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
        let result = can_transfer(&from, &to, Lamports(200), Pubkey([2;32]));
        assert_eq!(result, false);
    }

    #[test]
    fn check_transfer_consume_happy() {
               let from = TokenAccount{
            owner: Pubkey([1;32]),
            mint: Pubkey([2;32]),
            amount: Lamports(100),
        };
        let to = TokenAccount {
            owner: Pubkey([2;32]),
            mint: Pubkey([2;32]),
            amount: Lamports(300),
        };
        let expected = (
            TokenAccount{
            owner: Pubkey([1;32]),
            mint: Pubkey([2;32]),
            amount: Lamports(0),},
            
             TokenAccount {
            owner: Pubkey([2;32]),
            mint: Pubkey([2;32]),
            amount: Lamports(400),},

             Transfer {
        from: Pubkey([1;32]),
        to: Pubkey([2;32]),
        mint: Pubkey([2;32]), 
        amount: Lamports(100),
        });

        let result = transfer_consume(from, to, Lamports(100), Pubkey([2;32]));

        assert_eq!(result, expected);

    }

       #[test]
    fn amount_too_high_should_fail() {
               let from = TokenAccount{
            owner: Pubkey([1;32]),
            mint: Pubkey([2;32]),
            amount: Lamports(100),
        };
        let to = TokenAccount {
            owner: Pubkey([2;32]),
            mint: Pubkey([2;32]),
            amount: Lamports(300)
        };
        let expected = false;

        let result = can_transfer(&from, &to, Lamports(200), Pubkey([2;32]));

        assert_eq!(result, expected );
}

#[test]
fn check_account_summary() {
    let acc = TokenAccount {
        owner: Pubkey([1; 32]),
        mint: Pubkey([2; 32]),
        amount: Lamports(100),
    };

    let summary = account_summary(&acc);

    assert!(summary.contains("amount: 100"));
    assert!(summary.contains("Owner:"));
    assert!(summary.contains("mint:"));
}

}