use anchor_lang::{prelude::*, system_program};
use crate::state::*;
use crate::error::AcademyError;

#[cfg(test)]
mod tests {
    use super::*;
    
    // Helper untuk setup Config
    fn setup_config() -> Config {
        Config {
            authority: Pubkey::new_from_array([0u8; 32]),
            backend_signer: Pubkey::new_from_array([1u8; 32]),
            current_season: 1,
            current_mint: Pubkey::new_from_array([2u8; 32]),
            season_closed: false,
            season_started_at: 1000000000,
            max_daily_xp: 2000,
            max_achievement_xp: 500,
            _reserved: [0; 32],
            bump: 255,
        }
    }
    
    // Helper untuk setup Course
    fn setup_course() -> Course {
        Course {
            course_id: "anchor-beginner".to_string(),
            creator: Pubkey::new_from_array([3u8; 32]),
            authority: Pubkey::new_from_array([3u8; 32]),
            content_tx_id: [0u8; 32],
            version: 1,
            lesson_count: 10,
            difficulty: 1,
            xp_per_lesson: 30,
            track_id: 1,
            track_level: 1,
            prerequisite: None,
            completion_bonus_xp: 200,
            creator_reward_xp: 50,
            min_completions_for_reward: 10,
            total_completions: 0,
            total_enrollments: 0,
            is_active: true,
            created_at: 1000000000,
            updated_at: 1000000000,
            _reserved: [0; 16],
            bump: 255,
        }
    }
    
    // Helper untuk setup LearnerProfile
    fn setup_learner_profile() -> LearnerProfile {
        LearnerProfile {
            authority: Pubkey::new_from_array([4u8; 32]),
            current_streak: 5,
            longest_streak: 10,
            last_activity_date: 1000000000,
            streak_freezes: 2,
            achievement_flags: [0, 0, 0, 0],
            xp_earned_today: 100,
            last_xp_day: 11574, // 1000000000 / 86400
            referral_count: 0,
            has_referrer: false,
            _reserved: [0; 16],
            bump: 255,
        }
    }
    
    // Helper untuk setup Enrollment
    fn setup_enrollment() -> Enrollment {
        Enrollment {
            course: Pubkey::new_from_array([5u8; 32]),
            enrolled_version: 1,
            enrolled_at: 1000000000,
            completed_at: None,
            lesson_flags: [0, 0, 0, 0],
            credential_asset: None,
            bonus_claimed: false,
            _reserved: [0; 7],
            bump: 255,
        }
    }
    
    // ═══════════════════════════════════════════════════════════════
    // TESTS: Enrollment Progress Bitmap
    // ═══════════════════════════════════════════════════════════════
    
    #[test]
    fn test_complete_lesson() {
        let mut enrollment = setup_enrollment();
        
        // Complete lesson 0
        let new_completion = enrollment.complete_lesson(0);
        assert!(new_completion);
        assert!(enrollment.is_lesson_completed(0));
        assert!(!enrollment.is_lesson_completed(1));
        assert_eq!(enrollment.completed_lessons(), 1);
        
        // Complete lesson 5
        let new_completion = enrollment.complete_lesson(5);
        assert!(new_completion);
        assert_eq!(enrollment.completed_lessons(), 2);
        
        // Try complete same lesson again (should return false)
        let duplicate = enrollment.complete_lesson(0);
        assert!(!duplicate);
        assert_eq!(enrollment.completed_lessons(), 2);
    }
    
    #[test]
    fn test_is_course_completed() {
        let mut enrollment = setup_enrollment();
        let course = setup_course();
        
        // Initially not completed
        assert!(!enrollment.is_course_completed(course.lesson_count));
        
        // Complete all lessons
        for i in 0..course.lesson_count {
            enrollment.complete_lesson(i);
        }
        
        // Now should be completed
        assert!(enrollment.is_course_completed(course.lesson_count));
        assert_eq!(enrollment.completed_lessons(), course.lesson_count);
    }
    
    #[test]
    fn test_lesson_out_of_bounds() {
        let mut enrollment = setup_enrollment();
        
        // Lesson index >= 128 should fail gracefully
        let result = enrollment.complete_lesson(128);
        assert!(!result);
        assert!(!enrollment.is_lesson_completed(128));
    }
    
    // ═══════════════════════════════════════════════════════════════
    // TESTS: Learner Achievement Bitmap
    // ═══════════════════════════════════════════════════════════════
    
    #[test]
    fn test_claim_achievement() {
        let mut learner = setup_learner_profile();
        
        // Claim achievement 0
        assert!(!learner.is_achievement_claimed(0));
        learner.claim_achievement(0);
        assert!(learner.is_achievement_claimed(0));
        
        // Claim achievement 65 (should be in second u64)
        assert!(!learner.is_achievement_claimed(65));
        learner.claim_achievement(65);
        assert!(learner.is_achievement_claimed(65));
        
        // Achievement 0 still claimed
        assert!(learner.is_achievement_claimed(0));
    }
    
    #[test]
    fn test_claim_achievement_across_words() {
        let mut learner = setup_learner_profile();
        
        // Test indices across all 4 words
        let indices = vec![0, 63, 64, 127, 128, 191, 192, 255];
        
        for idx in indices {
            assert!(!learner.is_achievement_claimed(idx));
            learner.claim_achievement(idx);
            assert!(learner.is_achievement_claimed(idx));
        }
        
        // Should have exactly 8 achievements claimed
        let total: u32 = learner.achievement_flags.iter().map(|w| w.count_ones()).sum();
        assert_eq!(total, 8);
    }
    
    #[test]
    fn test_unclaimed_achievement() {
        let learner = setup_learner_profile();
        
        // Index 0-255 should all return false initially
        for i in 0..256u16 {
            assert!(!learner.is_achievement_claimed(i as u8));
        }
    }
    
    // ═══════════════════════════════════════════════════════════════
    // TESTS: Config Validation
    // ═══════════════════════════════════════════════════════════════
    
    #[test]
    fn test_config_size() {
        // Config size should be < 10kb (reasonable)
        assert!(Config::SIZE < 10000);
        assert!(Config::SIZE > 100);
    }
    
    #[test]
    fn test_config_seeds() {
        assert_eq!(Config::SEED, b"config");
    }
    
    // ═══════════════════════════════════════════════════════════════
    // TESTS: Course Validation
    // ═══════════════════════════════════════════════════════════════
    
    #[test]
    fn test_course_seeds() {
        let course_id = "anchor-beginner";
        let seeds = Course::seeds(course_id);
        assert_eq!(seeds.len(), 2);
        assert_eq!(seeds[0], b"course");
        assert_eq!(seeds[1], course_id.as_bytes());
    }
    
    #[test]
    fn test_course_size() {
        // Course size should be reasonable
        assert!(Course::SIZE < 1000);
        assert!(Course::SIZE > 180);
    }
    
    #[test]
    fn test_course_id_constraints() {
        // Empty string
        assert!("".as_bytes().len() <= Course::MAX_COURSE_ID_LEN);
        
        // Max length
        let max_id = "a".repeat(32);
        assert_eq!(max_id.len(), Course::MAX_COURSE_ID_LEN);
        
        // Over max (should be rejected in instruction)
        let over_max = "a".repeat(33);
        assert!(over_max.len() > Course::MAX_COURSE_ID_LEN);
    }
    
    // ═══════════════════════════════════════════════════════════════
    // TESTS: Enrollment Size
    // ═══════════════════════════════════════════════════════════════
    
    #[test]
    fn test_enrollment_size() {
        // Enrollment should be small enough for low cost
        assert!(Enrollment::SIZE < 200);
        assert!(Enrollment::SIZE > 130);
    }
    
    // ═══════════════════════════════════════════════════════════════
    // TESTS: LearnerProfile Size
    // ═══════════════════════════════════════════════════════════════
    
    #[test]
    fn test_learner_profile_size() {
        // Should be small and cheap
        assert!(LearnerProfile::SIZE < 150);
        assert!(LearnerProfile::SIZE > 80);
    }
}
