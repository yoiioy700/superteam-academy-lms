use anchor_lang::prelude::*;

use crate::state::*;
use crate::error::AcademyError;

/// Update config: rotate backend signer, adjust rate limits
#[derive(Accounts)]
pub struct UpdateConfig<'info> {
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

/// Params for update_config
#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct UpdateConfigParams {
    pub backend_signer: Option<Pubkey>,
    pub max_daily_xp: Option<u32>,
    pub max_achievement_xp: Option<u32>,
}

pub fn update_config(
    ctx: Context<UpdateConfig>,
    params: UpdateConfigParams,
) -> Result<()> {
    let config = &mut ctx.accounts.config;
    
    if let Some(backend_signer) = params.backend_signer {
        config.backend_signer = backend_signer;
        msg!("Backend signer updated to: {}", backend_signer);
    }
    
    if let Some(max_daily_xp) = params.max_daily_xp {
        config.max_daily_xp = max_daily_xp;
        msg!("Max daily XP updated to: {}", max_daily_xp);
    }
    
    if let Some(max_achievement_xp) = params.max_achievement_xp {
        config.max_achievement_xp = max_achievement_xp;
        msg!("Max achievement XP updated to: {}", max_achievement_xp);
    }
    
    Ok(())
}
