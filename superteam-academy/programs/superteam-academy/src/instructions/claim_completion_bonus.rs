use anchor_lang::prelude::*;

use crate::state::*;
use crate::error::AcademyError;
use crate::utils::check_and_update_daily_xp;

/// Claim completion bonus XP
#[derive(Accounts)]
pub struct ClaimCompletionBonus<'info> {
    /// Learner wallet
    pub learner: Signer<'info>,
    
    /// Config PDA
    #[account(
        seeds = [Config::SEED],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,
    
    /// Course PDA
    #[account(
        seeds = Course::seeds(&course.course_id),
        bump = course.bump,
    )]
    pub course: Account<'info, Course>,
    
    /// LearnerProfile PDA
    #[account(
        mut,
        seeds = [LearnerProfile::SEED, learner.key().as_ref()],
        bump = learner_profile.bump,
    )]
    pub learner_profile: Account<'info, LearnerProfile>,
    
    /// Enrollment PDA
    #[account(
        mut,
        seeds = [
            b"enrollment",
            course.course_id.as_bytes(),
            learner.key().as_ref(),
        ],
        bump = enrollment.bump,
    )]
    pub enrollment: Account<'info, Enrollment>,
}

pub fn claim_completion_bonus(ctx: Context<ClaimCompletionBonus>) -> Result<()> {
    let course = &ctx.accounts.course;
    let enrollment = &mut ctx.accounts.enrollment;
    let learner_profile = &mut ctx.accounts.learner_profile;
    let config = &ctx.accounts.config;
    
    // Must be finalized
    require!(
        enrollment.completed_at.is_some(),
        AcademyError::CourseNotFinalized
    );
    
    // Not already claimed
    require!(
        !enrollment.bonus_claimed,
        AcademyError::BonusAlreadyClaimed
    );
    
    // Check daily cap
    check_and_update_daily_xp(learner_profile, config, course.completion_bonus_xp)?;
    
    // Mark claimed
    enrollment.bonus_claimed = true;
    
    // TODO: Mint completion bonus XP
    msg!(
        "Completion bonus claimed: {} XP for {}",
        course.completion_bonus_xp,
        course.course_id
    );
    
    Ok(())
}
