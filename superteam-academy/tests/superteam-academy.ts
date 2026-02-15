import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { PublicKey, Keypair, SystemProgram } from '@solana/web3.js';
import { assert } from 'chai';
import { SuperteamAcademy } from '../target/types/superteam_academy';

// Test accounts
let provider: anchor.AnchorProvider;
let program: Program<SuperteamAcademy>;

// Test keypairs
let authority: Keypair;
let backendSigner: Keypair;
let learner: Keypair;
let creator: Keypair;

// PDAs
let configPDA: PublicKey;
let configBump: number;
let learnerProfilePDA: PublicKey;
let learnerProfileBump: number;
let coursePDA: PublicKey;
let courseBump: number;
let enrollmentPDA: PublicKey;
let enrollmentBump: number;

const COURSE_ID = "anchor-beginner";

describe('Superteam Academy', () => {
  before(async () => {
    provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    program = anchor.workspace.SuperteamAcademy as Program<SuperteamAcademy>;
    
    // Generate test accounts
    authority = Keypair.generate();
    backendSigner = Keypair.generate();
    learner = Keypair.generate();
    creator = Keypair.generate();
    
    // Airdrop SOL
    await provider.connection.requestAirdrop(authority.publicKey, 10 * anchor.web3.LAMPORTS_PER_SOL);
    await provider.connection.requestAirdrop(learner.publicKey, 10 * anchor.web3.LAMPORTS_PER_SOL);
    await provider.connection.requestAirdrop(creator.publicKey, 10 * anchor.web3.LAMPORTS_PER_SOL);
    
    // Wait for airdrop
    await new Promise(resolve => setTimeout(resolve, 2000));
    
    // Derive PDAs
    [configPDA, configBump] = PublicKey.findProgramAddressSync(
      [Buffer.from('config')],
      program.programId
    );
    
    [learnerProfilePDA, learnerProfileBump] = PublicKey.findProgramAddressSync(
      [Buffer.from('learner'), learner.publicKey.toBuffer()],
      program.programId
    );
    
    [coursePDA, courseBump] = PublicKey.findProgramAddressSync(
      [Buffer.from('course'), Buffer.from(COURSE_ID)],
      program.programId
    );
    
    [enrollmentPDA, enrollmentBump] = PublicKey.findProgramAddressSync(
      [
        Buffer.from('enrollment'),
        Buffer.from(COURSE_ID),
        learner.publicKey.toBuffer(),
      ],
      program.programId
    );
  });
  
  describe('Platform Management', () => {
    it('Initialize platform', async () => {
      const tx = await program.methods
        .initialize({
          maxDailyXp: 2000,
          maxAchievementXp: 500,
        })
        .accounts({
          payer: authority.publicKey,
          authority: authority.publicKey,
          backendSigner: backendSigner.publicKey,
          config: configPDA,
          systemProgram: SystemProgram.programId,
        })
        .signers([authority])
        .rpc();
      
      console.log('Initialize tx:', tx);
      
      // Verify config
      const config = await program.account.config.fetch(configPDA);
      assert.equal(config.authority.toBase58(), authority.publicKey.toBase58());
      assert.equal(config.backendSigner.toBase58(), backendSigner.publicKey.toBase58());
      assert.equal(config.maxDailyXp, 2000);
      assert.equal(config.maxAchievementXp, 500);
      assert.equal(config.currentSeason, 0);
      assert.equal(config.seasonClosed, true);
    });
    
    it('Create season', async () => {
      // Note: In real test, need XP mint account
      // This is a simplified test
      console.log('Season creation requires Token-2022 mint setup');
    });
    
    it('Update config', async () => {
      const newBackendSigner = Keypair.generate().publicKey;
      
      const tx = await program.methods
        .updateConfig({
          backendSigner: newBackendSigner,
          maxDailyXp: 2500,
          maxAchievementXp: null,
        })
        .accounts({
          config: configPDA,
          authority: authority.publicKey,
        })
        .signers([authority])
        .rpc();
      
      console.log('Update config tx:', tx);
      
      const config = await program.account.config.fetch(configPDA);
      assert.equal(config.maxDailyXp, 2500);
    });
  });
  
  describe('Course Management', () => {
    it('Create course', async () => {
      const contentTxId = new Array(32).fill(0);
      
      const tx = await program.methods
        .createCourse(COURSE_ID, {
          creator: creator.publicKey,
          authority: creator.publicKey,
          contentTxId: contentTxId,
          lessonCount: 10,
          difficulty: 1,
          xpPerLesson: 30,
          trackId: 1,
          trackLevel: 1,
          prerequisite: null,
          completionBonusXp: 200,
          creatorRewardXp: 50,
          minCompletionsForReward: 10,
        })
        .accounts({
          payer: authority.publicKey,
          config: configPDA,
          authority: authority.publicKey,
          course: coursePDA,
          prerequisite: null,
          systemProgram: SystemProgram.programId,
        })
        .signers([authority])
        .rpc();
      
      console.log('Create course tx:', tx);
      
      const course = await program.account.course.fetch(coursePDA);
      assert.equal(course.courseId, COURSE_ID);
      assert.equal(course.lessonCount, 10);
      assert.equal(course.difficulty, 1);
      assert.equal(course.xpPerLesson, 30);
      assert.equal(course.isActive, true);
    });
    
    it('Update course', async () => {
      const newContentTxId = new Array(32).fill(1);
      
      const tx = await program.methods
        .updateCourse({
          contentTxId: newContentTxId,
          isActive: null,
          completionBonusXp: null,
          creatorRewardXp: 75,
          minCompletionsForReward: null,
        })
        .accounts({
          course: coursePDA,
          authority: creator.publicKey,
        })
        .signers([creator])
        .rpc();
      
      console.log('Update course tx:', tx);
      
      const course = await program.account.course.fetch(coursePDA);
      assert.equal(course.version, 2);
      assert.equal(course.creatorRewardXp, 75);
    });
  });
  
  describe('Learner Operations', () => {
    it('Initialize learner profile', async () => {
      const tx = await program.methods
        .initLearner()
        .accounts({
          payer: learner.publicKey,
          learner: learner.publicKey,
          profile: learnerProfilePDA,
          systemProgram: SystemProgram.programId,
        })
        .signers([learner])
        .rpc();
      
      console.log('Init learner tx:', tx);
      
      const profile = await program.account.learnerProfile.fetch(learnerProfilePDA);
      assert.equal(profile.authority.toBase58(), learner.publicKey.toBase58());
      assert.equal(profile.currentStreak, 0);
      assert.equal(profile.referralCount, 0);
    });
    
    it('Enroll in course', async () => {
      const tx = await program.methods
        .enroll(COURSE_ID)
        .accounts({
          payer: learner.publicKey,
          learner: learner.publicKey,
          learnerProfile: learnerProfilePDA,
          course: coursePDA,
          enrollment: enrollmentPDA,
          prerequisiteEnrollment: null,
          systemProgram: SystemProgram.programId,
        })
        .signers([learner])
        .rpc();
      
      console.log('Enroll tx:', tx);
      
      const enrollment = await program.account.enrollment.fetch(enrollmentPDA);
      assert.equal(enrollment.enrolledVersion, 2); // Course was updated
      assert.equal(enrollment.lessonFlags[0], 0);
      assert.isNull(enrollment.completedAt);
    });
    
    // Note: complete_lesson, finalize_course, issue_credential require
    // Token-2022 mint and additional accounts - tested in integration tests
  });
  
  describe('Cleanup', () => {
    it('Close enrollment', async () => {
      // Note: This test will fail if course not completed (24h cooldown)
      // For completed course, should work immediately
      console.log('Close enrollment requires course completion or 24h cooldown');
    });
  });
});
