use anchor_lang::prelude::*;

use crate::state::*;
use crate::error::AcademyError;

/// Register a referral
#[derive(Accounts)]
pub struct RegisterReferral<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    
    /// The new learner
    pub learner: Signer<'info>,
    
    /// LearnerProfile PDA (referrer)
    #[account(
        mut,
        seeds = [LearnerProfile::SEED, referrer.key().as_ref()],
        bump = referrer_profile.bump,
    )]
    pub referrer_profile: Account<'info, LearnerProfile>,
    
    /// Referrer wallet
    pub referrer: SystemAccount<'info>,
    
    /// LearnerProfile PDA (new learner)
    #[account(
        mut,
        seeds = [LearnerProfile::SEED, learner.key().as_ref()],
        bump = learner_profile.bump,
    )]
    pub learner_profile: Account<'info, LearnerProfile>,
}

pub fn register_referral(ctx: Context<RegisterReferral>) -> Result<()> {
    let learner_key = ctx.accounts.learner.key();
    let referrer_key = ctx.accounts.referrer.key();
    
    // Cannot refer self
    require!(
        learner_key != referrer_key,
        AcademyError::SelfReferral
    );
    
    // Cannot have existing referrer
    require!(
        !ctx.accounts.learner_profile.has_referrer,
        AcademyError::AlreadyReferred
    );
    
    // Referrer must exist (already has profile)
    // This is implicitly checked by the account constraint
    
    // Update learner
    ctx.accounts.learner_profile.has_referrer = true;
    
    // Update referrer
    ctx.accounts.referrer_profile.referral_count = ctx.accounts.referrer_profile
        .referral_count
        .checked_add(1)
        .ok_or(AcademyError::Overflow)?;
    
    msg!("Referral registered: {} -> {}", learner_key, referrer_key);
    
    Ok(())
}
