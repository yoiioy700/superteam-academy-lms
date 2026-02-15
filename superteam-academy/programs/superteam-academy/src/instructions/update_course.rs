use anchor_lang::prelude::*;

use crate::state::*;
use crate::error::AcademyError;

/// Update course content, rewards, or deactivate
#[derive(Accounts)]
pub struct UpdateCourse<'info> {
    /// Course PDA
    #[account(
        mut,
        seeds = Course::seeds(&course.course_id),
        bump = course.bump,
        has_one = authority @ AcademyError::Unauthorized,
    )]
    pub course: Account<'info, Course>,
    
    /// Course authority
    pub authority: Signer<'info>,
}

/// Params for update_course
#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct UpdateCourseParams {
    pub content_tx_id: Option<[u8; 32]>,
    pub is_active: Option<bool>,
    pub completion_bonus_xp: Option<u32>,
    pub creator_reward_xp: Option<u32>,
    pub min_completions_for_reward: Option<u16>,
}

pub fn update_course(
    ctx: Context<UpdateCourse>,
    params: UpdateCourseParams,
) -> Result<()> {
    let course = &mut ctx.accounts.course;
    let now = Clock::get()?.unix_timestamp;
    
    if let Some(content_tx_id) = params.content_tx_id {
        course.content_tx_id = content_tx_id;
        course.version = course.version.checked_add(1)
            .ok_or(AcademyError::Overflow)?;
    }
    
    if let Some(is_active) = params.is_active {
        course.is_active = is_active;
    }
    
    if let Some(completion_bonus_xp) = params.completion_bonus_xp {
        course.completion_bonus_xp = completion_bonus_xp;
    }
    
    if let Some(creator_reward_xp) = params.creator_reward_xp {
        course.creator_reward_xp = creator_reward_xp;
    }
    
    if let Some(min_completions) = params.min_completions_for_reward {
        course.min_completions_for_reward = min_completions;
    }
    
    course.updated_at = now;
    
    msg!("Course updated: {}", course.course_id);
    msg!("Version: {}", course.version);
    
    Ok(())
}
