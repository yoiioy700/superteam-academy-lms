use anchor_lang::prelude::*;
use mpl_core::instructions::CreateV2CpiBuilder;
use mpl_core::instructions::UpdateV1CpiBuilder;
use mpl_core::instructions::UpdatePluginV1CpiBuilder;
use mpl_core::types::{
    PluginAuthorityPair, Plugin,
    PermanentFreezeDelegate, Attributes, Attribute,
    PluginAuthority,
    UpdateAuthority,
};

use crate::state::*;
use crate::error::AcademyError;
use crate::events::CredentialIssued;

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
        mut,
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
    /// CHECK: Verified by MetaPlex Core, PDA seeds checked off-chain
    #[account(mut)]
    pub track_collection: AccountInfo<'info>,
    
    /// Credential asset (new or existing)
    /// CHECK: Created or updated via CPI
    #[account(mut)]
    pub credential_asset: AccountInfo<'info>,
    
    /// Metaplex Core program
    /// CHECK: Metaplex Core
    pub mpl_core_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

/// Level names for credential displayn level_name(level: u8) -> &'static str {
    match level {
        0 => "Beginner",
        1 => "Beginner",
        2 => "Intermediate",
        3 => "Advanced",
        _ => "Expert",
    }
}

/// Track names (hardcoded V1)
fn track_name(track_id: u16) -> String {
    match track_id {
        0 => "Standalone".to_string(),
        1 => "Anchor Framework".to_string(),
        2 => "Rust for Solana".to_string(),
        3 => "DeFi Development".to_string(),
        4 => "Program Security".to_string(),
        _ => format!("Track {}", track_id),
    }
}

pub fn issue_credential(
    ctx: Context<IssueCredential>,
    metadata_uri: String,
) -> Result<()> {
    let course = &ctx.accounts.course;
    let enrollment = &mut ctx.accounts.enrollment;
    let config = &ctx.accounts.config;
    let now = Clock::get()?.unix_timestamp;
    
    // Must be finalized
    require!(
        enrollment.completed_at.is_some(),
        AcademyError::CourseNotFinalized
    );
    
    let is_new = enrollment.credential_asset.is_none();
    let config_seeds = &[ Config::SEED, &[config.bump] ];
    let config_signer_seeds = &[&config_seeds[..]];
    let track = track_name(course.track_id);
    let level = level_name(course.track_level);
    let display_name = format!("{} — {}", track, level);
    
    if is_new {
        // Create new credential NFT
        // CPI: CreateV2 with plugins
        let create_builder = CreateV2CpiBuilder::new(
            &ctx.accounts.mpl_core_program
        );
        
        // Setup the create instruction
        create_builder
            .asset(&ctx.accounts.credential_asset)
            .collection(Some(&ctx.accounts.track_collection))
            .payer(&ctx.accounts.payer)
            .owner(Some(&ctx.accounts.learner))
            .update_authority(UpdateAuthority::Address(config.key()))
            .name(display_name.clone())
            .uri(metadata_uri.clone())
            .plugins(vec![
                PluginAuthorityPair {
                    plugin: Plugin::PermanentFreezeDelegate(
                        PermanentFreezeDelegate { frozen: true }
                    ),
                    authority: Some(PluginAuthority::UpdateAuthority),
                },
                PluginAuthorityPair {
                    plugin: Plugin::Attributes(Attributes {
                        attribute_list: vec![
                            Attribute { 
                                key: "track_id".into(), 
                                value: course.track_id.to_string(),
                            },
                            Attribute { 
                                key: "track_name".into(), 
                                value: track.clone(),
                            },
                            Attribute { 
                                key: "level".into(), 
                                value: course.track_level.to_string(),
                            },
                            Attribute { 
                                key: "level_name".into(), 
                                value: level.to_string(),
                            },
                            Attribute { 
                                key: "courses_completed".into(), 
                                value: "1".to_string(),
                            },
                        ],
                    }),
                    authority: Some(PluginAuthority::UpdateAuthority),
                },
            ])
            .invoke_signed(config_signer_seeds)
            .map_err(|_| AcademyError::Unauthorized)?;
        
        // Store asset address
        enrollment.credential_asset = Some(ctx.accounts.credential_asset.key());
        
        // Emit event
        emit!(CredentialIssued {
            learner: ctx.accounts.learner.key(),
            track_id: course.track_id,
            credential_asset: ctx.accounts.credential_asset.key(),
            credential_created: true,
            credential_upgraded: false,
            current_level: course.track_level,
            timestamp: now,
        });
        
        msg!(
            "Credential created: {} for track {} level {}",
            display_name,
            course.track_id,
            course.track_level
        );
    } else {
        // Upgrade existing credential
        if let Some(asset_key) = enrollment.credential_asset {
            // Count completed courses (simplification - in real app, query all enrollments)
            let new_courses_count = course.track_level; // Approximation
            
            // Update name and URI
            let update_builder = UpdateV1CpiBuilder::new(
                &ctx.accounts.mpl_core_program
            );
            
            update_builder
                .asset(&ctx.accounts.credential_asset)
                .collection(Some(&ctx.accounts.track_collection))
                .authority(Some(&ctx.accounts.config))
                .new_name(Some(display_name.clone()))
                .new_uri(Some(metadata_uri))
                .invoke_signed(config_signer_seeds)
                .map_err(|_| AcademyError::Unauthorized)?;
            
            // Update attributes
            let update_plugin = UpdatePluginV1CpiBuilder::new(
                &ctx.accounts.mpl_core_program
            );
            
            update_plugin
                .asset(&ctx.accounts.credential_asset)
                .collection(Some(&ctx.accounts.track_collection))
                .authority(Some(&ctx.accounts.config))
                .plugin(Plugin::Attributes(Attributes {
                    attribute_list: vec![
                        Attribute { 
                            key: "level".into(), 
                            value: course.track_level.to_string(),
                        },
                        Attribute { 
                            key: "level_name".into(), 
                            value: level.to_string(),
                        },
                        Attribute { 
                            key: "courses_completed".into(), 
                            value: new_courses_count.to_string(),
                        },
                    ],
                }))
                .invoke_signed(config_signer_seeds)
                .map_err(|_| AcademyError::Unauthorized)?;
            
            // Emit event
            emit!(CredentialIssued {
                learner: ctx.accounts.learner.key(),
                track_id: course.track_id,
                credential_asset: asset_key,
                credential_created: false,
                credential_upgraded: true,
                current_level: course.track_level,
                timestamp: now,
            });
            
            msg!(
                "Credential upgraded: {} to level {}",
                display_name,
                course.track_level
            );
        }
    }
    
    Ok(())
}
