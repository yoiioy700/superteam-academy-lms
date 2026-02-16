import * as anchor from '@coral-xyz/anchor';
import { PublicKey, SystemProgram, TransactionInstruction } from '@solana/web3.js';
// import { SuperteamAcademy } from './idl/superteam_academy';

export const PROGRAM_ID = new PublicKey('Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS');

// ═══════════════════════════════════════════════════════════════
// SEEDS & PDAS
// ═══════════════════════════════════════════════════════════════

export const SEEDS = {
  CONFIG: Buffer.from('config'),
  LEARNER: Buffer.from('learner'),
  ENROLLMENT: Buffer.from('enrollment'),
  COURSE_PREFIX: Buffer.from('course'),
};

export function getConfigPDA(): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [SEEDS.CONFIG],
    PROGRAM_ID
  );
}

export function getCoursePDA(courseId: string): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [SEEDS.COURSE_PREFIX, Buffer.from(courseId)],
    PROGRAM_ID
  );
}

export function getLearnerProfilePDA(learner: PublicKey): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [SEEDS.LEARNER, learner.toBuffer()],
    PROGRAM_ID
  );
}

export function getEnrollmentPDA(
  courseId: string,
  learner: PublicKey
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [
      SEEDS.ENROLLMENT,
      Buffer.from(courseId),
      learner.toBuffer(),
    ],
    PROGRAM_ID
  );
}

// ═══════════════════════════════════════════════════════════════
// TRACKS
// ═══════════════════════════════════════════════════════════════

export interface TrackInfo {
  name: string;
  display: string;
}

export const TRACKS: Record<number, TrackInfo> = {
  0: { name: 'standalone', display: 'Standalone Course' },
  1: { name: 'anchor', display: 'Anchor Framework' },
  2: { name: 'rust', display: 'Rust for Solana' },
  3: { name: 'defi', display: 'DeFi Development' },
  4: { name: 'security', display: 'Program Security' },
};

// ═══════════════════════════════════════════════════════════════
// LEVELS
// ═══════════════════════════════════════════════════════════════

export function getLevel(xp: number): number {
  return Math.floor(Math.sqrt(xp / 100));
}

export function getLevelName(level: number): string {
  if (level === 0) return 'Newcomer';
  if (level <= 5) return 'Beginner';
  if (level <= 15) return 'Intermediate';
  if (level <= 30) return 'Advanced';
  return 'Expert';
}

export function getNextLevelXp(level: number): number {
  return (level + 1) ** 2 * 100;
}

export function getLevelProgress(xp: number): number {
  const level = getLevel(xp);
  const levelStart = level ** 2 * 100;
  const levelEnd = (level + 1) ** 2 * 100;
  return ((xp - levelStart) / (levelEnd - levelStart)) * 100;
}

// ═══════════════════════════════════════════════════════════════
// TRACKS
// ═══════════════════════════════════════════════════════════════

export function getTrackName(trackId: number): string {
  return TRACKS[trackId]?.display || `Track ${trackId}`;
}

export function getTrackUrlName(trackId: number): string {
  return TRACKS[trackId]?.name || 'unknown';
}

// ═══════════════════════════════════════════════════════════════
// STREAK & MILESTONES
// ═══════════════════════════════════════════════════════════════

export const STREAK_MILESTONES = [7, 30, 100, 365];

export function isStreakMilestone(streak: number): boolean {
  return STREAK_MILESTONES.includes(streak);
}

export function getNextStreakMilestone(streak: number): number | null {
  for (const milestone of STREAK_MILESTONES) {
    if (milestone > streak) return milestone;
  }
  return null;
}

// ═══════════════════════════════════════════════════════════════
// ACHIEVEMENTS
// ═══════════════════════════════════════════════════════════════

export interface Achievement {
  index: number;
  name: string;
  description: string;
  xpReward: number;
  icon: string;
}

export const ACHIEVEMENTS: Achievement[] = [
  { index: 0, name: 'First Steps', description: 'Complete your first lesson', xpReward: 50, icon: '🚀' },
  { index: 1, name: 'Course Completor', description: 'Complete your first course', xpReward: 100, icon: '🎓' },
  { index: 2, name: '7-Day Streak', description: 'Maintain a 7-day learning streak', xpReward: 150, icon: '🔥' },
  { index: 3, name: '30-Day Streak', description: 'Maintain a 30-day learning streak', xpReward: 300, icon: '📅' },
  { index: 4, name: 'Referrer', description: 'Refer 3 friends', xpReward: 200, icon: '👥' },
];

// ═══════════════════════════════════════════════════════════════
// ARWEAVE HELPERS
// ═══════════════════════════════════════════════════════════════

export const ARWEAVE_BASE_URL = 'https://arweave.net/';

export function getArweaveUrl(txId: string): string {
  return `${ARWEAVE_BASE_URL}${txId}`;
}

// ═══════════════════════════════════════════════════════════════
// DIFFICULTY
// ═══════════════════════════════════════════════════════════════

export function getDifficultyName(difficulty: number): string {
  switch (difficulty) {
    case 1: return 'Beginner';
    case 2: return 'Intermediate';
    case 3: return 'Advanced';
    default: return 'Unknown';
  }
}

export function getDifficultyColor(difficulty: number): string {
  switch (difficulty) {
    case 1: return 'bg-green-500';
    case 2: return 'bg-yellow-500';
    case 3: return 'bg-red-500';
    default: return 'bg-gray-500';
  }
}

// ═══════════════════════════════════════════════════════════════
// FORMATTING
// ═══════════════════════════════════════════════════════════════

export function formatXp(xp: number): string {
  if (xp >= 1000000) {
    return `${(xp / 1000000).toFixed(1)}M`;
  }
  if (xp >= 1000) {
    return `${(xp / 1000).toFixed(1)}K`;
  }
  return xp.toString();
}

export function formatDate(timestamp: number): string {
  return new Date(timestamp * 1000).toLocaleDateString();
}

export function formatDuration(seconds: number): string {
  const days = Math.floor(seconds / 86400);
  const hours = Math.floor((seconds % 86400) / 3600);
  
  if (days > 0) return `${days}d ${hours}h`;
  return `${hours}h`;
}

// ═══════════════════════════════════════════════════════════════
// LESSON UTILS
// ═══════════════════════════════════════════════════════════════

export function isLessonCompleted(
  lessonFlags: anchor.BN[],
  lessonIndex: number
): boolean {
  if (lessonIndex >= 128) return false;
  const word = Math.floor(lessonIndex / 64);
  const bit = lessonIndex % 64;
  const wordValue = lessonFlags[word]?.toNumber() || 0;
  return (wordValue & (1 << bit)) !== 0;
}

export function countCompletedLessons(lessonFlags: anchor.BN[]): number {
  let count = 0;
  for (const word of lessonFlags) {
    const num = word?.toNumber() || 0;
    count += (num << 0 >>> 0).toString(2).replace(/0/g, '').length;
  }
  return count;
}

export function getProgressPercent(
  lessonFlags: anchor.BN[],
  totalLessons: number
): number {
  const completed = countCompletedLessons(lessonFlags);
  return Math.round((completed / totalLessons) * 100);
}
