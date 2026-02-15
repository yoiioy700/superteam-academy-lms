use anchor_lang::prelude::*;

/// Course PDA - Course registry
/// Seeds: ["course", course_id.as_bytes()]
#[account]
pub struct Course {
    /// Unique course identifier (slug, max 32 chars)
    pub course_id: String, // 4 + 32 bytes
    
    /// Course creator (earns XP on completions)
    pub creator: Pubkey,
    
    /// Who can update course content
    pub authority: Pubkey,
    
    /// Arweave transaction ID (32 bytes raw)
    pub content_tx_id: [u8; 32],
    
    /// Content version
    pub version: u16,
    
    /// Total lessons in course
    pub lesson_count: u8,
    
    /// Difficulty: 1=beginner, 2=intermediate, 3=advanced
    pub difficulty: u8,
    
    /// Per-lesson XP
    pub xp_per_lesson: u32,
    
    /// Track ID (0=standalone, 1=anchor, 2=rust, etc.)
    pub track_id: u16,
    
    /// Level within track (1=beginner, 2=intermediate, 3=advanced)
    pub track_level: u8,
    
    /// Optional prerequisite course PDA
    pub prerequisite: Option<Pubkey>,
    
    /// Bonus XP on course completion
    pub completion_bonus_xp: u32,
    
    /// XP to creator per student completion
    pub creator_reward_xp: u32,
    
    /// Minimum completions before creator earns
    pub min_completions_for_reward: u16,
    
    /// Total completions
    pub total_completions: u32,
    
    /// Total enrollments
    pub total_enrollments: u32,
    
    /// Accepts new enrollments
    pub is_active: bool,
    
    /// Creation timestamp
    pub created_at: i64,
    
    /// Last update timestamp
    pub updated_at: i64,
    
    /// Reserved for future use
    pub _reserved: [u8; 16],
    
    /// PDA bump
    pub bump: u8,
}

impl Course {
    pub const MAX_COURSE_ID_LEN: usize = 32;
    pub const SIZE: usize = 8 + // discriminator
        4 + Self::MAX_COURSE_ID_LEN + // course_id (String overhead + max chars)
        32 + // creator
        32 + // authority
        32 + // content_tx_id
        2 +  // version
        1 +  // lesson_count
        1 +  // difficulty
        4 +  // xp_per_lesson
        2 +  // track_id
        1 +  // track_level
        33 + // prerequisite (Option<Pubkey> = 1 + 32)
        4 +  // completion_bonus_xp
        4 +  // creator_reward_xp
        2 +  // min_completions_for_reward
        4 +  // total_completions
        4 +  // total_enrollments
        1 +  // is_active
        8 +  // created_at
        8 +  // updated_at
        16 + // reserved
        1;   // bump
    
    pub fn seeds(course_id: &str) -> Vec<&[u8]> {
        vec![b"course", course_id.as_bytes()]
    }
}
