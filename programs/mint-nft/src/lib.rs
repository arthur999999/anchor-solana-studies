use anchor_lang::prelude::*;

declare_id!("7RYU21hggffWcHJGA2adRD45vB6XnAabZbXpqv6XFE6S");

#[program]
pub mod mint_nft {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
