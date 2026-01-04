# Underground State

Discord-подобная платформа с Web3 аутентификацией, голосовыми звонками и DAO governance на SUI блокчейне.

## Особенности

- **Web3 Auth** - Вход через MetaMask, Phantom, SUI Wallet
- **Real-time Chat** - WebSocket messaging с шардингом
- **Voice/Video** - mediasoup SFU для масштабируемых звонков
- **KYC** - Паспортная верификация с AES-256 шифрованием
- **DAO Governance** - Token voting на SUI блокчейне
- **DEX Integration** - Cetus CLMM для торговли токенами

## Стек технологий

| Layer | Technology |
|-------|------------|
| Backend | Rust (Axum), PostgreSQL, Redis |
| Frontend | Flutter Web |
| Blockchain | SUI (Move) |
| Voice | mediasoup (WebRTC SFU) |
| Auth | SIWE, hCaptcha |

## Быстрый старт

### Prerequisites

- Rust 1.75+
- Flutter 3.16+
- PostgreSQL 15+
- Redis 7+
- SUI CLI

### Backend

```bash
cd source_code/server
cp .env.example .env
cargo build --release
sqlx database create
sqlx migrate run
cargo run
```

### Frontend

```bash
cd source_code/client
flutter pub get
flutter run -d chrome
```

### Smart Contracts

```bash
cd source_code/contracts
sui move build
sui move test
sui client publish --gas-budget 100000000
```

## Архитектура

```
┌─────────────────────────────────────────────────────────────┐
│                     Flutter Web Client                       │
│          (Chat, Voice, KYC, Wallet Connection)               │
└─────────────────────────┬───────────────────────────────────┘
                          │ HTTPS + WebSocket
┌─────────────────────────▼───────────────────────────────────┐
│                    Rust Backend (Axum)                       │
│  ┌──────┐ ┌──────┐ ┌─────────┐ ┌───────┐ ┌─────┐           │
│  │ Auth │ │ Chat │ │ Voice   │ │ KYC   │ │ API │           │
│  │      │ │      │ │(mediasoup)│       │ │     │           │
│  └──────┘ └──────┘ └─────────┘ └───────┘ └─────┘           │
└─────────────────────────┬───────────────────────────────────┘
                          │
        ┌─────────────────┼─────────────────┐
        ▼                 ▼                 ▼
┌──────────────┐  ┌──────────────┐  ┌──────────────┐
│  PostgreSQL  │  │    Redis     │  │ SUI Network  │
│  (Sharded)   │  │  (Sessions)  │  │ (DAO/Token)  │
└──────────────┘  └──────────────┘  └──────────────┘
```

## Структура проекта

```
underground_state/
├── docs/                       # Документация
│   ├── api/                   # OpenAPI specs
│   ├── architecture/          # ADRs
│   └── guides/                # Руководства
│
├── source_code/
│   ├── server/                # Rust Backend
│   ├── client/                # Flutter Web
│   └── contracts/             # SUI Move
│
├── DEVPLAN.md                 # План разработки
└── README.md
```

## API

### Authentication

```bash
# Get nonce
curl -X POST /api/v1/auth/nonce \
  -d '{"wallet_address": "0x..."}'

# Verify signature
curl -X POST /api/v1/auth/verify \
  -d '{"wallet_address": "0x...", "signature": "0x...", "nonce": "...", "hcaptcha_token": "..."}'
```

### Messages

```bash
# Get messages
curl -H "Authorization: Bearer <jwt>" \
  /api/v1/channels/{channel_id}/messages

# Send message
curl -X POST -H "Authorization: Bearer <jwt>" \
  /api/v1/channels/{channel_id}/messages \
  -d '{"content": "Hello!"}'
```

### WebSocket Gateway

```javascript
const ws = new WebSocket('wss://api.example.com/gateway');

// Identify
ws.send(JSON.stringify({ op: 0, d: { token: 'jwt...' } }));

// Listen for messages
ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  if (data.t === 'MESSAGE_CREATE') {
    console.log('New message:', data.d);
  }
};
```

## Конфигурация

### Environment Variables

```env
DATABASE_URL=postgres://user:pass@localhost/underground_state
REDIS_URL=redis://localhost:6379
JWT_SECRET=your-secret-key
HCAPTCHA_SECRET=your-hcaptcha-secret
MEDIASOUP_WORKERS=4
SUI_RPC_URL=https://fullnode.mainnet.sui.io
```

## Лицензия

MIT

## Контакты

- GitHub: [underground-state](https://github.com/underground-state)
- Discord: [Join our server](https://discord.gg/underground)
