use anchor_lang::prelude::*;

/// Enrollment PDA - User's course enrollment
/// Seeds: ["enrollment", course_id.as_bytes(), user_pubkey]
#[account]
pub struct Enrollment {
    /// The Course PDA this enrollment belongs to
    pub course: Pubkey,
    
    /// Course version at time of enrollment
    pub enrolled_version: u16,
    
    /// When learner enrolled
    pub enrolled_at: i64,
    
    /// When course was completed
    pub completed_at: Option<i64>,
    
    /// Lesson completion bitmap (up to 256 lessons)
    pub lesson_flags: [u64; 4],
    
    /// Credential NFT address for this track
    pub credential_asset: Option<Pubkey>,
    
    /// Whether completion bonus claimed
    pub bonus_claimed: bool,
    
    /// Reserved
    pub _reserved: [u8; 7],
    
    /// PDA bump
    pub bump: u8,
}

impl Enrollment {
    pub const SIZE: usize = 8 + // discriminator
        32 + // course
        2 +  // enrolled_version
        8 +  // enrolled_at
        9 +  // completed_at (Option<i64> = 1 + 8)
        32 + // lesson_flags ([u64; 4])
        33 + // credential_asset (Option<Pubkey> = 1 + 32)
        1 +  // bonus_claimed
        7 +  // reserved
        1;   // bump
    
    pub fn course_seeds(course_id: &str) -> Vec<&[u8]> {
        vec![b"course", course_id.as_bytes()]
    }
    
    pub fn learner_seeds(course_id: &str, learner: &Pubkey) -> Vec<Vec<u8>> {
        vec![
            b"enrollment".to_vec(),
            course_id.as_bytes().to_vec(),
            learner.to_bytes().to_vec(),
        ]
    }
    
    /// Check if lesson is completed
    pub fn is_lesson_completed(&self, lesson_index: u8) -> bool {
        if lesson_index >= 128 {
            return false;
        }
        let word = (lesson_index / 64) as usize;
        let bit = (lesson_index % 64) as u64;
        self.lesson_flags[word] & (1u64 << bit) != 0
    }
    
    /// Mark lesson as completed
    pub fn complete_lesson(&mut self, lesson_index: u8) -> bool {
        if lesson_index >= 128 {
            return false;
        }
        let word = (lesson_index / 64) as usize;
        let bit = (lesson_index % 64) as u64;
        let already_completed = self.lesson_flags[word] & (1u64 << bit) != 0;
        if !already_completed {
            self.lesson_flags[word] |= 1u64 << bit;
        }
        !already_completed
    }
    
    /// Check if all lessons completed
    pub fn is_course_completed(&self, lesson_count: u8) -> bool {
        for i in 0..lesson_count {
            if !self.is_lesson_completed(i) {
                return false;
            }
        }
        true
    }
    
    /// Count completed lessons
    pub fn completed_lessons(&self) -> u8 {
        let mut count = 0u8;
        for word in self.lesson_flags.iter() {
            count += word.count_ones() as u8;
        }
        count
    }
}
