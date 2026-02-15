use anchor_lang::prelude::*;

use crate::state::*;
use crate::error::AcademyError;

/// Award a streak freeze to learner
#[derive(Accounts)]
pub struct AwardStreakFreeze<'info> {
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
}

pub fn award_streak_freeze(ctx: Context<AwardStreakFreeze>) -> Result<()> {
    let learner_profile = &mut ctx.accounts.learner_profile;
    
    // Increment streak freezes
    learner_profile.streak_freezes = learner_profile
        .streak_freezes
        .checked_add(1)
        .ok_or(AcademyError::Overflow)?;
    
    msg!(
        "Streak freeze awarded to: {}, total: {}",
        ctx.accounts.learner.key(),
        learner_profile.streak_freezes
    );
    
    Ok(())
}
