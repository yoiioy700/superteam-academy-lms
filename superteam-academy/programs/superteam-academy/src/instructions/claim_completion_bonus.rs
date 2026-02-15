use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Token2022, TokenAccount, Mint, MintTo};

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
    
    // Mint completion bonus XP
    let bonus_amount = course.completion_bonus_xp as u64;
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
    anchor_spl::token_interface::mint_to(cpi_ctx, bonus_amount)?;
    
    msg!(
        "Completion bonus claimed: {} XP for {}",
        bonus_amount,
        course.course_id
    );
    
    Ok(())
}
