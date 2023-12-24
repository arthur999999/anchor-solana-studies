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
    use anchor_lang::solana_program::lamports;

    use super::*;

    pub fn mint_nft(ctx: Context<MintNft>) -> Result<()> {

        msg!("Criando mint account...");
        msg!("Mint: {}", &ctx.accounts.mint.key());
        system_program::create_account(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                system_program::CreateAccount { from: ctx.accounts.mint_authority.to_account_info(), to: ctx.accounts.mint.to_account_info() }),
            10000000, 
            82,
            &ctx.accounts.token_program.key(),
        )?;

        msg!("Iniciando mint account...");
        msg!("Mint: {}", &ctx.accounts.mint.key());
        token::initialize_mint(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::InitializeMint { 
                    mint: ctx.accounts.mint.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info()
                    }
            ),
            0,
            &ctx.accounts.mint_authority.key(),
            Some(&ctx.accounts.mint_authority.key())
        )?;

        msg!("Criando a Token Account...");
        msg!("Endereço de Token {}", &ctx.accounts.token_account.key());
        associated_token::create(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                associated_token::Create { 
                    payer: ctx.accounts.mint_authority.to_account_info(),
                    associated_token: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(), 
                    rent: ctx.accounts.rent.to_account_info() 
                  }
                )
        )?;

        msg!("Cunhando token para a token account");
        msg!("Mint: {}", &ctx.accounts.mint.to_account_info().key());
        msg!("Token Adress {}", &ctx.accounts.token_account.key() );
        token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::MintTo { 
                    mint: ctx.accounts.mint.to_account_info(), 
                    to: ctx.accounts.token_account.to_account_info(), 
                    authority: ctx.accounts.mint_authority.to_account_info()
                 }
            ),
            1
        )?;

        msg!("Criando a metadata account");
        msg!("Metadata account adress: {}", &ctx.accounts.metadata.to_account_info().key());
        invoke(
            &token_instruction::create_metadata_accounts_v2(
                program_id, 
                metadata_account, 
                mint, 
                mint_authority, 
                payer, 
                update_authority, 
                name, 
                symbol, 
                uri, 
                creators, 
                seller_fee_basis_points, 
                update_authority_is_signer, 
                is_mutable, 
                collection, 
                uses
            ), 
            account_infos
        );


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
