use anchor_lang::prelude::*;

declare_id!("EvUizY9L1NXMyzYHsDdCu8MZX7JuRyT9Gj5QuQPMxfEC");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
