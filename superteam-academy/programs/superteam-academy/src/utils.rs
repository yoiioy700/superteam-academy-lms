use anchor_lang::prelude::*;
use anchor_spl::token_2022::spl_token_2022::extension::{
    ExtensionType, 
    metadata_pointer::MetadataPointer,
    group_pointer::GroupPointer,
};
use anchor_spl::token_interface::{Token2022, Mint, TokenAccount};
use anchor_spl::token_2022::spl_token_2022::instruction::AuthorityType;

use crate::{state::*, error::AcademyError};

/// Check and update daily XP for rate limiting
pub fn check_and_update_daily_xp(
    learner: &mut LearnerProfile,
    config: &Config,
    xp_amount: u32,
) -> Result<()> {
    let today = (Clock::get()?.unix_timestamp / 86400) as u16;
    
    if today > learner.last_xp_day {
        // New day: reset counter
        learner.xp_earned_today = 0;
        learner.last_xp_day = today;
    }
    
    let new_total = learner
        .xp_earned_today
        .checked_add(xp_amount)
        .ok_or(AcademyError::Overflow)?;
    
    require!(
        new_total <= config.max_daily_xp,
        AcademyError::DailyXPLimitExceeded
    );
    
    learner.xp_earned_today = new_total;
    Ok(())
}

/// Update streak based on activity
pub fn update_streak(
    learner: &mut LearnerProfile,
) -> Result<Option<StreakUpdate>> {
    let now = Clock::get()?.unix_timestamp;
    let today = (now / 86400) as u64;
    let last_day = (learner.last_activity_date / 86400) as u64;
    
    if today <= last_day {
        // Same day, no streak update
        return Ok(None);
    }
    
    let gap = today
        .checked_sub(last_day)
        .ok_or(AcademyError::Overflow)?
        .checked_sub(1)
        .unwrap_or(0);
    
    let streak_update = if gap == 0 {
        // Consecutive day
        learner.current_streak = learner
            .current_streak
            .checked_add(1)
            .ok_or(AcademyError::Overflow)?;
        StreakUpdate::Continued { new_streak: learner.current_streak }
    } else if gap <= learner.streak_freezes as u64 {
        // Covered by freezes
        let freezes_used = gap as u8;
        learner.streak_freezes = learner
            .streak_freezes
            .checked_sub(freezes_used)
            .ok_or(AcademyError::Overflow)?;
        learner.current_streak = learner
            .current_streak
            .checked_add(1)
            .ok_or(AcademyError::Overflow)?;
        StreakUpdate::SavedByFreezes { 
            freezes_used,
            new_streak: learner.current_streak,
        }
    } else {
        // Streak broken
        let old_streak = learner.current_streak;
        learner.current_streak = 1;
        StreakUpdate::Broken { old_streak, days_missed: gap as u16 }
    };
    
    // Update longest streak
    if learner.current_streak > learner.longest_streak {
        learner.longest_streak = learner.current_streak;
    }
    
    learner.last_activity_date = now;
    
    Ok(Some(streak_update))
}

/// Streak update result
#[derive(Debug)]
pub enum StreakUpdate {
    Continued { new_streak: u16 },
    SavedByFreezes { freezes_used: u8, new_streak: u16 },
    Broken { old_streak: u16, days_missed: u16 },
}

/// Check milestone (7, 30, 100, 365)
pub fn check_milestone(streak: u16) -> Option<u16> {
    match streak {
        7 | 30 | 100 | 365 => Some(streak),
        _ => None,
    }
}

/// Get current day number (for XP tracking)
pub fn current_day() -> Result<u64> {
    Ok((Clock::get()?.unix_timestamp / 86400) as u64)
}
