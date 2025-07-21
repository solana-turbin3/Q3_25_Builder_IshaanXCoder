use anchor_lang::{prelude::*, solana_program::example_program_instruction::example_program_instruction};

use crate::states::Marketplace;

#[derive(Accounts)]
pub struct initializeMarketplace<'info>{
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(init,
    payer = admin, 
    space = 8 + Marketplace::INIT_SPACE,
    seeds
)]
    pub marrketplace: Account<'info, Marketplace>
}