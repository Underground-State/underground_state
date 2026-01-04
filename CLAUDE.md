# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Underground State is a Discord-like platform with Web3 authentication, voice/video calls, KYC verification, and DAO governance on the SUI blockchain.

## Tech Stack

- **Backend**: Rust (Axum) modular monolith with PostgreSQL + Redis
- **Frontend**: Flutter Web (not yet implemented)
- **Blockchain**: SUI (Move) smart contracts (not yet implemented)
- **Voice**: mediasoup (WebRTC SFU) (planned)
- **Auth**: SIWE (Sign-In with Ethereum) + hCaptcha

## Commands

### Backend (Rust)

```bash
# From source_code/server/
cargo build                    # Build all crates
cargo build --release          # Production build
cargo run -p ug-app            # Run the main server
cargo test                     # Run all tests
cargo test -p ug-auth          # Test specific crate
cargo clippy                   # Lint
cargo fmt                      # Format code

# Database (requires sqlx-cli)
sqlx database create           # Create database
sqlx migrate run               # Run migrations
```

### Frontend (Flutter) - When implemented

```bash
# From source_code/client/
flutter pub get                # Install dependencies
flutter run -d chrome          # Run in browser
flutter test                   # Run tests
```

### Smart Contracts (SUI) - When implemented

```bash
# From source_code/contracts/
sui move build                 # Build contracts
sui move test                  # Run tests
sui client publish --gas-budget 100000000
```

## Architecture

### Backend Crate Structure

```
source_code/server/
├── Cargo.toml              # Workspace root
└── crates/
    ├── app/                # Entry point (ug-app), binary: underground-state
    ├── core/               # Shared types, errors (ug-core)
    ├── common/             # Utilities like Snowflake ID generator (ug-common)
    ├── db/                 # PostgreSQL + Redis connections (ug-db)
    ├── auth/               # Web3 SIWE + hCaptcha + JWT (ug-auth)
    ├── users/              # User management (ug-users)
    ├── guilds/             # Guild/server management (ug-guilds)
    ├── channels/           # Text/voice channels (ug-channels)
    ├── messaging/          # REST + WebSocket gateway (ug-messaging)
    ├── kyc/                # Document encryption + verification (ug-kyc)
    └── voice/              # mediasoup SFU integration (ug-voice)
```

### Key Patterns

- **Error handling**: Use `ug_core::Error` and `ug_core::Result<T>` for all error types. Errors include `status_code()` and `error_code()` methods for API responses.
- **IDs**: All entities use Snowflake IDs (`i64`). Generate with `ug_common::snowflake::generate_id()`.
- **Database sharding**: Entities include `shard_id` field. Shard key is `hash(entity_id) % shard_count`.
- **Auth flow**: Nonce generation → SIWE message signing → Signature verification + hCaptcha → JWT issuance.

### Database State

The `DbState` struct in `ug-db` holds both PostgreSQL (`PgPool`) and Redis (`RedisPool`) connections and is passed as Axum state.

## API Endpoints

```
POST   /api/v1/auth/nonce       # Get nonce for wallet signing
POST   /api/v1/auth/verify      # Verify signature + hCaptcha → JWT
POST   /api/v1/auth/refresh     # Refresh JWT
DELETE /api/v1/auth/logout      # Logout
```

## Environment Variables

```env
DATABASE_URL=postgres://user:pass@localhost/underground_state
REDIS_URL=redis://localhost:6379
JWT_SECRET=your-secret-key
HCAPTCHA_SECRET=your-hcaptcha-secret
MEDIASOUP_WORKERS=4
SUI_RPC_URL=https://fullnode.mainnet.sui.io
```

## Development Notes

- The project is in early development. Only the Rust backend structure and auth module are partially implemented.
- Frontend (Flutter) and smart contracts (SUI Move) directories are planned but not yet created.
- Documentation is bilingual (Russian/English) - the README and DEVPLAN contain Russian text.
