use anchor_lang::prelude::*;

use crate::state::*;
use crate::error::AcademyError;

/// Issue or upgrade credential NFT
#[derive(Accounts)]
#[instruction(metadata_uri: String)]
pub struct IssueCredential<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    
    /// Backend signer
    pub backend_signer: Signer<'info>,
    
    /// Config PDA
    #[account(
        seeds = [Config::SEED],
        bump = config.bump,
        has_one = backend_signer @ AcademyError::Unauthorized,
    )]
    pub config: Account<'info, Config>,
    
    /// Course PDA
    #[account(
        seeds = Course::seeds(&course.course_id),
        bump = course.bump,
    )]
    pub course: Account<'info, Course>,
    
    /// Learner wallet
    pub learner: SystemAccount<'info>,
    
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
    
    /// System program
    pub system_program: Program<'info, System>,
}

pub fn issue_credential(
    ctx: Context<IssueCredential>,
    metadata_uri: String,
) -> Result<()> {
    let course = &ctx.accounts.course;
    let enrollment = &mut ctx.accounts.enrollment;
    
    // Must be finalized
    require!(
        enrollment.completed_at.is_some(),
        AcademyError::CourseNotFinalized
    );
    
    let is_new = enrollment.credential_asset.is_none();
    
    if is_new {
        // Create new credential NFT
        // TODO: CPI to Metaplex Core to mint new NFT
        // Store asset pubkey in enrollment.credential_asset
        
        enrollment.credential_asset = Some(Pubkey::default()); // Placeholder
        
        msg!(
            "Credential created for track {} level {}",
            course.track_id,
            course.track_level
        );
    } else {
        // Upgrade existing credential
        // TODO: CPI to Metaplex Core to update URI and attributes
        
        msg!(
            "Credential upgraded for track {} to level {}",
            course.track_id,
            course.track_level
        );
    }
    
    Ok(())
}
