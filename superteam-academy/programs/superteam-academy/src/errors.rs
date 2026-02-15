use anchor_lang::prelude::*;

#[error_code]
pub enum AcademyError {
    #[msg("Unauthorized signer")]
    Unauthorized,
    
    #[msg("Course not active")]
    CourseNotActive,
    
    #[msg("Already enrolled")]
    AlreadyEnrolled,
    
    #[msg("Not enrolled")]
    NotEnrolled,
    
    #[msg("Lesson index out of bounds")]
    LessonOutOfBounds,
    
    #[msg("Lesson already completed")]
    LessonAlreadyCompleted,
    
    #[msg("Not all lessons completed")]
    CourseNotCompleted,
    
    #[msg("Course already finalized")]
    CourseAlreadyFinalized,
    
    #[msg("Achievement already claimed")]
    AchievementAlreadyClaimed,
    
    #[msg("Course not finalized")]
    CourseNotFinalized,
    
    #[msg("Completion bonus already claimed")]
    BonusAlreadyClaimed,
    
    #[msg("Season already closed")]
    SeasonClosed,
    
    #[msg("Cannot refer yourself")]
    SelfReferral,
    
    #[msg("Already has a referrer")]
    AlreadyReferred,
    
    #[msg("Referrer not found")]
    ReferrerNotFound,
    
    #[msg("Prerequisite not met")]
    PrerequisiteNotMet,
    
    #[msg("Daily XP limit exceeded")]
    DailyXPLimitExceeded,
    
    #[msg("Close cooldown not met (24h)")]
    UnenrollCooldown,
    
    #[msg("Enrollment/course mismatch")]
    EnrollmentCourseMismatch,
    
    #[msg("Season not active")]
    SeasonNotActive,
    
    #[msg("Arithmetic overflow")]
    Overflow,
    
    #[msg("Invalid course ID length")]
    InvalidCourseId,
    
    #[msg("Course ID too long (max 32 chars)")]
    CourseIdTooLong,
    
    #[msg("Season not closed")]
    SeasonNotClosed,
    
    #[msg("Already initialized")]
    AlreadyInitialized,
    
    #[msg("Learner not initialized")]
    LearnerNotInitialized,
    
    #[msg("Not enough streak freezes")]
    NotEnoughFreezes,
    
    #[msg("Achievement index out of bounds")]
    AchievementOutOfBounds,
    
    #[msg("Invalid difficulty level")]
    InvalidDifficulty,
    
    #[msg("Invalid track level")]
    InvalidTrackLevel,
    
    #[msg("Season numbers must be sequential")]
    InvalidSeasonNumber,
    
    #[msg("Course already exists")]
    CourseAlreadyExists,
}
