use anchor_lang::prelude::*;

declare_id!("cxCf4YKdsjQFDJM6zeVkhoadNb3JSSbG44wDrun5Wou");

#[program]
pub mod nft_marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
