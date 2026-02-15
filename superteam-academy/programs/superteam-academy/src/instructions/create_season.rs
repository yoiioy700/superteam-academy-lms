use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Token2022};

use crate::state::*;
use crate::error::AcademyError;

/// Create a new season
/// Creates new Token-2022 mint with soulbound config
#[derive(Accounts)]
#[instruction(season: u16)]
pub struct CreateSeason<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    
    /// Config PDA
    #[account(
        mut,
        seeds = [Config::SEED],
        bump = config.bump,
        has_one = authority @ AcademyError::Unauthorized,
    )]
    pub config: Account<'info, Config>,
    
    /// Platform authority
    pub authority: Signer<'info>,
    
    /// New XP mint for this season
    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = config,
    )]
    pub xp_mint: InterfaceAccount<'info, anchor_spl::token_interface::Mint>,
    
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn create_season(ctx: Context<CreateSeason>, season: u16) -> Result<()> {
    let config = &mut ctx.accounts.config;
    
    // Season must be sequential
    require!(
        season == config.current_season + 1,
        AcademyError::InvalidSeasonNumber
    );
    
    // Close previous season if exists
    if config.current_season > 0 {
        require!(config.season_closed, AcademyError::SeasonNotClosed);
    }
    
    config.current_season = season;
    config.current_mint = ctx.accounts.xp_mint.key();
    config.season_closed = false;
    config.season_started_at = Clock::get()?.unix_timestamp;
    
    msg!("Season {} created", season);
    msg!("XP Mint: {}", ctx.accounts.xp_mint.key());
    
    Ok(())
}
