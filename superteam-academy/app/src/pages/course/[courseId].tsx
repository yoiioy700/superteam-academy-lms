import { useRouter } from 'next/router';
import Head from 'next/head';
import Link from 'next/link';
import { useWallet } from '@solana/wallet-adapter-react';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import { useEnrollment, useCourses, useLearnerProfile, useLessonProgress } from '../hooks';
import { getTrackName, getDifficultyName, getProgressPercent, isLessonCompleted } from '../lib';
import { useState } from 'react';

export default function CourseDetail() {
  const router = useRouter();
  const { courseId } = router.query;
  const { connected } = useWallet();
  const { profile } = useLearnerProfile();
  const { fetchCourse } = useCourses();
  const { enrollment, enroll, loading: enrollLoading } = useEnrollment(courseId as string || '');
  const { completeLesson } = useLessonProgress(courseId as string || '');
  const [selectedLesson, setSelectedLesson] = useState(0);

  // Mock course data (in real app, fetch from chain or API)
  const course = {
    id: courseId as string,
    name: 'Anchor Framework',
    description: 'Learn to build Solana programs with Anchor framework. From basics to advanced patterns.',
    trackId: 1,
    difficulty: 1,
    lessonCount: 10,
    xpPerLesson: 30,
    completionBonusXp: 200,
    lessons: [
      { title: 'Introduction to Anchor', content: 'Anchor is a framework for Solana...' },
      { title: 'Setting Up Environment', content: 'Install Anchor CLI and dependencies...' },
      { title: 'Your First Program', content: 'Create a basic Solana program...' },
      { title: 'Accounts and Data', content: 'Understanding Solana account model...' },
      { title: 'Instructions', content: 'Defining program instructions...' },
      { title: 'Errors and Validation', content: 'Handling errors properly...' },
      { title: 'Testing with Mocha', content: 'Write tests for your program...' },
      { title: 'Deployment', content: 'Deploy to devnet and mainnet...' },
      { title: 'Frontend Integration', content: 'Connect your React app...' },
      { title: 'Best Practices', content: 'Security and optimization tips...' },
    ],
  };

  const isEnrolled = !!enrollment;
  const completedLessons = enrollment?.lessonFlags 
    ? enrollment.lessonFlags.reduce((acc: number, flag: any) => acc + (flag?.toNumber() || 0).toString(2).replace(/0/g, '').length, 0)
    : 0;
  const progress = getProgressPercent(
    enrollment?.lessonFlags || [{ toNumber: () => 0 }, { toNumber: () => 0 }, { toNumber: () => 0 }, { toNumber: () => 0 }],
    course.lessonCount
  );

  const handleEnroll = async () => {
    if (!connected || !profile) {
      alert('Please connect wallet and initialize profile first');
      return;
    }
    try {
      await enroll();
    } catch (err: any) {
      alert(err.message);
    }
  };

  const handleCompleteLesson = async (index: number) => {
    try {
      await completeLesson(index);
      alert('Lesson marked as complete! (Backend signer required for real XP)');
    } catch (err: any) {
      alert(err.message);
    }
  };

  return (
    <>
      <Head>
        <title>{course.name} | Superteam Academy</title>
      </Head>
      
      <div className="min-h-screen bg-gray-900 text-white">
        {/* Header */}
        <header className="p-6 flex justify-between items-center border-b border-gray-800">
          <Link href="/" className="text-2xl font-bold bg-gradient-to-r from-purple-400 to-pink-400 bg-clip-text text-transparent">
            Superteam Academy
          </Link>
          <WalletMultiButton className="!bg-purple-600 hover:!bg-purple-700" />
        </header>

        <main className="container mx-auto px-6 py-8">
          {!connected ? (
            <div className="text-center py-20">
              <h2 className="text-2xl font-bold mb-4">Connect Wallet</h2>
              <p className="text-gray-400 mb-6">Please connect your wallet to view this course</p>
              <WalletMultiButton />
            </div>
          ) : (
            <div className="grid lg:grid-cols-3 gap-8">
              {/* Main Content */}
              <div className="lg:col-span-2 space-y-6">
                {/* Course Header */}
                <div className="bg-gray-800 rounded-xl p-6">
                  <div className="flex items-center gap-4 mb-4">
                    <span className="px-3 py-1 bg-blue-500 rounded-full text-sm font-medium">
                      {getTrackName(course.trackId)}
                    </span>
                    <span className="px-3 py-1 bg-gray-700 rounded-full text-sm">
                      {getDifficultyName(course.difficulty)}
                    </span>
                    <span className="text-gray-400 text-sm">
                      {course.lessonCount} lessons
                    </span>
                  </div>
                  <h1 className="text-3xl font-bold mb-2">{course.name}</h1>
                  <p className="text-gray-400 mb-4">{course.description}</p>
                  
                  {!isEnrolled ? (
                    <button
                      onClick={handleEnroll}
                      disabled={enrollLoading}
                      className="px-8 py-3 bg-purple-600 rounded-lg hover:bg-purple-700 disabled:opacity-50 font-semibold"
                    >
                      {enrollLoading ? 'Enrolling...' : 'Enroll Now'}
                    </button>
                  ) : (
                    <div className="flex items-center gap-4">
                      <span className="px-4 py-2 bg-green-600 rounded-lg">
                        ✓ Enrolled
                      </span>
                      <span className="text-gray-400">
                        {completedLessons}/{course.lessonCount} completed
                      </span>
                    </div>
                  )}
                </div>

                {/* Progress Bar */}
                {isEnrolled && (
                  <div className="bg-gray-800 rounded-xl p-4">
                    <div className="flex justify-between text-sm mb-2">
                      <span>Your Progress</span>
                      <span>{progress}%</span>
                    </div>
                    <div className="w-full bg-gray-700 rounded-full h-3">
                      <div 
                        className="bg-gradient-to-r from-purple-500 to-pink-500 h-3 rounded-full"
                        style={{ width: `${progress}%` }}
                      />
                    </div>
                  </div>
                )}

                {/* Lesson Viewer */}
                {isEnrolled && (
                  <div className="bg-gray-800 rounded-xl p-6">
                    <div className="flex justify-between items-center mb-6">
                      <h2 className="text-xl font-bold">
                        Lesson {selectedLesson + 1}: {course.lessons[selectedLesson].title}
                      </h2>
                      <span className="text-purple-400 font-medium">
                        +{course.xpPerLesson} XP
                      </span>
                    </div>
                    
                    <div className="prose prose-invert max-w-none mb-6">
                      <p className="text-gray-300">{course.lessons[selectedLesson].content}</p>
                    </div>

                    {/* Code Editor Placeholder */}
                    <div className="bg-gray-900 rounded-lg p-4 mb-6 border border-gray-700">
                      <div className="flex justify-between items-center mb-4">
                        <span className="text-gray-400 text-sm">Code Editor</span>
                        <span className="text-xs text-gray-500">Rust</span>
                      </div>
                      <pre className="text-sm text-gray-300 font-mono">
{`// Your code here
use anchor_lang::prelude::*;

#[program]
pub mod my_program {
    use super::*;
    
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}`}
                      </pre>
                    </div>

                    <button
                      onClick={() => handleCompleteLesson(selectedLesson)}
                      disabled={isLessonCompleted(enrollment?.lessonFlags || [], selectedLesson)}
                      className="w-full py-3 bg-green-600 rounded-lg hover:bg-green-700 disabled:opacity-50 disabled:cursor-not-allowed font-semibold"
                    >
                      {isLessonCompleted(enrollment?.lessonFlags || [], selectedLesson) 
                        ? '✓ Completed' 
                        : 'Mark as Complete'}
                    </button>
                  </div>
                )}
              </div>

              {/* Sidebar */}
              <div className="space-y-6">
                {/* Lesson List */}
                <div className="bg-gray-800 rounded-xl p-4">
                  <h3 className="font-bold mb-4">Course Content</h3>
                  <div className="space-y-2">
                    {course.lessons.map((lesson, index) => {
                      const isComplete = isLessonCompleted(enrollment?.lessonFlags || [], index);
                      const isLocked = !isEnrolled || (index > 0 && !isLessonCompleted(enrollment?.lessonFlags || [], index - 1));
                      
                      return (
                        <button
                          key={index}
                          onClick={() => isEnrolled && setSelectedLesson(index)}
                          disabled={!isEnrolled}
                          className={`w-full text-left p-3 rounded-lg flex items-center gap-3 ${
                            selectedLesson === index && isEnrolled
                              ? 'bg-purple-600'
                              : isComplete
                              ? 'bg-green-900/30'
                              : 'bg-gray-700 hover:bg-gray-600'
                          } ${!isEnrolled && 'opacity-50 cursor-not-allowed'}`}
                        >
                          <span className={`w-6 h-6 rounded-full flex items-center justify-center text-sm ${
                            isComplete ? 'bg-green-500' : 'bg-gray-600'
                          }`}>
                            {isComplete ? '✓' : index + 1}
                          </span>
                          <span className="text-sm truncate">{lesson.title}</span>
                        </button>
                      );
                    })}
                  </div>
                </div>

                {/* Rewards */}
                <div className="bg-gray-800 rounded-xl p-4">
                  <h3 className="font-bold mb-4">Rewards</h3>
                  <div className="space-y-3 text-sm">
                    <div className="flex justify-between">
                      <span className="text-gray-400">Per Lesson</span>
                      <span>{course.xpPerLesson} XP</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-400">Completion Bonus</span>
                      <span className="text-purple-400">{course.completionBonusXp} XP</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-400">Credential</span>
                      <span>NFT Badge</span>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          )}
        </main>
      </div>
    </>
  );
}
