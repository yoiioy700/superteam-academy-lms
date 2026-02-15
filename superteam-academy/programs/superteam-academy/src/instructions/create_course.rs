use anchor_lang::prelude::*;

use crate::state::*;
use crate::error::AcademyError;

/// Register a new course
#[derive(Accounts)]
#[instruction(course_id: String, params: CreateCourseParams)]
pub struct CreateCourse<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    
    /// Config PDA
    #[account(
        seeds = [Config::SEED],
        bump = config.bump,
        has_one = authority @ AcademyError::Unauthorized,
    )]
    pub config: Account<'info, Config>,
    
    /// Platform authority
    pub authority: Signer<'info>,
    
    /// Course PDA
    #[account(
        init,
        payer = payer,
        space = Course::SIZE,
        seeds = Course::seeds(&course_id),
        bump,
    )]
    pub course: Account<'info, Course>,
    
    /// Optional prerequisite course
    pub prerequisite: Option<Account<'info, Course>>,
    
    pub system_program: Program<'info, System>,
}

/// Params for create_course
#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CreateCourseParams {
    pub creator: Pubkey,
    pub authority: Pubkey,
    pub content_tx_id: [u8; 32],
    pub lesson_count: u8,
    pub difficulty: u8,
    pub xp_per_lesson: u32,
    pub track_id: u16,
    pub track_level: u8,
    pub prerequisite: Option<Pubkey>,
    pub completion_bonus_xp: u32,
    pub creator_reward_xp: u32,
    pub min_completions_for_reward: u16,
}

pub fn create_course(
    ctx: Context<CreateCourse>,
    course_id: String,
    params: CreateCourseParams,
) -> Result<()> {
    require!(
        course_id.len() <= Course::MAX_COURSE_ID_LEN,
        AcademyError::CourseIdTooLong
    );
    
    require!(
        params.difficulty >= 1 && params.difficulty <= 3,
        AcademyError::InvalidDifficulty
    );
    
    require!(
        params.track_level >= 1 && params.track_level <= 3,
        AcademyError::InvalidTrackLevel
    );
    
    let course = &mut ctx.accounts.course;
    let now = Clock::get()?.unix_timestamp;
    
    course.course_id = course_id.clone();
    course.creator = params.creator;
    course.authority = params.authority;
    course.content_tx_id = params.content_tx_id;
    course.version = 1;
    course.lesson_count = params.lesson_count;
    course.difficulty = params.difficulty;
    course.xp_per_lesson = params.xp_per_lesson;
    course.track_id = params.track_id;
    course.track_level = params.track_level;
    course.prerequisite = params.prerequisite;
    course.completion_bonus_xp = params.completion_bonus_xp;
    course.creator_reward_xp = params.creator_reward_xp;
    course.min_completions_for_reward = params.min_completions_for_reward;
    course.total_completions = 0;
    course.total_enrollments = 0;
    course.is_active = true;
    course.created_at = now;
    course.updated_at = now;
    course._reserved = [0; 16];
    course.bump = ctx.bumps.course;
    
    msg!("Course created: {}", course_id);
    msg!("Creator: {}", course.creator);
    
    Ok(())
}
