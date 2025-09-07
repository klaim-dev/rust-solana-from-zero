
use crate::errors::transfer_error::TransferError;
use crate::all_models::models::{Pubkey,Lamports};


    pub fn user_message(e: &TransferError) -> &'static str {
    match e {
        TransferError::SameOwner => "sender and receiver must be different",
        TransferError::ZeroAmount => "transfer amount must be greater than zero",
        TransferError::MintMismatch { .. } => "tokens must have matching mint",
        TransferError::InsufficientFunds { .. } => "not enough funds",
        TransferError::OverflowToBalance => "destination balance overflow",
        TransferError::SameIndex => "source and destination must differ",
        TransferError::IndexOutOfBounds { .. } => "index out of range",
        TransferError::AliasingDetected => "internal error: aliasing detected",
        TransferError::InvalidInput(_) => "invalid input provided",
    }
}

pub fn tech_message(e: &TransferError) -> String {
    match e {
        TransferError::SameOwner => "Transfer rejected: same owner for both accounts".into(),
        TransferError::ZeroAmount => "Transfer rejected: amount is zero".into(),
        TransferError::MintMismatch { expected, from, to } =>
            format!("Mint mismatch: expected {}, got from {} and to {}", expected, from, to),
        TransferError::InsufficientFunds { have, need } =>
            format!("Insufficient funds: have {}, need {}", have, need),
        TransferError::OverflowToBalance =>
            "Overflow when applying amount to destination account".into(),
        TransferError::SameIndex =>
            "Indexes for sender and receiver are the same".into(),
        TransferError::IndexOutOfBounds { len, i, j } =>
            format!("Index out of bounds: len = {}, i = {}, j = {}", len, i, j),
        TransferError::AliasingDetected =>
            "Aliasing detected: attempted to borrow same account twice".into(),
        TransferError::InvalidInput(msg) => format!("Input parsing failed: {}", msg),
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::all_models::models::{Lamports, Pubkey};

    #[test]
    fn user_message_same_owner() {
        let err = TransferError::SameOwner;
        assert_eq!(user_message(&err), "sender and receiver must be different");
    }

    #[test]
    fn user_message_zero_amount() {
        let err = TransferError::ZeroAmount;
        assert_eq!(user_message(&err), "transfer amount must be greater than zero");
    }

    #[test]
    fn user_message_mint_mismatch() {
        let err = TransferError::MintMismatch {
            expected: Pubkey([1; 32]),
            from: Pubkey([2; 32]),
            to: Pubkey([3; 32]),
        };
        assert_eq!(user_message(&err), "tokens must have matching mint");
    }

    #[test]
    fn user_message_insufficient_funds() {
        let err = TransferError::InsufficientFunds {
            have: Lamports(100),
            need: Lamports(200),
        };
        assert_eq!(user_message(&err), "not enough funds");
    }

    #[test]
    fn user_message_overflow() {
        let err = TransferError::OverflowToBalance;
        assert_eq!(user_message(&err), "destination balance overflow");
    }

    #[test]
    fn user_message_same_index() {
        let err = TransferError::SameIndex;
        assert_eq!(user_message(&err), "source and destination must differ");
    }

    #[test]
    fn user_message_index_out_of_bounds() {
        let err = TransferError::IndexOutOfBounds { len: 3, i: 5, j: 0 };
        assert_eq!(user_message(&err), "index out of range");
    }

    #[test]
    fn user_message_aliasing() {
        let err = TransferError::AliasingDetected;
        assert_eq!(user_message(&err), "internal error: aliasing detected");
    }

    #[test]
    fn tech_message_insufficient_funds_contains_values() {
        let err = TransferError::InsufficientFunds {
            have: Lamports(100),
            need: Lamports(200),
        };
        let msg = tech_message(&err);
        assert!(msg.contains("have 100"));
        assert!(msg.contains("need 200"));
    }

    #[test]
    fn tech_message_mint_mismatch_has_expected_and_actual() {
        let err = TransferError::MintMismatch {
            expected: Pubkey([1; 32]),
            from: Pubkey([2; 32]),
            to: Pubkey([3; 32]),
        };
        let msg = tech_message(&err);
        assert!(msg.contains("expected"));
        assert!(msg.contains("from"));
        assert!(msg.contains("to"));
    }

    #[test]
    fn tech_message_index_out_of_bounds_details() {
        let err = TransferError::IndexOutOfBounds { len: 3, i: 5, j: 7 };
        let msg = tech_message(&err);
        assert!(msg.contains("len = 3"));
        assert!(msg.contains("i = 5"));
        assert!(msg.contains("j = 7"));
    }

    #[test]
fn user_message_invalid_input() {
    let err = TransferError::InvalidInput("invalid amount format".into());
    assert_eq!(user_message(&err), "invalid input provided");
}

#[test]
fn tech_message_invalid_input_includes_detail() {
    let err = TransferError::InvalidInput("invalid amount format".into());
    let msg = tech_message(&err);
    assert!(msg.contains("invalid amount format"));
}

}
