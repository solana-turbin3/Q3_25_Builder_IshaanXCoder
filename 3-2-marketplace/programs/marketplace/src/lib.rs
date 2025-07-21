use anchor_lang::prelude::*;

declare_id!("9Lhxp8nPc3d5gKSD4jdtSJXcJW4CMMTwx6tRbttm1xGh");

pub mod instructions;
pub mod states;
pub mod constants;
pub mod errors;

pub use instructions::*;

#[program]
pub mod marketplace {
    use super::*;

    pub fn say_hello(ctx: Context<SayHello>) -> Result<()> {
        ctx.accounts.say_hello()?;
        Ok(())
    }
}