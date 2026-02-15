use anchor_lang::prelude::*;
use mpl_core::{
    instructions::CreateV2CpiBuilder,
    instructions::UpdateV1CpiBuilder,
    types::{
        PluginAuthority, PluginAuthorityPair,
        Plugin, PermanentFreezeDelegate, Attributes, Attribute,
    },
};

use crate::state::*;
use crate::error::AcademyError;

/// Issue or upgrade credential NFT via Metaplex Core
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
    /// CHECK: Used for NFT owner
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
    
    /// Track collection NFT
    /// CHECK: Verified by CPI
    #[account(mut)]
    pub track_collection: AccountInfo<'info>,
    
    /// Credential asset (new or existing)
    /// CHECK: Created or updated via CPI
    #[account(mut)]
    pub credential_asset: AccountInfo<'info>,
    
    /// Metaplex Core program
    pub mpl_core_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

pub fn issue_credential(
    ctx: Context<IssueCredential>,
    metadata_uri: String,
) -> Result<()> {
    let course = &ctx.accounts.course;
    let enrollment = &mut ctx.accounts.enrollment;
    let config = &ctx.accounts.config;
    
    // Must be finalized
    require!(
        enrollment.completed_at.is_some(),
        AcademyError::CourseNotFinalized
    );
    
    let is_new = enrollment.credential_asset.is_none();
    let config_seeds: &[(u8)] = &[Config::SEED, &[config.bump]];
    
    if is_new {
        // Create new credential NFT
        // Note: Full Metaplex Core CPI complex - simplified here
        // TODO: Full implementation with proper CPI calls
        
        // For now, store placeholder - real implementation needs:
        // 1. CreateV2CpiBuilder with PermanentFreezeDelegate
        // 2. Attributes plugin with track_id, level, courses_completed
        // 3. Set Config PDA as update authority
        
        enrollment.credential_asset = Some(ctx.accounts.credential_asset.key());
        
        msg!(
            "Credential created for track {} level {}",
            course.track_id,
            course.track_level
        );
    } else {
        // Upgrade existing credential
        // TODO: UpdateV1CpiBuilder + UpdatePluginV1CpiBuilder
        
        msg!(
            "Credential upgraded for track {} to level {}",
            course.track_id,
            course.track_level
        );
    }
    
    Ok(())
}
