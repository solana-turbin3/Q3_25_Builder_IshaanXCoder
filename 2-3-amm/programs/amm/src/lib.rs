use anchor_lang::prelude::*;

use anchor_spl::{associated_token::AssociatedToken, token::{mint_to, Mint, Token, TokenAccount}};
use crate::state::Config;

declare_id!("5pixSwsmnuvksmWQ1GmEUrSeE9DjXSxC9dP4h9pzS9sS");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
