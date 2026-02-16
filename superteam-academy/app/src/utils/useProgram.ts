import { useEffect, useState, useCallback } from 'react';
import { useAnchorWallet, useConnection } from '@solana/wallet-adapter-react';
import { Program, AnchorProvider, web3, BN } from '@coral-xyz/anchor';
import { PublicKey, SystemProgram } from '@solana/web3.js';
import { SuperteamAcademy } from '../types/superteam_academy';
import idl from '../idl/superteam_academy.json';

const PROGRAM_ID = new PublicKey('Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS');

// Helper to get provider
export const useProvider = () => {
  const wallet = useAnchorWallet();
  const { connection } = useConnection();

  if (!wallet) return null;

  return new AnchorProvider(
    connection,
    wallet,
    AnchorProvider.defaultOptions()
  );
};

// Get program instance
export const useProgram = () => {
  const provider = useProvider();
  
  if (!provider) return null;
  
  return new Program<SuperteamAcademy>(
    idl as any,
    PROGRAM_ID,
    provider
  );
};

// ═══════════════════════════════════════════════════════════════
// CONFIG
// ═══════════════════════════════════════════════════════════════

export const useConfig = () => {
  const program = useProgram();
  const [config, setConfig] = useState<any>(null);
  const [loading, setLoading] = useState(false);

  const fetchConfig = useCallback(async () => {
    if (!program) return;
    
    try {
      setLoading(true);
      const [configPDA] = PublicKey.findProgramAddressSync(
        [Buffer.from('config')],
        PROGRAM_ID
      );
      const data = await program.account.config.fetch(configPDA);
      setConfig(data);
    } catch (err) {
      console.error('Error fetching config:', err);
    } finally {
      setLoading(false);
    }
  }, [program]);

  useEffect(() => {
    fetchConfig();
  }, [fetchConfig]);

  return { config, loading, refetch: fetchConfig };
};

// ═══════════════════════════════════════════════════════════════
// LEARNER PROFILE
// ═══════════════════════════════════════════════════════════════

export const useLearnerProfile = () => {
  const program = useProgram();
  const wallet = useAnchorWallet();
  const [profile, setProfile] = useState<any>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchProfile = useCallback(async () => {
    if (!program || !wallet) return;
    
    try {
      setLoading(true);
      const [profilePDA] = PublicKey.findProgramAddressSync(
        [Buffer.from('learner'), wallet.publicKey.toBuffer()],
        PROGRAM_ID
      );
      const data = await program.account.learnerProfile.fetch(profilePDA);
      setProfile(data);
      setError(null);
    } catch (err: any) {
      if (err.message?.includes('Account does not exist')) {
        setProfile(null); // Not initialized yet
      } else {
        setError(err.message);
      }
    } finally {
      setLoading(false);
    }
  }, [program, wallet]);

  const initProfile = useCallback(async () => {
    if (!program || !wallet) return;
    
    try {
      setLoading(true);
      const [profilePDA] = PublicKey.findProgramAddressSync(
        [Buffer.from('learner'), wallet.publicKey.toBuffer()],
        PROGRAM_ID
      );
      
      const tx = await program.methods
        .initLearner()
        .accounts({
          payer: wallet.publicKey,
          learner: wallet.publicKey,
          profile: profilePDA,
          systemProgram: SystemProgram.programId,
        })
        .rpc();
      
      await fetchProfile();
      return tx;
    } catch (err: any) {
      setError(err.message);
      throw err;
    } finally {
      setLoading(false);
    }
  }, [program, wallet, fetchProfile]);

  useEffect(() => {
    fetchProfile();
  }, [fetchProfile]);

  return { profile, loading, error, initProfile, refetch: fetchProfile };
};

// ═══════════════════════════════════════════════════════════════
// COURSES
// ═══════════════════════════════════════════════════════════════

export const useCourses = () => {
  const program = useProgram();
  const [courses, setCourses] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);

  // Note: In real app, fetch from indexer/API
  // For now, we'll use mock data or fetch individual courses
  const fetchCourse = useCallback(async (courseId: string) => {
    if (!program) return null;
    
    try {
      const [coursePDA] = PublicKey.findProgramAddressSync(
        [Buffer.from('course'), Buffer.from(courseId)],
        PROGRAM_ID
      );
      return await program.account.course.fetch(coursePDA);
    } catch (err) {
      console.error('Error fetching course:', err);
      return null;
    }
  }, [program]);

  // Mock course list for demo
  const courseList = [
    { id: 'anchor-beginner', name: 'Anchor Framework', trackId: 1, level: 1 },
    { id: 'rust-beginner', name: 'Rust for Solana', trackId: 2, level: 1 },
    { id: 'defi-beginner', name: 'DeFi Development', trackId: 3, level: 1 },
    { id: 'security-beginner', name: 'Program Security', trackId: 4, level: 1 },
  ];

  return { courses: courseList, fetchCourse, loading };
};

// ═══════════════════════════════════════════════════════════════
// ENROLLMENT
// ═══════════════════════════════════════════════════════════════

export const useEnrollment = (courseId: string) => {
  const program = useProgram();
  const wallet = useAnchorWallet();
  const [enrollment, setEnrollment] = useState<any>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const getPDAs = useCallback(() => {
    if (!wallet) return null;
    
    const [enrollmentPDA] = PublicKey.findProgramAddressSync(
      [
        Buffer.from('enrollment'),
        Buffer.from(courseId),
        wallet.publicKey.toBuffer(),
      ],
      PROGRAM_ID
    );
    
    const [coursePDA] = PublicKey.findProgramAddressSync(
      [Buffer.from('course'), Buffer.from(courseId)],
      PROGRAM_ID
    );
    
    const [profilePDA] = PublicKey.findProgramAddressSync(
      [Buffer.from('learner'), wallet.publicKey.toBuffer()],
      PROGRAM_ID
    );
    
    return { enrollmentPDA, coursePDA, profilePDA };
  }, [wallet, courseId]);

  const fetchEnrollment = useCallback(async () => {
    if (!program || !wallet) return;
    
    const pdas = getPDAs();
    if (!pdas) return;
    
    try {
      setLoading(true);
      const data = await program.account.enrollment.fetch(pdas.enrollmentPDA);
      setEnrollment(data);
      setError(null);
    } catch (err: any) {
      if (err.message?.includes('Account does not exist')) {
        setEnrollment(null);
      } else {
        setError(err.message);
      }
    } finally {
      setLoading(false);
    }
  }, [program, wallet, getPDAs]);

  const enroll = useCallback(async () => {
    if (!program || !wallet) return;
    
    const pdas = getPDAs();
    if (!pdas) return;
    
    try {
      setLoading(true);
      
      const tx = await program.methods
        .enroll(courseId)
        .accounts({
          payer: wallet.publicKey,
          learner: wallet.publicKey,
          learnerProfile: pdas.profilePDA,
          course: pdas.coursePDA,
          enrollment: pdas.enrollmentPDA,
          prerequisiteEnrollment: null,
          systemProgram: SystemProgram.programId,
        })
        .rpc();
      
      await fetchEnrollment();
      return tx;
    } catch (err: any) {
      setError(err.message);
      throw err;
    } finally {
      setLoading(false);
    }
  }, [program, wallet, courseId, getPDAs, fetchEnrollment]);

  const closeEnrollment = useCallback(async () => {
    if (!program || !wallet) return;
    
    const pdas = getPDAs();
    if (!pdas) return;
    
    try {
      setLoading(true);
      
      const tx = await program.methods
        .closeEnrollment()
        .accounts({
          learner: wallet.publicKey,
          course: pdas.coursePDA,
          enrollment: pdas.enrollmentPDA,
        })
        .rpc();
      
      setEnrollment(null);
      return tx;
    } catch (err: any) {
      setError(err.message);
      throw err;
    } finally {
      setLoading(false);
    }
  }, [program, wallet, getPDAs]);

  useEffect(() => {
    fetchEnrollment();
  }, [fetchEnrollment]);

  return {
    enrollment,
    loading,
    error,
    enroll,
    closeEnrollment,
    refetch: fetchEnrollment,
  };
};

// ═══════════════════════════════════════════════════════════════
// LESSON PROGRESS
// ═══════════════════════════════════════════════════════════════

export const useLessonProgress = (courseId: string) => {
  const program = useProgram();
  const wallet = useAnchorWallet();
  const { config } = useConfig();
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const completeLesson = useCallback(async (lessonIndex: number) => {
    if (!program || !wallet || !config) return;
    
    const [enrollmentPDA] = PublicKey.findProgramAddressSync(
      [
        Buffer.from('enrollment'),
        Buffer.from(courseId),
        wallet.publicKey.toBuffer(),
      ],
      PROGRAM_ID
    );
    
    const [coursePDA] = PublicKey.findProgramAddressSync(
      [Buffer.from('course'), Buffer.from(courseId)],
      PROGRAM_ID
    );
    
    const [profilePDA] = PublicKey.findProgramAddressSync(
      [Buffer.from('learner'), wallet.publicKey.toBuffer()],
      PROGRAM_ID
    );
    
    const [configPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from('config')],
      PROGRAM_ID
    );
    
    try {
      setLoading(true);
      
      // Note: This requires backend signer - in real app, call backend API
      // For demo purposes, showing the structure
      console.log('Complete lesson:', lessonIndex);
      
      // Mock completion for UI demo
      return { signature: 'mock-tx-signature' };
    } catch (err: any) {
      setError(err.message);
      throw err;
    } finally {
      setLoading(false);
    }
  }, [program, wallet, config, courseId]);

  return { completeLesson, loading, error };
};

// ═══════════════════════════════════════════════════════════════
// XP BALANCE (from Token Account)
// ═══════════════════════════════════════════════════════════════

export const useXpBalance = () => {
  const { connection } = useConnection();
  const wallet = useAnchorWallet();
  const { config } = useConfig();
  const [balance, setBalance] = useState<number>(0);
  const [loading, setLoading] = useState(false);

  const fetchBalance = useCallback(async () => {
    if (!wallet || !config?.currentMint) return;
    
    try {
      setLoading(true);
      
      // Get token accounts by owner
      const tokenAccounts = await connection.getParsedTokenAccountsByOwner(
        wallet.publicKey,
        { mint: config.currentMint }
      );
      
      if (tokenAccounts.value.length > 0) {
        const amount = tokenAccounts.value[0].account.data.parsed.info.tokenAmount.uiAmount;
        setBalance(amount || 0);
      } else {
        setBalance(0);
      }
    } catch (err) {
      console.error('Error fetching XP balance:', err);
      setBalance(0);
    } finally {
      setLoading(false);
    }
  }, [connection, wallet, config]);

  useEffect(() => {
    fetchBalance();
  }, [fetchBalance]);

  return { balance, loading, refetch: fetchBalance };
};
