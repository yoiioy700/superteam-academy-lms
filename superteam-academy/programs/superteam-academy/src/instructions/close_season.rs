use anchor_lang::prelude::*;

use crate::state::*;
use crate::error::AcademyError;

/// Close current season (no more XP minting)
#[derive(Accounts)]
pub struct CloseSeason<'info> {
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
}

pub fn close_season(ctx: Context<CloseSeason>) -> Result<()> {
    let config = &mut ctx.accounts.config;
    
    config.season_closed = true;
    
    msg!("Season {} closed", config.current_season);
    
    Ok(())
}
