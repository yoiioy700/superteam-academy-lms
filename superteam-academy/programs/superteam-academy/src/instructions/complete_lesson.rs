use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Token2022, TokenAccount, Mint, MintTo};
use anchor_spl::token_2022::spl_token_2022::instruction::AuthorityType;

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
        mut,
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
    /// CHECK: Verified by PDA
    pub learner: AccountInfo<'info>,
    
    /// LearnerProfile PDA
    #[account(
        mut,
        seeds = [LearnerProfile::SEED, learner.key().as_ref()],
        bump = learner_profile.bump,
        constraint = learner_profile.authority == learner.key() @ AcademyError::Unauthorized,
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
    
    /// XP Mint (Token-2022)
    #[account(
        mut,
        address = config.current_mint @ AcademyError::SeasonNotActive,
    )]
    pub xp_mint: InterfaceAccount<'info, Mint>,
    
    /// Learner's XP token account
    #[account(
        mut,
        token::mint = xp_mint,
        token::authority = learner,
    )]
    pub learner_token: InterfaceAccount<'info, TokenAccount>,
    
    /// Config PDA as mint authority
    /// CHECK: Derived from config PDA
    #[account(
        seeds = [Config::SEED],
        bump = config.bump,
    )]
    pub config_pda: AccountInfo<'info>,
    
    pub token_program: Program<'info, Token2022>,
}

pub fn complete_lesson(
    ctx: Context<CompleteLesson>,
    lesson_index: u8,
) -> Result<()> {
    let course = &ctx.accounts.course;
    let enrollment = &mut ctx.accounts.enrollment;
    let learner_profile = &mut ctx.accounts.learner_profile;
    let config = &ctx.accounts.config;
    
    // Check season not closed
    require!(!config.season_closed, AcademyError::SeasonClosed);
    
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
    
    // Mint XP tokens
    let xp_amount = course.xp_per_lesson as u64;
    let config_key = ctx.accounts.config.key();
    let config_seeds = &[Config::SEED, &[config.bump]];
    let signer_seeds = &[&config_seeds[..]];
    
    let cpi_accounts = MintTo {
        mint: ctx.accounts.xp_mint.to_account_info(),
        to: ctx.accounts.learner_token.to_account_info(),
        authority: ctx.accounts.config_pda.clone(),
    };
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        signer_seeds,
    );
    anchor_spl::token_interface::mint_to(cpi_ctx, xp_amount)?;
    
    msg!(
        "Lesson completed: {} - lesson {}, earned {} XP",
        course.course_id,
        lesson_index,
        xp_amount
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
