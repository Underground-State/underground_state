# Underground State - Development Plan

## Обзор проекта

Discord-подобная платформа с Web3 аутентификацией, голосовыми/видео звонками, KYC верификацией и DAO governance на блокчейне SUI.

### Выбранный стек

| Компонент | Технология |
|-----------|------------|
| Backend | Rust (Axum) - модульный монолит |
| Database | PostgreSQL + Redis |
| Frontend | Flutter Web |
| Blockchain | SUI (Move) |
| Voice/Video | mediasoup (WebRTC SFU) |
| Auth | Web3 кошельки (MetaMask, Phantom, SUI Wallet) |
| Captcha | hCaptcha |
| KYC Storage | Локальное шифрование (AES-256-GCM) |

---

## Фаза 1: Инфраструктура и Auth (MVP Core)

### 1.1 Структура проекта

```
underground_state/
├── backend/                    # Rust модульный монолит
│   ├── Cargo.toml
│   ├── migrations/
│   └── crates/
│       ├── app/               # Entry point
│       ├── core/              # Shared types, errors
│       ├── db/                # Database + sharding
│       ├── auth/              # Web3 + hCaptcha
│       ├── users/
│       ├── guilds/
│       ├── channels/
│       ├── messaging/         # REST + WebSocket
│       ├── voice/             # mediasoup
│       ├── kyc/               # Document encryption
│       └── common/            # Utilities
│
├── frontend/                   # Flutter Web
│   └── lib/
│       ├── core/
│       ├── shared/
│       └── features/
│           ├── auth/
│           ├── chat/
│           ├── voice/
│           └── kyc/
│
├── contracts/                  # SUI Move
│   ├── Move.toml
│   └── sources/
│       ├── governance/
│       │   ├── token.move
│       │   ├── dao.move
│       │   └── treasury.move
│       └── dex/
│           └── integrations.move
│
└── docs/
```

### 1.2 Backend: Auth модуль

**Файлы:**
- `backend/crates/auth/src/web3.rs` - SIWE (Sign-In with Ethereum)
- `backend/crates/auth/src/captcha.rs` - hCaptcha верификация
- `backend/crates/auth/src/jwt.rs` - JWT токены

**API Endpoints:**
```
POST /api/v1/auth/nonce     - Получить nonce для подписи
POST /api/v1/auth/verify    - Верификация подписи + hCaptcha
POST /api/v1/auth/refresh   - Обновление JWT
DELETE /api/v1/auth/logout  - Выход
```

### 1.3 Database Schema

```sql
-- users
CREATE TABLE users (
    id BIGINT PRIMARY KEY,              -- Snowflake ID
    wallet_address VARCHAR(66) NOT NULL UNIQUE,
    username VARCHAR(32) NOT NULL UNIQUE,
    kyc_status VARCHAR(16) DEFAULT 'none',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    shard_id SMALLINT NOT NULL
);

-- guilds
CREATE TABLE guilds (
    id BIGINT PRIMARY KEY,
    owner_id BIGINT REFERENCES users(id),
    name VARCHAR(100) NOT NULL,
    shard_id SMALLINT NOT NULL
);

-- channels
CREATE TABLE channels (
    id BIGINT PRIMARY KEY,
    guild_id BIGINT REFERENCES guilds(id),
    channel_type SMALLINT NOT NULL,     -- 0=text, 1=voice
    name VARCHAR(100),
    shard_id SMALLINT NOT NULL
);

-- messages
CREATE TABLE messages (
    id BIGINT PRIMARY KEY,
    channel_id BIGINT REFERENCES channels(id),
    author_id BIGINT REFERENCES users(id),
    content TEXT NOT NULL,
    shard_id SMALLINT NOT NULL
);

-- kyc_documents
CREATE TABLE kyc_documents (
    id BIGINT PRIMARY KEY,
    user_id BIGINT REFERENCES users(id),
    document_type VARCHAR(32),
    storage_path VARCHAR(512),
    encryption_key_id VARCHAR(64),
    status VARCHAR(16) DEFAULT 'pending',
    shard_id SMALLINT NOT NULL
);
```

---

## Фаза 2: Messaging (Real-time Chat)

### 2.1 WebSocket Gateway

**Файл:** `backend/crates/messaging/src/gateway.rs`

**События:**
```typescript
// Client → Server
{ "op": 0, "d": { "token": "jwt..." } }           // IDENTIFY
{ "op": 1, "d": null }                             // HEARTBEAT

// Server → Client
{ "op": 0, "t": "MESSAGE_CREATE", "d": {...} }
{ "op": 0, "t": "PRESENCE_UPDATE", "d": {...} }
```

### 2.2 Flutter Chat Feature

**Файлы:**
- `frontend/lib/features/chat/presentation/bloc/chat_bloc.dart`
- `frontend/lib/features/chat/data/datasources/chat_websocket_datasource.dart`

---

## Фаза 3: Voice/Video (mediasoup)

### 3.1 Backend SFU

**Файл:** `backend/crates/voice/src/sfu.rs`

```rust
pub struct VoiceServer {
    worker: mediasoup::Worker,
    routers: HashMap<i64, Router>,  // channel_id → Router
}
```

### 3.2 Flutter WebRTC

**Пакет:** `mediasfu_mediasoup_client: ^0.0.8`

**Файлы:**
- `frontend/lib/features/voice/data/datasources/mediasoup_datasource.dart`
- `frontend/lib/features/voice/presentation/bloc/voice_bloc.dart`

---

## Фаза 4: KYC

### 4.1 Document Encryption

**Файл:** `backend/crates/kyc/src/encryption.rs`

- AES-256-GCM шифрование
- Локальное хранение: `/kyc/{user_id}/{doc_type}/{uuid}.enc`
- SHA-256 хеш оригинала для верификации

### 4.2 Moderation Queue

**API:**
```
POST   /api/v1/kyc/documents           - Загрузка документа
GET    /api/v1/kyc/status              - Статус KYC
GET    /api/v1/admin/kyc/queue         - Очередь модерации
POST   /api/v1/admin/kyc/{id}/approve  - Одобрить
POST   /api/v1/admin/kyc/{id}/reject   - Отклонить
```

---

## Фаза 5: SUI Blockchain

### 5.1 Governance Token (UGS)

**Файл:** `contracts/sources/governance/token.move`

```move
module underground_state::token {
    public struct UGS has drop {}

    fun init(witness: UGS, ctx: &mut TxContext) {
        let (treasury_cap, metadata) = coin::create_currency<UGS>(
            witness, 9, b"UGS", b"Underground State", ...
        );
        transfer::public_freeze_object(metadata);
        transfer::public_transfer(treasury_cap, tx_context::sender(ctx));
    }
}
```

### 5.2 DAO Contract

**Файл:** `contracts/sources/governance/dao.move`

- Token voting (1 токен = 1 голос)
- Proposal lifecycle: PENDING → ACTIVE → SUCCEEDED/DEFEATED → EXECUTED
- Quorum validation
- Treasury management

### 5.3 DEX Integration

**Файл:** `contracts/sources/dex/integrations.move`

- Интеграция с Cetus CLMM
- Swap wrapper
- Liquidity management

---

## Фаза 6: Sharding

### 6.1 Shard Router

**Файл:** `backend/crates/db/src/shard.rs`

```rust
pub struct ShardRouter {
    shards: HashMap<u16, PgPool>,
    shard_count: u16,
}

impl ShardRouter {
    pub fn get_shard_id(&self, entity_id: i64) -> u16 {
        (murmur3_hash(entity_id) % self.shard_count as u32) as u16
    }
}
```

### 6.2 Sharding Strategy

| Entity | Shard Key |
|--------|-----------|
| Users | `hash(user_id)` |
| Guilds | `hash(guild_id)` |
| Messages | `hash(channel_id)` |
| KYC | `hash(user_id)` |

---

## Фаза 7: Logo & Branding

- Дизайн логотипа (отдельная задача)
- Цветовая схема в стиле Discord
- Primary: `#5865F2` (blurple)
- Иконки для токена UGS

---

## Критические файлы

### Backend
1. `backend/crates/app/src/main.rs` - Entry point
2. `backend/crates/db/src/shard.rs` - Sharding router
3. `backend/crates/auth/src/web3.rs` - SIWE auth
4. `backend/crates/messaging/src/gateway.rs` - WebSocket gateway
5. `backend/crates/kyc/src/encryption.rs` - Document encryption

### Frontend
1. `frontend/lib/core/di/injection_container.dart` - DI setup
2. `frontend/lib/core/router/app_router.dart` - Navigation
3. `frontend/lib/features/auth/data/datasources/web3/wallet_provider.dart` - Wallet interface
4. `frontend/lib/features/voice/data/datasources/mediasoup_datasource.dart` - WebRTC

### Contracts
1. `contracts/sources/governance/token.move` - UGS token
2. `contracts/sources/governance/dao.move` - DAO voting
3. `contracts/sources/dex/integrations.move` - Cetus integration

---

## Ключевые зависимости

### Rust (Cargo.toml)
```toml
axum = "0.8"
sqlx = { version = "0.8", features = ["postgres"] }
redis = "0.27"
siwe = "0.6"
hcaptcha = "3.0"
mediasoup = "0.20"
aes-gcm = "0.10"
jsonwebtoken = "9"
tokio = { version = "1.43", features = ["full"] }
```

### Flutter (pubspec.yaml)
```yaml
flutter_bloc: ^8.1.3
go_router: ^13.0.0
get_it: ^7.6.4
dio: ^5.4.0
flutter_web3: ^2.1.9
web3auth_flutter: ^4.0.0
mediasfu_mediasoup_client: ^0.0.8
file_picker: ^6.1.1
```

### SUI Move (Move.toml)
```toml
[dependencies]
Sui = { git = "https://github.com/MystenLabs/sui.git", rev = "framework/mainnet" }
CetusClmm = { git = "https://github.com/CetusProtocol/cetus-contracts.git" }
```

---

## Порядок реализации

1. **Backend Foundation** - проект структура, config, database migrations
2. **Auth** - Web3 SIWE + hCaptcha + JWT
3. **Users & Guilds** - CRUD операции
4. **Messaging** - REST API + WebSocket gateway
5. **KYC** - Upload + encryption + moderation
6. **Voice** - mediasoup integration
7. **Flutter App** - Auth + Chat + Voice + KYC flows
8. **SUI Contracts** - Token + DAO + DEX integration
9. **Sharding** - Horizontal scaling
10. **Testing & Deployment**

---

# README.md

```markdown
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
cd backend
cp .env.example .env
cargo build --release
sqlx database create
sqlx migrate run
cargo run
```

### Frontend

```bash
cd frontend
flutter pub get
flutter run -d chrome
```

### Smart Contracts

```bash
cd contracts
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
```
