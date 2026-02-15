use anchor_lang::prelude::*;

use crate::state::*;
use crate::error::AcademyError;

/// Enroll in a course
#[derive(Accounts)]
#[instruction(course_id: String)]
pub struct Enroll<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    
    /// Learner wallet
    pub learner: Signer<'info>,
    
    /// LearnerProfile PDA
    #[account(
        seeds = [LearnerProfile::SEED, learner.key().as_ref()],
        bump = learner_profile.bump,
    )]
    pub learner_profile: Account<'info, LearnerProfile>,
    
    /// Course PDA
    #[account(
        seeds = Course::seeds(&course.course_id),
        bump = course.bump,
    )]
    pub course: Account<'info, Course>,
    
    /// Enrollment PDA
    #[account(
        init,
        payer = payer,
        space = Enrollment::SIZE,
        seeds = [
            b"enrollment",
            course_id.as_bytes(),
            learner.key().as_ref(),
        ],
        bump,
    )]
    pub enrollment: Account<'info, Enrollment>,
    
    /// Optional prerequisite enrollment
    pub prerequisite_enrollment: Option<Account<'info, Enrollment>>,
    
    pub system_program: Program<'info, System>,
}

pub fn enroll(ctx: Context<Enroll>, course_id: String) -> Result<()> {
    let course = &ctx.accounts.course;
    let enrollment = &mut ctx.accounts.enrollment;
    let now = Clock::get()?.unix_timestamp;
    
    // Course must be active
    require!(course.is_active, AcademyError::CourseNotActive);
    
    // Check prerequisite if set
    if let Some(prerequisite) = course.prerequisite {
        require!(
            ctx.accounts.prerequisite_enrollment.is_some(),
            AcademyError::PrerequisiteNotMet
        );
        
        let prereq = ctx.accounts.prerequisite_enrollment.as_ref().unwrap();
        require!(
            prereq.completed_at.is_some(),
            AcademyError::PrerequisiteNotMet
        );
    }
    
    // Initialize enrollment
    enrollment.course = ctx.accounts.course.key();
    enrollment.enrolled_version = course.version;
    enrollment.enrolled_at = now;
    enrollment.completed_at = None;
    enrollment.lesson_flags = [0; 4];
    enrollment.credential_asset = None;
    enrollment.bonus_claimed = false;
    enrollment._reserved = [0; 7];
    enrollment.bump = ctx.bumps.enrollment;
    
    msg!("Enrolled: {} in {}", ctx.accounts.learner.key(), course_id);
    
    Ok(())
}
