use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Token2022, TokenAccount, Mint, MintTo};

use crate::state::*;
use crate::error::AcademyError;

/// Finalize entire course: verify completion, award creator XP
#[derive(Accounts)]
pub struct FinalizeCourse<'info> {
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
    
    /// Course creator
    /// CHECK: Used for token account
    pub creator: AccountInfo<'info>,
    
    /// Learner wallet
    /// CHECK: Used for PDA
    pub learner: AccountInfo<'info>,
    
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
    
    /// Creator's XP token account
    #[account(
        mut,
        token::mint = xp_mint,
        token::authority = creator,
    )]
    pub creator_token: InterfaceAccount<'info, TokenAccount>,
    
    /// Config PDA as mint authority
    /// CHECK: Derived from config PDA
    #[account(
        seeds = [Config::SEED],
        bump = config.bump,
    )]
    pub config_pda: AccountInfo<'info>,
    
    pub token_program: Program<'info, Token2022>,
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
        let creator_xp = course.creator_reward_xp as u64;
        if creator_xp > 0 {
            let config_seeds = &[Config::SEED, &[ctx.accounts.config.bump]];
            let signer_seeds = &[&config_seeds[..]];
            
            let cpi_accounts = MintTo {
                mint: ctx.accounts.xp_mint.to_account_info(),
                to: ctx.accounts.creator_token.to_account_info(),
                authority: ctx.accounts.config_pda.clone(),
            };
            let cpi_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                cpi_accounts,
                signer_seeds,
            );
            anchor_spl::token_interface::mint_to(cpi_ctx, creator_xp)?;
            
            msg!(
                "Creator reward: {} XP to {}",
                creator_xp,
                ctx.accounts.creator.key()
            );
        }
    }
    
    msg!(
        "Course finalized: {} by {}",
        course.course_id,
        ctx.accounts.learner.key()
    );
    
    Ok(())
}
