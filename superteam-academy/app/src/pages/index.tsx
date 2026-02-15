import Head from 'next/head';
import Link from 'next/link';
import { useWallet } from '@solana/wallet-adapter-react';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';

export default function Home() {
  const { connected } = useWallet();

  return (
    <>
      <Head>
        <title>Superteam Academy</title>
        <meta name="description" content="Learn Solana, earn credentials" />
      </Head>
      
      <div className="min-h-screen bg-gray-900 text-white">
        <header className="p-6 flex justify-between items-center border-b border-gray-800">
          <h1 className="text-2xl font-bold bg-gradient-to-r from-purple-400 to-pink-400 bg-clip-text text-transparent">
            Superteam Academy
          </h1>
          <WalletMultiButton className="!bg-purple-600 hover:!bg-purple-700" />
        </header>

        <main className="container mx-auto px-6 py-12">
          {!connected ? (
            <div className="text-center py-20">
              <h2 className="text-4xl font-bold mb-6">Learn Solana. Get Credentials.</h2>
              <p className="text-gray-400 mb-8">
                Master blockchain development with hands-on courses
              </p>
              <div className="flex justify-center gap-4">
                <WalletMultiButton />
              </div>
            </div>
          ) : (
            <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
              <CourseCard
                title="Anchor Framework"
                level="Beginner"
                lessons={10}
                xp={500}
                track="Anchor"
              />
              <CourseCard
                title="Rust for Solana"
                level="Intermediate"
                lessons={15}
                xp={1000}
                track="Rust"
              />
              <CourseCard
                title="Program Security"
                level="Advanced"
                lessons={20}
                xp={2000}
                track="Security"
              />
            </div>
          )}
        </main>
      </div>
    </>
  );
}

function CourseCard({ title, level, lessons, xp, track }: {
  title: string;
  level: string;
  lessons: number;
  xp: number;
  track: string;
}) {
  const colors: Record<string, string> = {
    Anchor: 'bg-blue-500',
    Rust: 'bg-orange-500',
    Security: 'bg-red-500',
    DeFi: 'bg-green-500',
  };

  return (
    <div className="bg-gray-800 rounded-xl p-6 border border-gray-700 hover:border-purple-500 transition-colors">
      <div className={`w-12 h-12 rounded-lg ${colors[track] || 'bg-gray-600'} flex items-center justify-center mb-4`>
        <span className="text-xl font-bold">{track[0]}</span>
      </div>
      <h3 className="text-xl font-semibold mb-2">{title}</h3>
      <p className="text-gray-400 text-sm mb-4">{level} • {lessons} lessons • {xp} XP</p>
      <button className="w-full py-2 bg-purple-600 rounded-lg hover:bg-purple-700 transition-colors">
        Start Course
      </button>
    </div>
  );
}
