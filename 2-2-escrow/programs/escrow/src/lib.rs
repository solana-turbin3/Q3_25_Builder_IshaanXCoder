use anchor_lang::prelude::*;

declare_id!("FudaFtfdXeH7WQo4ut9JnKvVfpWYMhT2LFvDoB4akhBq");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
