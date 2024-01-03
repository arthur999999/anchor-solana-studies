use {
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        metadata::{
            create_master_edition_v3, create_metadata_accounts_v3, CreateMasterEditionV3,
            CreateMetadataAccountsV3, Metadata,
        },
        token::{mint_to, Mint, MintTo, TokenAccount},
    },
    mpl_token_metadata::{
        pda::{find_master_edition_account, find_metadata_account},
        state::DataV2,
    },
};


declare_id!("GwuQbUd8WeBG92PC5akefN7664eYYhGExaYyjQ25XMow");

#[program]
pub mod mint_nft_v2 {
    use super::*;

    pub fn mint_nft(ctx: Context<MintNft>, nft_name: String, nft_symbol: String, nft_uri: String) -> Result<()> {

            msg!("Iniciando o mint");
            mint_to(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    MintTo { 
                        mint: ctx.accounts.mint_account.to_account_info(),
                        to: ctx.accounts.associated_token_account.to_account_info(),
                        authority: ctx.accounts.payer.to_account_info() }),
                1
            )?;
            msg!("Mint finalizado");
            msg!("Criando a metadata");
            create_metadata_accounts_v3(
                CpiContext::new(
                    ctx.accounts.token_metadata_program.to_account_info(),
                    CreateMetadataAccountsV3 { 
                        metadata: ctx.accounts.metadata_account.to_account_info(),
                        mint: ctx.accounts.mint_account.to_account_info(), 
                        mint_authority: ctx.accounts.payer.to_account_info(), 
                        payer: ctx.accounts.payer.to_account_info(), 
                        update_authority: ctx.accounts.payer.to_account_info(), 
                        system_program: ctx.accounts.system_program.to_account_info(), 
                        rent: ctx.accounts.rent.to_account_info() 
                    }),
                DataV2 { 
                    name: nft_name, 
                    symbol: nft_symbol, 
                    uri: nft_uri, 
                    seller_fee_basis_points: 0, 
                    creators: None, 
                    collection: None, 
                    uses: None 
                },
                false,
                true,
                None
            )?;

            msg!("Criando a master edition");
            create_master_edition_v3(
                CpiContext::new(
                    ctx.accounts.token_metadata_program.to_account_info(), 
                    CreateMasterEditionV3 { 
                        edition: ctx.accounts.edition_account.to_account_info(), 
                        mint: ctx.accounts.mint_account.to_account_info(), 
                        update_authority: ctx.accounts.payer.to_account_info(), 
                        mint_authority: ctx.accounts.payer.to_account_info(), 
                        payer: ctx.accounts.payer.to_account_info(), 
                        metadata: ctx.accounts.metadata_account.to_account_info(), 
                        token_program: ctx.accounts.token_program.to_account_info(), 
                        system_program: ctx.accounts.system_program.to_account_info(), 
                        rent: ctx.accounts.rent.to_account_info() 
                    }
                ), 
                None
            )?;
            msg!("Tudo feito!");
        
        Ok(())
    }
   
}

#[derive(Accounts)]
pub struct MintNft<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    ///CHECK: 
    #[account(
        mut,
        address=find_metadata_account(&mint_account.key()).0
    )]
    pub metadata_account: UncheckedAccount<'info>,
    ///CHECK:
    #[account(
        mut,
        address=find_master_edition_account(&mint_account.key()).0
    )]
    pub edition_account: UncheckedAccount<'info>,
    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = payer.key(),
        mint::freeze_authority = payer.key()
    )]
    pub mint_account: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint_account,
        associated_token::authority = payer
    )]
    pub associated_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Metadata>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>
}

