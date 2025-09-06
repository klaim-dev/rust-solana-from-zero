use std::fmt;
use rand::Rng;
use bs58;

#[derive(Debug, Copy, Clone, Eq,  PartialEq)]
pub struct Pubkey(pub [u8;32]);

impl fmt::Display for Pubkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let base58_str = bs58::encode(self.0).into_string();
        write!(f, "{}", base58_str)
    }
}

impl Pubkey {
    pub fn new_random() -> Self {
        let mut bytes = [0u8; 32];
        rand::thread_rng().fill(&mut bytes);
        Pubkey(bytes)
    }

}



#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd)]
pub struct Lamports(pub u64);

impl fmt::Display for Lamports {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} lamports", self.0)
    }
}



pub struct TokenAccount {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub amount: Lamports,
}


