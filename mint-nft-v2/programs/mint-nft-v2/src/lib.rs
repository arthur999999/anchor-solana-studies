use {
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        metadata::{
            create_master_edition_v3, create_metadata_accounts_v3, CreateMasterEditionV3,
            CreateMetadataAccountsV3, Metadata,
        },
        token::{mint_to, Mint, MintTo, Token, TokenAccount},
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

   
}

