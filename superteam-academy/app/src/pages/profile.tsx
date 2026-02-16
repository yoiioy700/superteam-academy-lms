import Head from 'next/head';
import Link from 'next/link';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import { useLearnerProfile, useXpBalance } from '../hooks/useProgram';
import { 
  getLevel, 
  getLevelName, 
  getLevelProgress, 
  getNextLevelXp,
  formatXp,
  STREAK_MILESTONES,
  isStreakMilestone,
  ACHIEVEMENTS,
  getTrackName,
} from '../lib/sdk';

export default function Profile() {
  const { profile, loading: profileLoading } = useLearnerProfile();
  const { balance, loading: balanceLoading } = useXpBalance();

  const level = getLevel(balance);
  const levelName = getLevelName(level);
  const progress = getLevelProgress(balance);
  const nextLevelXp = getNextLevelXp(level);
  const xpToNext = nextLevelXp - balance;

  if (profileLoading || balanceLoading) {
    return (
      <div className="min-h-screen bg-gray-900 text-white flex items-center justify-center">
        <div className="text-xl">Loading...</div>
      </div>
    );
  }

  if (!profile) {
    return (
      <div className="min-h-screen bg-gray-900 text-white">
        <header className="p-6 flex justify-between items-center border-b border-gray-800">
          <Link href="/" className="text-2xl font-bold bg-gradient-to-r from-purple-400 to-pink-400 bg-clip-text text-transparent">
            Superteam Academy
          </Link>
          <WalletMultiButton className="!bg-purple-600 hover:!bg-purple-700" />
        </header>
        <div className="container mx-auto px-6 py-20 text-center">
          <h2 className="text-2xl font-bold mb-4">Profile Not Found</h2>
          <p className="text-gray-400 mb-6">Please initialize your profile from the home page</p>
          <Link href="/" className="px-6 py-3 bg-purple-600 rounded-lg hover:bg-purple-700">
            Go Home
          </Link>
        </div>
      </div>
    );
  }

  const nextMilestone = STREAK_MILESTONES.find(m => m > profile.currentStreak);

  return (
    <>
      <Head>
        <title>Profile | Superteam Academy</title>
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
          {/* Profile Header */}
          <div className="bg-gradient-to-r from-purple-600 to-pink-600 rounded-2xl p-8 mb-8">
            <div className="flex items-center gap-6">
              <div className="w-24 h-24 bg-white/20 rounded-full flex items-center justify-center text-4xl">
                🎓
              </div>
              <div>
                <h1 className="text-3xl font-bold mb-2">{levelName}</h1>
                <p className="text-white/80">Level {level} • {formatXp(balance)} XP</p>
                <p className="text-white/60 text-sm mt-1">
                  {xpToNext > 0 ? `${formatXp(xpToNext)} XP to next level` : 'Max Level!'}
                </p>
              </div>
            </div>
          </div>

          <div className="grid lg:grid-cols-3 gap-8">
            {/* Main Stats */}
            <div className="lg:col-span-2 space-y-6">
              {/* Level Progress */}
              <div className="bg-gray-800 rounded-xl p-6">
                <h3 className="font-bold mb-4">Level Progress</h3>
                <div className="flex justify-between mb-2">
                  <span className="text-gray-400">Level {level} → {level + 1}</span>
                  <span className="text-gray-400">{progress.toFixed(1)}%</span>
                </div>
                <div className="w-full bg-gray-700 rounded-full h-4 mb-4">
                  <div 
                    className="bg-gradient-to-r from-purple-500 to-pink-500 h-4 rounded-full transition-all"
                    style={{ width: `${progress}%` }}
                  />
                </div>
                <div className="flex justify-between text-sm text-gray-400">
                  <span>{formatXp(level ** 2 * 100)} XP</span>
                  <span>{formatXp((level + 1) ** 2 * 100)} XP</span>
                </div>
              </div>

              {/* Streak Section */}
              <div className="bg-gray-800 rounded-xl p-6">
                <div className="flex items-center justify-between mb-4">
                  <h3 className="font-bold">Learning Streak</h3>
                  <div className="flex items-center gap-2">
                    <span className="text-2xl">🔥</span>
                    <span className="text-2xl font-bold">{profile.currentStreak}</span>
                    <span className="text-gray-400">days</span>
                  </div>
                </div>
                
                <div className="grid grid-cols-2 gap-4 mb-4">
                  <div className="bg-gray-700 rounded-lg p-4">
                    <div className="text-gray-400 text-sm">Current Streak</div>
                    <div className="text-2xl font-bold">{profile.currentStreak} days</div>
                  </div>
                  <div className="bg-gray-700 rounded-lg p-4">
                    <div className="text-gray-400 text-sm">Best Streak</div>
                    <div className="text-2xl font-bold">{profile.longestStreak} days</div>
                  </div>
                </div>

                {nextMilestone && (
                  <div className="bg-orange-900/30 border border-orange-500/30 rounded-lg p-4">
                    <div className="flex items-center gap-2 text-orange-400">
                      <span>🏆</span>
                      <span>Next milestone: {nextMilestone} days</span>
                    </div>
                  </div>
                )}

                {profile.streakFreezes > 0 && (
                  <div className="mt-4 flex items-center gap-2 text-blue-400">
                    <span>❄️</span>
                    <span>{profile.streakFreezes} streak freeze{profile.streakFreezes > 1 ? 's' : ''} available</span>
                  </div>
                )}
              </div>

              {/* Recent Activity */}
              <div className="bg-gray-800 rounded-xl p-6">
                <h3 className="font-bold mb-4">Recent Activity</h3>
                <div className="space-y-3">
                  <ActivityItem 
                    icon="📚"
                    text="Completed 'Introduction to Anchor'"
                    xp={30}
                    time="2 hours ago"
                  />
                  <ActivityItem 
                    icon="🎯"
                    text="Enrolled in 'Anchor Framework'"
                    xp={0}
                    time="1 day ago"
                  />
                  <ActivityItem 
                    icon="🚀"
                    text="Profile initialized"
                    xp={0}
                    time="1 day ago"
                  />
                </div>
              </div>
            </div>

            {/* Sidebar */}
            <div className="space-y-6">
              {/* Stats Grid */}
              <div className="bg-gray-800 rounded-xl p-6">
                <h3 className="font-bold mb-4">Statistics</h3>
                <div className="grid grid-cols-2 gap-4">
                  <StatBox label="Total XP" value={formatXp(balance)} />
                  <StatBox label="Current Level" value={level.toString()} />
                  <StatBox label="Referrals" value={profile.referralCount.toString()} />
                  <StatBox label="Has Referrer" value={profile.hasReferrer ? 'Yes' : 'No'} />
                </div>
              </div>

              {/* Achievements */}
              <div className="bg-gray-800 rounded-xl p-6">
                <h3 className="font-bold mb-4">Achievements</h3>
                <div className="space-y-3">
                  {ACHIEVEMENTS.slice(0, 5).map((ach) => {
                    const isUnlocked = profile.achievementFlags.some(
                      (flag: any) => (flag?.toNumber() || 0) & (1 << ach.index)
                    );
                    
                    return (
                      <div 
                        key={ach.index}
                        className={`flex items-center gap-3 p-3 rounded-lg ${
                          isUnlocked ? 'bg-purple-900/30' : 'bg-gray-700 opacity-50'
                        }`}
                      >
                        <span className="text-2xl">{ach.icon}</span>
                        <div className="flex-1">
                          <div className="font-medium text-sm">{ach.name}</div>
                          <div className="text-xs text-gray-400">{ach.description}</div>
                        </div>
                        {isUnlocked && (
                          <span className="text-purple-400 text-xs font-medium">+{ach.xpReward} XP</span>
                        )}
                      </div>
                    );
                  })}
                </div>
              </div>

              {/* Credentials */}
              <div className="bg-gray-800 rounded-xl p-6">
                <h3 className="font-bold mb-4">Credentials</h3>
                <div className="text-center py-8 text-gray-400">
                  <div className="text-4xl mb-2">🏅</div>
                  <p className="text-sm">Complete courses to earn NFT credentials</p>
                </div>
              </div>
            </div>
          </div>
        </main>
      </div>
    </>
  );
}

function StatBox({ label, value }: { label: string; value: string }) {
  return (
    <div className="bg-gray-700 rounded-lg p-3 text-center">
      <div className="text-2xl font-bold">{value}</div>
      <div className="text-xs text-gray-400">{label}</div>
    </div>
  );
}

function ActivityItem({ icon, text, xp, time }: { icon: string; text: string; xp: number; time: string }) {
  return (
    <div className="flex items-center gap-4 p-3 bg-gray-700/50 rounded-lg">
      <span className="text-xl">{icon}</span>
      <div className="flex-1">
        <div className="text-sm">{text}</div>
        <div className="text-xs text-gray-400">{time}</div>
      </div>
      {xp > 0 && (
        <span className="text-purple-400 text-sm font-medium">+{xp} XP</span>
      )}
    </div>
  );
}
