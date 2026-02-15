use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Token2022, TokenAccount, Mint};
use anchor_spl::token_2022::spl_token_2022::instruction::AuthorityType;

use crate::state::*;
use crate::error::AcademyError;
use crate::utils::check_and_update_daily_xp;

/// Claim an achievement
#[derive(Accounts)]
#[instruction(achievement_index: u8, xp_reward: u32)]
pub struct ClaimAchievement<'info> {
    /// Backend signer
    pub backend_signer: Signer<'info>,
    
    /// Config PDA
    #[account(
        seeds = [Config::SEED],
        bump = config.bump,
        has_one = backend_signer @ AcademyError::Unauthorized,
    )]
    pub config: Account<'info, Config>,
    
    /// Learner wallet
    pub learner: SystemAccount<'info>,
    
    /// LearnerProfile PDA
    #[account(
        mut,
        seeds = [LearnerProfile::SEED, learner.key().as_ref()],
        bump = learner_profile.bump,
    )]
    pub learner_profile: Account<'info, LearnerProfile>,
    
    /// XP Mint (Token-2022)
    pub xp_mint: InterfaceAccount<'info, Mint>,
    
    /// Learner's XP token account
    #[account(
        mut,
        token::mint = xp_mint,
        token::authority = learner,
    )]
    pub learner_token: InterfaceAccount<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token2022>,
}

pub fn claim_achievement(
    ctx: Context<ClaimAchievement>,
    achievement_index: u8,
    xp_reward: u32,
) -> Result<()> {
    require!(
        achievement_index < 256,
        AcademyError::AchievementOutOfBounds
    );
    
    let config = &ctx.accounts.config;
    let learner_profile = &mut ctx.accounts.learner_profile;
    
    // Check not already claimed
    require!(
        !learner_profile.is_achievement_claimed(achievement_index),
        AcademyError::AchievementAlreadyClaimed
    );
    
    // Cap XP reward
    let capped_reward = xp_reward.min(config.max_achievement_xp);
    
    // Check daily rate limit
    check_and_update_daily_xp(learner_profile, config, capped_reward)?;
    
    // Mark achievement claimed
    learner_profile.claim_achievement(achievement_index);
    
    // TODO: Mint XP tokens to learner_token
    // This requires permanent_delegate authority which the config PDA should have
    
    msg!(
        "Achievement claimed: index={}, xp={}",
        achievement_index,
        capped_reward
    );
    
    Ok(())
}
