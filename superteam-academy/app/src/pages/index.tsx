import Head from 'next/head';
import Link from 'next/link';
import { useWallet } from '@solana/wallet-adapter-react';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import { useLearnerProfile, useXpBalance, useCourses } from '../hooks/useProgram';
import { getTrackName, getLevel, getLevelName, getLevelProgress } from '../lib/sdk';

export default function Home() {
  const { connected } = useWallet();
  const { profile, initProfile, loading: profileLoading } = useLearnerProfile();
  const { balance } = useXpBalance();
  const { courses } = useCourses();

  const level = getLevel(balance);
  const levelName = getLevelName(level);
  const progress = getLevelProgress(balance);

  return (
    <>
      <Head>
        <title>Superteam Academy</title>
        <meta name="description" content="Learn Solana, earn credentials" />
      </Head>
      
      <div className="min-h-screen bg-gray-900 text-white">
        {/* Header */}
        <header className="p-6 flex justify-between items-center border-b border-gray-800">
          <Link href="/" className="text-2xl font-bold bg-gradient-to-r from-purple-400 to-pink-400 bg-clip-text text-transparent">
            Superteam Academy
          </Link>
          <div className="flex items-center gap-4">
            {connected && profile && (
              <Link href="/profile" className="flex items-center gap-2 px-4 py-2 bg-gray-800 rounded-lg hover:bg-gray-700">
                <span className="text-sm text-gray-400">Level {level}</span>
                <span className="font-semibold">{balance.toLocaleString()} XP</span>
              </Link>
            )}
            <WalletMultiButton className="!bg-purple-600 hover:!bg-purple-700" />
          </div>
        </header>

        <main className="container mx-auto px-6 py-12">
          {!connected ? (
            // Not Connected State
            <div className="text-center py-20">
              <h2 className="text-5xl font-bold mb-6">
                Master <span className="text-purple-400">Solana</span> Development
              </h2>
              <p className="text-xl text-gray-400 mb-8 max-w-2xl mx-auto">
                Learn blockchain development with hands-on courses. Earn XP, build streaks, 
                and collect verifiable credentials on-chain.
              </p>
              <div className="flex justify-center gap-4">
                <WalletMultiButton className="!px-8 !py-3 !text-lg" />
              </div>
              
              {/* Stats Preview */}
              <div className="grid grid-cols-3 gap-8 mt-16 max-w-3xl mx-auto">
                <StatCard number="4" label="Learning Tracks" />
                <StatCard number="10k+" label="Lessons Completed" />
                <StatCard number="$4.8k" label="Prize Pool" />
              </div>
            </div>
          ) : !profile ? (
            // Connected but no profile
            <div className="text-center py-20">
              <h2 className="text-3xl font-bold mb-4">Welcome to Superteam Academy!</h2>
              <p className="text-gray-400 mb-8">
                Initialize your learner profile to start tracking progress and earning XP.
              </p>
              <button
                onClick={initProfile}
                disabled={profileLoading}
                className="px-8 py-3 bg-purple-600 rounded-lg hover:bg-purple-700 disabled:opacity-50"
              >
                {profileLoading ? 'Initializing...' : 'Initialize Profile'}
              </button>
            </div>
          ) : (
            // Full Dashboard
            <div className="space-y-8">
              {/* Hero Stats */}
              <div className="grid md:grid-cols-4 gap-6">
                <StatCardLarge 
                  title="Current Level"
                  value={levelName}
                  subtitle={`${balance.toLocaleString()} XP`}
                  icon="🎓"
                />
                <StatCardLarge 
                  title="Learning Streak"
                  value={`${profile.currentStreak} days`}
                  subtitle={`Best: ${profile.longestStreak} days`}
                  icon="🔥"
                />
                <StatCardLarge 
                  title="Freezes"
                  value={profile.streakFreezes.toString()}
                  subtitle="Available"
                  icon="❄️"
                />
                <StatCardLarge 
                  title="Achievements"
                  value={profile.achievementFlags.reduce((a, b) => a + (b?.toNumber() || 0), 0).toString()}
                  subtitle="Unlocked"
                  icon="🏆"
                />
              </div>

              {/* Level Progress */}
              <div className="bg-gray-800 rounded-xl p-6">
                <div className="flex justify-between mb-2">
                  <span className="text-gray-400">Level {level} → {level + 1}</span>
                  <span className="text-gray-400">{progress.toFixed(0)}%</span>
                </div>
                <div className="w-full bg-gray-700 rounded-full h-4">
                  <div 
                    className="bg-gradient-to-r from-purple-500 to-pink-500 h-4 rounded-full transition-all"
                    style={{ width: `${progress}%` }}
                  />
                </div>
              </div>

              {/* Course Catalog */}
              <div>
                <h3 className="text-2xl font-bold mb-6">Available Courses</h3>
                <div className="grid md:grid-cols-2 lg:grid-cols-4 gap-6">
                  {courses.map((course) => (
                    <CourseCard key={course.id} {...course} />
                  ))}
                </div>
              </div>
            </div>
          )}
        </main>
      </div>
    </>
  );
}

function StatCard({ number, label }: { number: string; label: string }) {
  return (
    <div className="text-center">
      <div className="text-3xl font-bold text-purple-400">{number}</div>
      <div className="text-gray-400">{label}</div>
    </div>
  );
}

function StatCardLarge({ title, value, subtitle, icon }: { title: string; value: string; subtitle: string; icon: string }) {
  return (
    <div className="bg-gray-800 rounded-xl p-6">
      <div className="text-3xl mb-2">{icon}</div>
      <div className="text-gray-400 text-sm">{title}</div>
      <div className="text-2xl font-bold">{value}</div>
      <div className="text-gray-500 text-sm">{subtitle}</div>
    </div>
  );
}

function CourseCard({ id, name, trackId }: { id: string; name: string; trackId: number }) {
  const colors: Record<number, string> = {
    0: 'bg-gray-500',
    1: 'bg-blue-500',
    2: 'bg-orange-500',
    3: 'bg-green-500',
    4: 'bg-red-500',
  };
  
  const track = getTrackName(trackId);
  
  return (
    <Link href={`/course/${id}`}>
      <div className="bg-gray-800 rounded-xl p-6 border border-gray-700 hover:border-purple-500 transition-all hover:transform hover:scale-105">
        <div className={`w-12 h-12 rounded-lg ${colors[trackId] || 'bg-gray-600'} flex items-center justify-center mb-4`}>
          <span className="text-xl font-bold">{track[0]}</span>
        </div>
        <h3 className="text-lg font-semibold mb-1">{name}</h3>
        <p className="text-gray-400 text-sm mb-4">{track}</p>
        <div className="text-purple-400 text-sm font-medium">Start Learning →</div>
      </div>
    </Link>
  );
}
