use anchor_lang::prelude::*;

declare_id!("H58FtNcorkNGHVcasm71w7fnCoV6PtZroEbpZsV7UmJo");

pub mod instructions;
pub mod states;
pub mod constants;
pub mod errors;

pub use instructions::*;

#[program]
pub mod satoshi_dice {
    use super::*;

    pub fn say_hello(ctx: Context<SayHello>) -> Result<()> {
        ctx.accounts.say_hello()?;
        Ok(())
    }
}