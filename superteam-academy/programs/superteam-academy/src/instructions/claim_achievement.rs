use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Token2022, TokenAccount, Mint, MintTo};

use crate::state::*;
use crate::error::AcademyError;
use crate::utils::check_and_update_daily_xp;

/// Claim an achievement
#[derive(Accounts)]
#[instruction(achievement_index: u8, xp_reward: u32)]
pub struct ClaimAchievement<'info> {
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

pub fn claim_achievement(
    ctx: Context<ClaimAchievement>,
    achievement_index: u8,
    xp_reward: u32,
) -> Result<()> {
    require!(
        achievement_index < 256,
        AcademyError::AchievementOutOfBounds
    );
    
    let config = &mut ctx.accounts.config;
    let learner_profile = &mut ctx.accounts.learner_profile;
    
    // Check not already claimed
    require!(
        !learner_profile.is_achievement_claimed(achievement_index),
        AcademyError::AchievementAlreadyClaimed
    );
    
    // Cap XP reward
    let capped_reward = xp_reward.min(config.max_achievement_xp);
    
    // Check daily rate limit
    check_and_update_daily_xp(learner_profile, config, capped_reward)?;
    
    // Mark achievement claimed
    learner_profile.claim_achievement(achievement_index);
    
    // Mint XP tokens
    let xp_amount = capped_reward as u64;
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
        "Achievement claimed: index={}, xp={}",
        achievement_index,
        capped_reward
    );
    
    Ok(())
}
