use super::models::{Pubkey, Lamports, TokenAccount};

pub fn can_transfer(from: &TokenAccount, to: &TokenAccount, amount: Lamports, mint: Pubkey) -> bool {
    from.owner != to.owner
    && from.amount >= amount
    && from.mint == mint
    && to.mint == mint
    && amount != Lamports(0)

}

pub fn apply_transfer_inplace(from: &mut TokenAccount, to: &mut TokenAccount, amount: Lamports, mint: Pubkey ) {
    assert!(from.amount >= amount);
    assert!(from.owner != to.owner);
    assert!(from.mint == mint && mint == to.mint);

    from.amount.0 -= amount.0;
    to.amount.0 += amount.0;

}

pub fn get_index(acc: &[TokenAccount], fromi: Pubkey, toj: Pubkey) -> (usize, usize) {
    let mut i = 0;
    let mut j = 0;
    
    if let Some(from) = acc.iter().position(|acc| acc.owner == fromi) {
        i = from;
    }
    if let  Some(to) = acc.iter().position(|acc| acc.owner == toj) {
        j = to;
    }

    (i,j)


}

pub fn apply_transfer_by_index(accounts: &mut [TokenAccount], i:usize, j: usize, amount: Lamports, mint: Pubkey ) {

  assert!(i != j);

    let (from, to) = if i < j {

       let (left, right) =  accounts.split_at_mut(j);
       (&mut left[i], &mut right[0])

    } else {

        let(left, right) = accounts.split_at_mut(i);
        (&mut right[0], &mut left[j])
    };

    if !can_transfer(&from, &to, amount, mint) {
        return;
    }


    from.amount.0 -= amount.0;
    to.amount.0 += amount.0;

}

pub fn account_summary(acc: &TokenAccount) -> String {
    format!("owner: {}, amount: {}, mint: {}", acc.owner, acc.amount, acc.mint)

}







#[cfg(test)]
mod tests {
    use std::result;

    use super::*;

    #[test]
    fn can_transfer_same_owner() {
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

        let result = can_transfer(&from, &to, Lamports(100), Pubkey([2;32]));
        assert_eq!(result, false);
    }


     #[test]
    fn can_transfer_not_enought_amount() {
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
    fn can_transfer_miss_mint() {
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

        let result = can_transfer(&from, &to, Lamports(100), Pubkey([3;32]));
        assert_eq!(result, false);
    }

     #[test]
    fn can_transfer_happy() {
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
    fn apply_transfer_inplace_happy() {
        let mut from = TokenAccount{
            owner: Pubkey([1;32]),
            mint: Pubkey([2;32]),
            amount: Lamports(100),
        };
        let mut to = TokenAccount {
            owner: Pubkey([2;32]),
            mint: Pubkey([2;32]),
            amount: Lamports(300),
        };

         apply_transfer_inplace(&mut from, &mut to, Lamports(100), Pubkey([2;32]));
         let result = from.amount == Lamports(0) && to.amount == Lamports(400);
        let expected = true;
        assert_eq!(result, expected);
    }

    #[test] 
    fn get_index_happy() {
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
    let (i,j) = get_index(&accounts, fromi, toj);

    let result = (i, j) == (0, 2);
    let expected = true;
    assert_eq!(result, expected);
    }


    #[test] 
    fn apply_transfer_by_index_happy() {

        let i = 0;
        let j = 2;

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

    
        apply_transfer_by_index(&mut accounts, i, j, Lamports(100), Pubkey([9;32]));
        let result = accounts[0].amount == Lamports(0) &&  accounts[2].amount == Lamports(400);
        let expected = true;
        assert_eq!(result, expected);



    }

    #[test] 
    fn apply_transfer_by_index_miss_ammount() {

        let i = 0;
        let j = 2;

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

    
        apply_transfer_by_index(&mut accounts, i, j, Lamports(200), Pubkey([9;32]));
        assert!(accounts[0].amount == Lamports(100) && accounts[2].amount == Lamports(300));



    }

   #[test]
fn apply_transfer_by_index_same_owner() {
    let i = 0;
    let j = 1;

    let mut accounts = vec![
        TokenAccount {
            owner: Pubkey([1; 32]),
            mint: Pubkey([9; 32]),
            amount: Lamports(100),
        },
        TokenAccount {
            owner: Pubkey([1; 32]), 
            mint: Pubkey([9; 32]),
            amount: Lamports(300),
        },
    ];

    apply_transfer_by_index(&mut accounts, i, j, Lamports(50), Pubkey([9;32]));

    assert!(accounts[0].amount == Lamports(100) && accounts[1].amount == Lamports(300));





}

#[test]
fn apply_transfer_by_index_miss_mint() {
    let i = 0;
    let j = 1;

    let mut accounts = vec![
        TokenAccount {
            owner: Pubkey([1; 32]),
            mint: Pubkey([9; 32]),
            amount: Lamports(100),
        },
        TokenAccount {
            owner: Pubkey([2; 32]),
            mint: Pubkey([8; 32]),
            amount: Lamports(300),
        },
    ];

    apply_transfer_by_index(&mut accounts, i, j, Lamports(50), Pubkey([9;32]));

    assert!(accounts[0].amount == Lamports(100) && accounts[1].amount == Lamports(300));
}

   #[test]
fn apply_transfer_by_index_zero_amount() {
    let i = 0;
    let j = 1;

    let mut accounts = vec![
        TokenAccount {
            owner: Pubkey([1; 32]),
            mint: Pubkey([9; 32]),
            amount: Lamports(100),
        },
        TokenAccount {
            owner: Pubkey([2; 32]),
            mint: Pubkey([9; 32]),
            amount: Lamports(300),
        },
    ];

    apply_transfer_by_index(&mut accounts, i, j, Lamports(0), Pubkey([9;32]));

    assert!(accounts[0].amount == Lamports(100) && accounts[1].amount == Lamports(300));
}
}
