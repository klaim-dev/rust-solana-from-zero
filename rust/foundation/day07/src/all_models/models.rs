use std::fmt;
use bs58;
use rand::Rng;

#[derive(Debug, Copy, Clone,PartialEq, Eq,)]
pub struct Pubkey(pub [u8;32]);

impl fmt::Display for Pubkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let base58_str = bs58::encode(self.0).into_string();
        write!(f, "{}", base58_str)
    }
}

impl Pubkey {
    pub fn new_random() -> Self {
        let mut bytes = [0u8;32];
        rand::thread_rng().fill(&mut bytes);
        Pubkey(bytes)

    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, Eq, PartialEq,PartialOrd)]
pub struct Lamports(pub u64);

impl fmt::Display for Lamports {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} lamports", self.0)
    }
}
#[derive(Debug)]
pub struct TokenAccount {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub amount: Lamports,
}

impl TokenAccount {
    pub fn new_dummy(byte: u8) -> Self {
        TokenAccount {
            owner: Pubkey([byte; 32]),
            mint: Pubkey([byte; 32]),
            amount: Lamports(100),
        }
    }

    pub fn new_with(owner_byte: u8, amount: u64, mint: Pubkey) -> Self {
        TokenAccount {
            owner: Pubkey([owner_byte; 32]),
            mint,
            amount: Lamports(amount),
        }
    }
}

#[derive(Debug)]
pub struct Transfer {
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: Lamports,
}