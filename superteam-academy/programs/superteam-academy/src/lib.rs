use anchor_lang::prelude::*;

pub mod state;
pub mod instructions;
pub mod error;
pub mod utils;

use state::*;
use instructions::*;
use error::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod superteam_academy {
    use super::*;
    
    // ═══════════════════════════════════════════════════════════════
    // PLATFORM MANAGEMENT
    // ═══════════════════════════════════════════════════════════════
    
    /// Initialize platform
    pub fn initialize(
        ctx: Context<Initialize>,
        params: InitializeParams,
    ) -> Result<()> {
        instructions::initialize(ctx, params)
    }
    
    /// Create new season
    pub fn create_season(
        ctx: Context<CreateSeason>,
        season: u16,
    ) -> Result<()> {
        instructions::create_season(ctx, season)
    }
    
    /// Close current season
    pub fn close_season(ctx: Context<CloseSeason>) -> Result<()> {
        instructions::close_season(ctx)
    }
    
    /// Update config
    pub fn update_config(
        ctx: Context<UpdateConfig>,
        params: UpdateConfigParams,
    ) -> Result<()> {
        instructions::update_config(ctx, params)
    }
    
    // ═══════════════════════════════════════════════════════════════
    // COURSES
    // ═══════════════════════════════════════════════════════════════
    
    /// Create course
    pub fn create_course(
        ctx: Context<CreateCourse>,
        course_id: String,
        params: CreateCourseParams,
    ) -> Result<()> {
        instructions::create_course(ctx, course_id, params)
    }
    
    /// Update course
    pub fn update_course(
        ctx: Context<UpdateCourse>,
        params: UpdateCourseParams,
    ) -> Result<()> {
        instructions::update_course(ctx, params)
    }
    
    // ═══════════════════════════════════════════════════════════════
    // LEARNERS
    // ═══════════════════════════════════════════════════════════════
    
    /// Initialize learner profile
    pub fn init_learner(ctx: Context<InitLearner>) -> Result<()> {
        instructions::init_learner(ctx)
    }
    
    /// Register referral
    pub fn register_referral(ctx: Context<RegisterReferral>) -> Result<()> {
        instructions::register_referral(ctx)
    }
    
    /// Claim achievement
    pub fn claim_achievement(
        ctx: Context<ClaimAchievement>,
        achievement_index: u8,
        xp_reward: u32,
    ) -> Result<()> {
        instructions::claim_achievement(ctx, achievement_index, xp_reward)
    }
    
    /// Award streak freeze
    pub fn award_streak_freeze(ctx: Context<AwardStreakFreeze>) -> Result<()> {
        instructions::award_streak_freeze(ctx)
    }
    
    // ═══════════════════════════════════════════════════════════════
    // ENROLLMENT & PROGRESS
    // ═══════════════════════════════════════════════════════════════
    
    /// Enroll in course
    pub fn enroll(
        ctx: Context<Enroll>,
        course_id: String,
    ) -> Result<()> {
        instructions::enroll(ctx, course_id)
    }
    
    /// Complete lesson
    pub fn complete_lesson(
        ctx: Context<CompleteLesson>,
        lesson_index: u8,
    ) -> Result<()> {
        instructions::complete_lesson(ctx, lesson_index)
    }
    
    /// Finalize course
    pub fn finalize_course(ctx: Context<FinalizeCourse>) -> Result<()> {
        instructions::finalize_course(ctx)
    }
    
    /// Claim completion bonus
    pub fn claim_completion_bonus(
        ctx: Context<ClaimCompletionBonus>,
    ) -> Result<()> {
        instructions::claim_completion_bonus(ctx)
    }
    
    /// Issue credential
    pub fn issue_credential(
        ctx: Context<IssueCredential>,
        metadata_uri: String,
    ) -> Result<()> {
        instructions::issue_credential(ctx, metadata_uri)
    }
    
    /// Close enrollment
    pub fn close_enrollment(ctx: Context<CloseEnrollment>) -> Result<()> {
        instructions::close_enrollment(ctx)
    }
}
