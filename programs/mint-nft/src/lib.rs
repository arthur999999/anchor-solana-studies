use {
    anchor_lang::{
        prelude::*,
        solana_program::program::invoke,
        system_program,
    },
    anchor_spl::{
        associated_token,
        token,
    },
    mpl_token_metadata::{
        ID as TOKEN_METADATA_ID,
        instruction as token_instruction,
    },
};

declare_id!("7RYU21hggffWcHJGA2adRD45vB6XnAabZbXpqv6XFE6S");

#[program]
pub mod mint_nft {
    use super::*;

    pub fn mint_nft(ctx: Context<MintNft>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintNft<'info> {

    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,

    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,

    #[account(mut)]
    pub mint: Signer<'info>,

    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,

    #[account(mut)]
    pub mint_authority: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
    pub token_metadata_program: UncheckedAccount<'info>

}
