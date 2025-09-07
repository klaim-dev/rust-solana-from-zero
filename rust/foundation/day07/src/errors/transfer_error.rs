use crate::all_models::models::{Lamports, Pubkey,TokenAccount};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransferError {
    SameOwner,
    ZeroAmount,
    MintMismatch { expected: Pubkey, from: Pubkey, to: Pubkey },
    InsufficientFunds { have: Lamports, need: Lamports },
    OverflowToBalance,
    SameIndex,
    IndexOutOfBounds { len: usize, i: usize, j: usize },
    AliasingDetected,
    InvalidInput(String),
}

impl std::fmt::Display for TransferError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SameOwner =>
                write!(f, "Source and destination owners are the same"),
            Self::ZeroAmount =>
                write!(f, "Transfer amount cannot be zero"),
            Self::MintMismatch { expected, from, to } =>
                write!(f, "Mint mismatch: expected {}, from {}, to {}", expected, from, to),
            Self::InsufficientFunds { have, need } =>
                write!(f, "Not enough funds: have {}, need {}", have, need),
            Self::OverflowToBalance =>
                write!(f, "Balance overflow on destination account"),
            Self::SameIndex =>
                write!(f, "Source and destination indexes are the same"),
            Self::IndexOutOfBounds { len, i, j } =>
                write!(f, "Index out of bounds: len={}, i={}, j={}", len, i, j),
            Self::AliasingDetected =>
                write!(f, "Aliasing detected: cannot borrow same account mutably twice"),
            Self::InvalidInput(msg) =>
                write!(f, "Invalid input: {}", msg),    
        }
    }
}

