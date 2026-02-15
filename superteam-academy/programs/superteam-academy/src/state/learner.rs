use anchor_lang::prelude::*;

/// LearnerProfile PDA - User progress tracking
/// Seeds: ["learner", user_pubkey]
#[account]
pub struct LearnerProfile {
    /// Learner wallet
    pub authority: Pubkey,
    
    /// Current consecutive-day streak
    pub current_streak: u16,
    
    /// Longest streak ever
    pub longest_streak: u16,
    
    /// Last activity (unix timestamp)
    pub last_activity_date: i64,
    
    /// Available streak freezes
    pub streak_freezes: u8,
    
    /// Bitmap of claimed achievements (256 possible)
    pub achievement_flags: [u64; 4],
    
    /// XP earned today
    pub xp_earned_today: u32,
    
    /// Day number of last XP earn (unix_ts / 86400)
    pub last_xp_day: u16,
    
    /// Number of successful referrals
    pub referral_count: u16,
    
    /// Has referrer
    pub has_referrer: bool,
    
    /// Reserved
    pub _reserved: [u8; 16],
    
    /// PDA bump
    pub bump: u8,
}

impl LearnerProfile {
    pub const SIZE: usize = 8 + // discriminator
        32 + // authority
        2 +  // current_streak
        2 +  // longest_streak
        8 +  // last_activity_date
        1 +  // streak_freezes
        32 + // achievement_flags ([u64; 4])
        4 +  // xp_earned_today
        2 +  // last_xp_day
        2 +  // referral_count
        1 +  // has_referrer
        16 + // reserved
        1;   // bump
    
    pub const SEED: &'static [u8] = b"learner";
    
    /// Check if achievement is claimed
    pub fn is_achievement_claimed(&self, index: u8) -> bool {
        let word = (index / 64) as usize;
        let bit = (index % 64) as u64;
        self.achievement_flags[word] & (1u64 << bit) != 0
    }
    
    /// Mark achievement as claimed
    pub fn claim_achievement(&mut self, index: u8) {
        let word = (index / 64) as usize;
        let bit = (index % 64) as u64;
        self.achievement_flags[word] |= 1u64 << bit;
    }
}
