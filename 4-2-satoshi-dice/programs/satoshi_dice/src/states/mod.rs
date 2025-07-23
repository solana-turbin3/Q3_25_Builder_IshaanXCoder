use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Game {
    pub player: Pubkey,
  pub amount: u64,
  //   roll for the number generated
  pub slot: u64,
  pub seed: u128,
  pub roll: u64,
  pub bump: u8,
}

impl Bet{
    pub fn to_slice(&self) -> Vec<u8>{
        let mut s = self.player.to_bytes().to_vec;
        s.extend_from_slice(&self.amount.to_le_bytes());
        s.extend_from_slice(&self.slot.to_le_bytes());
        s.extend_from_slice(&self.seed.to_le_bytes());
        s.extend_from_slice(&self.bump.to_le_bytes());
        s.extend_from_slice(&self.roll.to_le_bytes());
        s
    }
}