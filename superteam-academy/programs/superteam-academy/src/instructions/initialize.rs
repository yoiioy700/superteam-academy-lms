use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Token2022, Mint};
use anchor_spl::token_2022::{create_mint, initialize_non_transferable};
use anchor_spl::token_2022::spl_token_2022::extension::{
    ExtensionType,
    metadata_pointer::MetadataPointer,
    permanent_delegate::PermanentDelegate,
};

use crate::state::*;
use crate::error::AcademyError;

/// Initialize the platform
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    
    /// Platform authority (multisig)
    pub authority: SystemAccount<'info>,
    
    /// Backend signer
    pub backend_signer: SystemAccount<'info>,
    
    /// Config PDA
    #[account(
        init,
        payer = payer,
        space = Config::SIZE,
        seeds = [Config::SEED],
        bump,
    )]
    pub config: Account<'info, Config>,
    
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

/// Params for initialize
#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct InitializeParams {
    pub max_daily_xp: u32,
    pub max_achievement_xp: u32,
}

pub fn initialize(ctx: Context<Initialize>, params: InitializeParams) -> Result<()> {
    let config = &mut ctx.accounts.config;
    
    config.authority = ctx.accounts.authority.key();
    config.backend_signer = ctx.accounts.backend_signer.key();
    config.current_season = 0;
    config.current_mint = Pubkey::default();
    config.season_closed = true; // Will be set to false when first season created
    config.season_started_at = 0;
    config.max_daily_xp = params.max_daily_xp;
    config.max_achievement_xp = params.max_achievement_xp;
    config._reserved = [0; 32];
    config.bump = ctx.bumps.config;
    
    msg!("Platform initialized");
    msg!("Authority: {}", config.authority);
    msg!("Backend signer: {}", config.backend_signer);
    
    Ok(())
}
