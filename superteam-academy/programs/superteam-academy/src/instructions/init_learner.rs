use anchor_lang::prelude::*;

use crate::state::*;
use crate::error::AcademyError;

/// Initialize learner profile
#[derive(Accounts)]
pub struct InitLearner<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    
    /// Learner wallet
    pub learner: Signer<'info>,
    
    /// LearnerProfile PDA
    #[account(
        init,
        payer = payer,
        space = LearnerProfile::SIZE,
        seeds = [LearnerProfile::SEED, learner.key().as_ref()],
        bump,
    )]
    pub profile: Account<'info, LearnerProfile>,
    
    pub system_program: Program<'info, System>,
}

pub fn init_learner(ctx: Context<InitLearner>) -> Result<()> {
    let profile = &mut ctx.accounts.profile;
    let now = Clock::get()?.unix_timestamp;
    
    profile.authority = ctx.accounts.learner.key();
    profile.current_streak = 0;
    profile.longest_streak = 0;
    profile.last_activity_date = now;
    profile.streak_freezes = 0;
    profile.achievement_flags = [0; 4];
    profile.xp_earned_today = 0;
    profile.last_xp_day = 0;
    profile.referral_count = 0;
    profile.has_referrer = false;
    profile._reserved = [0; 16];
    profile.bump = ctx.bumps.profile;
    
    msg!("Learner initialized: {}", profile.authority);
    
    Ok(())
}
