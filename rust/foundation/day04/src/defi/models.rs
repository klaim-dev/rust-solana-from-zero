use std::fmt;
use rand::Rng;
use bs58;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Pubkey(pub [u8; 32]);

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
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Lamports(pub u64);

impl fmt::Display for Lamports {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} lamports", self.0)
    }
}



#[derive(Debug, PartialEq, Eq)]
pub struct TokenAccount {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub amount: Lamports,

}
#[derive(Debug, PartialEq, Eq)]
pub struct Transfer {
    pub from: Pubkey,
    pub to: Pubkey,
    pub mint: Pubkey,
    pub amount: Lamports,

}




#[cfg(test)]
mod tests {
    use std::result;

    use super::*;

    #[test]
fn check_display_lamports() {
    let amount = Lamports(12345);
    let printed = format!("{}", amount);
    assert_eq!(printed, "12345 lamports");
}

#[test]
fn check_display_base58() {
    let key = Pubkey([1; 32]);
    let expected = bs58::encode(key.0).into_string();
    assert_eq!(format!("{}", key), expected);
}


}