use anchor_lang::prelude::*;

use crate::state::*;
use crate::error::AcademyError;
use crate::utils::{update_streak, check_and_update_daily_xp};

/// Complete a lesson
#[derive(Accounts)]
#[instruction(lesson_index: u8)]
pub struct CompleteLesson<'info> {
    /// Backend signer
    pub backend_signer: Signer<'info>,
    
    /// Config PDA
    #[account(
        seeds = [Config::SEED],
        bump = config.bump,
        has_one = backend_signer @ AcademyError::Unauthorized,
    )]
    pub config: Account<'info, Config>,
    
    /// Course PDA
    #[account(
        mut,
        seeds = Course::seeds(&course.course_id),
        bump = course.bump,
    )]
    pub course: Account<'info, Course>,
    
    /// Learner wallet
    pub learner: SystemAccount<'info>,
    
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

pub fn complete_lesson(
    ctx: Context<CompleteLesson>,
    lesson_index: u8,
) -> Result<()> {
    let course = &ctx.accounts.course;
    let enrollment = &mut ctx.accounts.enrollment;
    let learner_profile = &mut ctx.accounts.learner_profile;
    let config = &ctx.accounts.config;
    
    // Check lesson bounds
    require!(
        lesson_index < course.lesson_count,
        AcademyError::LessonOutOfBounds
    );
    
    // Mark lesson complete (returns false if already completed)
    let is_new = enrollment.complete_lesson(lesson_index);
    require!(is_new, AcademyError::LessonAlreadyCompleted);
    
    // Check daily XP cap
    check_and_update_daily_xp(learner_profile, config, course.xp_per_lesson)?;
    
    // Update streak
    let streak_update = update_streak(learner_profile)?;
    
    msg!(
        "Lesson completed: {} - lesson {}",
        course.course_id,
        lesson_index
    );
    
    if let Some(update) = streak_update {
        match update {
            crate::utils::StreakUpdate::Continued { new_streak } => {
                msg!("Streak continued: {}", new_streak);
            }
            crate::utils::StreakUpdate::SavedByFreezes { freezes_used, new_streak } => {
                msg!("Streak saved by {} freezes, now: {}", freezes_used, new_streak);
            }
            crate::utils::StreakUpdate::Broken { old_streak, days_missed } => {
                msg!("Streak broken after {} days, missed {} days", old_streak, days_missed);
            }
        }
    }
    
    Ok(())
}
