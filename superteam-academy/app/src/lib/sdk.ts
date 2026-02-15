import * as anchor from '@coral-xyz/anchor';
import { PublicKey, SystemProgram } from '@solana/web3.js';

export const PROGRAM_ID = new PublicKey('Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS');

// PDA Seeds
export const SEEDS = {
  CONFIG: Buffer.from('config'),
  LEARNER: Buffer.from('learner'),
  ENROLLMENT: Buffer.from('enrollment'),
  COURSE_PREFIX: Buffer.from('course'),
};

// Track registry
export const TRACKS: Record<number, { name: string; display: string }> = {
  0: { name: 'standalone', display: 'Standalone Course' },
  1: { name: 'anchor', display: 'Anchor Framework' },
  2: { name: 'rust', display: 'Rust for Solana' },
  3: { name: 'defi', display: 'DeFi Development' },
  4: { name: 'security', display: 'Program Security' },
};

// Get Config PDA
export function getConfigPDA(): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [SEEDS.CONFIG],
    PROGRAM_ID
  );
}

// Get Course PDA
export function getCoursePDA(courseId: string): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [SEEDS.COURSE_PREFIX, Buffer.from(courseId)],
    PROGRAM_ID
  );
}

// Get LearnerProfile PDA
export function getLearnerProfilePDA(learner: PublicKey): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [SEEDS.LEARNER, learner.toBuffer()],
    PROGRAM_ID
  );
}

// Get Enrollment PDA
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

// Calculate level from XP
export function getLevel(xp: number): number {
  return Math.floor(Math.sqrt(xp / 100));
}

// Check milestone (7, 30, 100, 365)
export function isMilestone(streak: number): boolean {
  return [7, 30, 100, 365].includes(streak);
}

// Format track name
export function getTrackName(trackId: number): string {
  return TRACKS[trackId]?.display || 'Unknown Track';
}

// Types (matching Rust)
export interface Course {
  courseId: string;
  creator: PublicKey;
  authority: PublicKey;
  contentTxId: number[];
  version: number;
  lessonCount: number;
  difficulty: number;
  xpPerLesson: anchor.BN;
  trackId: number;
  trackLevel: number;
  prerequisite: PublicKey | null;
  completionBonusXp: anchor.BN;
  creatorRewardXp: anchor.BN;
  minCompletionsForReward: number;
  totalCompletions: number;
  totalEnrollments: number;
  isActive: boolean;
  createdAt: anchor.BN;
  updatedAt: anchor.BN;
  bump: number;
}

export interface LearnerProfile {
  authority: PublicKey;
  currentStreak: number;
  longestStreak: number;
  lastActivityDate: anchor.BN;
  streakFreezes: number;
  achievementFlags: anchor.BN[];
  xpEarnedToday: number;
  lastXpDay: number;
  referralCount: number;
  hasReferrer: boolean;
  bump: number;
}

export interface Enrollment {
  course: PublicKey;
  enrolledVersion: number;
  enrolledAt: anchor.BN;
  completedAt: anchor.BN | null;
  lessonFlags: anchor.BN[];
  credentialAsset: PublicKey | null;
  bonusClaimed: boolean;
  bump: number;
}
