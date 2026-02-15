use anchor_lang::prelude::*;

use crate::state::*;
use crate::error::AcademyError;

/// Finalize entire course: verify completion, award creator XP
#[derive(Accounts)]
pub struct FinalizeCourse<'info> {
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
    
    /// Course creator
    pub creator: SystemAccount<'info>,
    
    /// Learner wallet
    pub learner: SystemAccount<'info>,
    
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

pub fn finalize_course(ctx: Context<FinalizeCourse>) -> Result<()> {
    let course = &mut ctx.accounts.course;
    let enrollment = &mut ctx.accounts.enrollment;
    let now = Clock::get()?.unix_timestamp;
    
    // Must not already be finalized
    require!(
        enrollment.completed_at.is_none(),
        AcademyError::CourseAlreadyFinalized
    );
    
    // Verify all lessons completed
    require!(
        enrollment.is_course_completed(course.lesson_count),
        AcademyError::CourseNotCompleted
    );
    
    // Mark as completed
    enrollment.completed_at = Some(now);
    
    // Increment course completions
    course.total_completions = course
        .total_completions
        .checked_add(1)
        .ok_or(AcademyError::Overflow)?;
    
    // Award creator XP if threshold met
    if course.total_completions >= course.min_completions_for_reward as u32 {
        msg!(
            "Creator reward: {} XP to {}",
            course.creator_reward_xp,
            ctx.accounts.creator.key()
        );
        // TODO: Mint XP to creator token account
    }
    
    msg!(
        "Course finalized: {} by {}",
        course.course_id,
        ctx.accounts.learner.key()
    );
    
    Ok(())
}
