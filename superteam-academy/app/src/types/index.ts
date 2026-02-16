export interface Course {
  id: string;
  name: string;
  description: string;
  trackId: number;
  difficulty: number;
  lessonCount: number;
  xpPerLesson: number;
  completionBonusXp: number;
}

export interface Lesson {
  index: number;
  title: string;
  content: string;
  codeExample?: string;
  xpReward: number;
}

export interface EnrollmentData {
  course: string;
  enrolledVersion: number;
  enrolledAt: number;
  completedAt: number | null;
  lessonFlags: number[];
  credentialAsset: string | null;
  bonusClaimed: boolean;
}

export interface ProfileData {
  authority: string;
  currentStreak: number;
  longestStreak: number;
  lastActivityDate: number;
  streakFreezes: number;
  achievementFlags: number[];
  xpEarnedToday: number;
  lastXpDay: number;
  referralCount: number;
  hasReferrer: boolean;
}

// IDL Types
export type SuperteamAcademy = any;
