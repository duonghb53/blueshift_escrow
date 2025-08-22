use pinocchio::{pubkey::Pubkey};

#[repr(C)]
pub struct Escrow {
    pub seed: u64,      // Random seed for PDA derivation
    pub maker: Pubkey,  // Creator of the escrow
    pub mint_a: Pubkey, // Token being deposited
    pub mint_b: Pubkey, // Token being requested
    pub receive: u64,   // Amount of token B wanted
    pub bump: [u8; 1],  // PDA bump seed
}
