use crate::states::Bet;
use anchor_lang::prelude::*;
use anchor_lang::prelude::system_program::{transfer, Transfer};

#[derive(Accounts)]
#[instruction(amount: u128)]
pub struct PlaceBet<'info>{
    #[account(mut)]
    pub player: Signer<'info>,
    pub house: UncheckedAccount<'info>,
    //unchecked account is used to avoid checking the account

    #[account(init, 
    payer = player,
    space = 8 + Bet::INIT_SPACE,
    seeds = [b"bet", player.key().as_ref()],
    bump
    )]
    pub bet: Account<'info, Bet>,

    #[account(mut,
        seeds = [b"vault", house.key().as_ref()],
        bump
    )]

    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> PlaceBet<'info> {
    pub fn place_bet(&mut self, amount: u64, seed: u128, roll: u8, bump: u8) -> Result<()> {
        self.bet.set_inner(Bet {
            slot: Clock::get()?.slot,
            player: self.player.key(),
            seed,
            roll,
            amount,
            bump,
        });
        Ok(())
    }

    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let accounts = Transfer {
            from: self.player.to_account_info(),
            to: self.vault.to_account_info(),
        };
        let ctx = CpiContext::new(self.system_program.to_account_info(), accounts);
        transfer(ctx, amount)
    }
}