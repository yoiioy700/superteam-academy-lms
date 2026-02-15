# Superteam Academy LMS

Gamified Learning Management System on Solana with on-chain credentials.

## Architecture

### On-Chain Program (Anchor)
- **4 Account Types**: Config, Course, LearnerProfile, Enrollment
- **16 Instructions**: Full learning lifecycle
- **Token-2022 XP**: Soulbound, non-transferable
- **Metaplex Core**: Wallet-visible credential NFTs

### Frontend (Next.js)
- Wallet authentication
- Course catalog & content
- Code editor integration
- Gamification (XP, streaks, achievements)
- Credential viewer

## Quick Start

### Prerequisites
- Rust >= 1.75
- Node.js >= 18
- Solana CLI

### Build Program
```bash
cd superteam-academy
anchor build
```

### Run Frontend
```bash
cd app
npm install
npm run dev
```

## Features

- ✅ Course enrollment with prerequisites
- ✅ XP tracking (Token-2022)
- ✅ Streak system with freezes
- ✅ Achievement bitmaps
- ✅ Metaplex Core credentials
- ✅ Creator rewards
- ✅ Rate limiting

## Deployed Program

Devnet: `Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS`

## License

MIT
