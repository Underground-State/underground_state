# Underground State

## What is it?

Underground State is a platform for secure communication and collaboration with blockchain technology integration. The platform combines instant messaging, voice and video calls, and decentralized community governance.

## Key Features

### Secure Authentication via Crypto Wallets

Sign-in is performed through cryptographic wallets (MetaMask, Phantom, SUI Wallet). This means:
- No passwords to remember
- Users have full control over their accounts
- Account forgery or theft is impossible without wallet access

### Real-time Communication

- **Text messages** — instant delivery via WebSocket
- **Voice calls** — high-quality communication with large group support
- **Video calls** — conferences with no participant limits

### Identity Verification (KYC)

Optional passport verification for:
- Increasing trust level within the community
- Access to advanced features
- Participation in DAO voting

Documents are stored locally with AES-256-GCM encryption — only moderators have access.

### Decentralized Governance (DAO)

The community is governed by UGS tokens on the SUI blockchain:
- **Voting** — one token = one vote
- **Proposals** — any token holder can create a proposal
- **Treasury** — community funds are managed by smart contracts
- **Transparency** — all decisions are recorded on the blockchain

### DEX Integration

Built-in Cetus CLMM integration allows:
- Swapping tokens directly within the app
- Adding liquidity
- Participating in the platform economy

## Who is it for?

### Crypto Communities
A secure space for discussing projects, coordinating actions, and making collective decisions.

### DAOs and Decentralized Organizations
Complete infrastructure for governance: from discussions to voting and fund distribution.

### Developer Teams
Secure communication with participant verification and Web3 tool integration.

### Privacy-focused Communities
Cryptographic authentication and data encryption provide maximum protection.

## Project Goals

### 1. User Sovereignty
Users own their accounts through crypto wallets. The platform cannot block or delete an account without a DAO decision.

### 2. Decentralized Governance
Key decisions about platform development are made by the community through voting, not by a single owner.

### 3. Transparency
All financial operations, votes, and decisions are recorded on the public SUI blockchain.

### 4. Scalability
Sharded architecture allows serving millions of users without performance degradation.

### 5. Privacy
- Messages are stored on sharded servers
- KYC documents are encrypted with AES-256
- Authentication does not require personal data

## Technology Stack

| Component | Technology |
|-----------|------------|
| Backend | Rust (Axum) |
| Database | PostgreSQL + Redis |
| Frontend | Flutter Web |
| Blockchain | SUI (Move) |
| Voice/Video | mediasoup (WebRTC) |
| Encryption | AES-256-GCM |

## How does it work?

```
┌─────────────────────────────────────────────────────────────┐
│                          User                                │
│              (Flutter Web / Mobile App)                      │
└─────────────────────────┬───────────────────────────────────┘
                          │ HTTPS + WebSocket
                          ▼
┌─────────────────────────────────────────────────────────────┐
│                    Rust Backend (Axum)                       │
│                                                              │
│   ┌────────────┐  ┌────────────┐  ┌────────────────────┐    │
│   │    Auth    │  │  Messaging │  │   Voice/Video      │    │
│   │  (Web3)    │  │ (WebSocket)│  │   (mediasoup)      │    │
│   └────────────┘  └────────────┘  └────────────────────┘    │
│                                                              │
│   ┌────────────┐  ┌────────────┐  ┌────────────────────┐    │
│   │    KYC     │  │   Guilds   │  │     Channels       │    │
│   │(Encrypted) │  │            │  │                    │    │
│   └────────────┘  └────────────┘  └────────────────────┘    │
└─────────────────────────┬───────────────────────────────────┘
                          │
        ┌─────────────────┼─────────────────┐
        ▼                 ▼                 ▼
┌──────────────┐  ┌──────────────┐  ┌──────────────┐
│  PostgreSQL  │  │    Redis     │  │ SUI Network  │
│  (Sharded)   │  │  (Sessions)  │  │ (DAO/Token)  │
└──────────────┘  └──────────────┘  └──────────────┘
```

## Getting Started

### For Users

1. Install a crypto wallet (MetaMask, Phantom, or SUI Wallet)
2. Navigate to the web application
3. Connect your wallet and sign the message to log in
4. Join guilds or create your own

### For Developers

```bash
# Clone the repository
git clone https://github.com/underground-state/underground_state

# Run the backend
cd source_code/server
cp .env.example .env
cargo run

# Run the frontend
cd source_code/client
flutter pub get
flutter run -d chrome
```

## Roadmap

1. **Infrastructure and Auth** — basic Web3 authorization
2. **Messaging** — real-time text messages
3. **Voice/Video** — voice and video calls
4. **KYC** — identity verification
5. **SUI Integration** — UGS token, DAO, DEX integration
6. **Sharding** — horizontal scaling

## License

MIT
