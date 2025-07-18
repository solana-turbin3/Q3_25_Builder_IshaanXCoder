use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Transfer, transfer, Mint, Token, TokenAccount, MintTo, mint_to}};
use constant_product_curve::ConstantProduct;

use crate::{state::Config, error::AmmError};

#[derive(Accounts)]
pub struct Swap<'info> {
   #[account(mut)]
   pub user: Signer<'info>,
   pub mint_x: Account<'info, Mint>
    pub mint_y: Account<'info, Mint>
}