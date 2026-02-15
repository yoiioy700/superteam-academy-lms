use anchor_lang::prelude::*;

use crate::state::*;
use crate::error::AcademyError;

/// Close enrollment (completed or abandoned)
#[derive(Accounts)]
pub struct CloseEnrollment<'info> {
    /// Learner wallet (receives rent back)
    #[account(mut)]
    pub learner: Signer<'info>,
    
    /// Course PDA
    #[account(
        seeds = Course::seeds(&course.course_id),
        bump = course.bump,
    )]
    pub course: Account<'info, Course>,
    
    /// Enrollment PDA to close
    #[account(
        mut,
        seeds = [
            b"enrollment",
            course.course_id.as_bytes(),
            learner.key().as_ref(),
        ],
        bump = enrollment.bump,
        close = learner, // Return rent to learner
    )]
    pub enrollment: Account<'info, Enrollment>,
}

pub fn close_enrollment(ctx: Context<CloseEnrollment>) -> Result<()> {
    let enrollment = &ctx.accounts.enrollment;
    let now = Clock::get()?.unix_timestamp;
    
    let is_completed = enrollment.completed_at.is_some();
    
    if !is_completed {
        // Must wait 24h cooldown for abandoned courses
        require!(
            now - enrollment.enrolled_at >= 86400,
            AcademyError::UnenrollCooldown
        );
    }
    
    msg!(
        "Enrollment closed: {} completed={}",
        ctx.accounts.course.course_id,
        is_completed
    );
    
    // Account closed automatically by Anchor close constraint
    // Rent returned to learner
    
    Ok(())
}
