# AuthSession - Rust Authentication System

Sistem autentikasi berbasis Rust dengan dukungan Email/Password dan Google OAuth login menggunakan Axum framework.

## 🚀 Quick Start

### 1. Clone & Setup

```bash
# Install Rust jika belum
# https://rustup.rs/

# Install Diesel CLI
cargo install diesel_cli --no-default-features --features postgres

# Clone repository (jika dari git)
git clone <repository-url>
cd AuthSession
```

### 2. Konfigurasi

File `.env` sudah dikonfigurasi dengan:
- Server: `0.0.0.0:8000`
- Database: Remote PostgreSQL
- Google OAuth: Configured

### 3. Jalankan Database Migrations

```bash
diesel migration run
```

Diesel CLI otomatis membaca `DATABASE_URL` dari file `.env`

### 4. Jalankan Server

```bash
cargo run
```

Server akan berjalan di: `http://0.0.0.0:8000`

### 5. Akses Aplikasi

- **Registration**: http://localhost:8000/static/register.html
- **Login**: http://localhost:8000/static/login.html
- **Dashboard**: http://localhost:8000/static/dashboard.html

## 📚 Dokumentasi

- **[SETUP.md](SETUP.md)** - Setup lengkap aplikasi dan Google OAuth
- **[MIGRATION_GUIDE.md](MIGRATION_GUIDE.md)** - Panduan database migrations (Bahasa Indonesia)
- **[migrations/README.md](../../migrations/README.md)** - Detail tentang migrations

## 🔑 Fitur

### Authentication Methods
- ✅ Email/Password Registration & Login
- ✅ Google OAuth 2.0 Login
- ✅ JWT Token-based Authentication
- ✅ Secure Password Hashing (Argon2)

### Technical Stack
- **Framework**: Axum 0.8
- **Database**: PostgreSQL + Diesel ORM
- **Async Runtime**: Tokio
- **OAuth**: oauth2 crate
- **Password**: Argon2
- **JWT**: jsonwebtoken

### Database Features
- ✅ PostgreSQL dengan Diesel ORM
- ✅ Async queries dengan diesel-async
- ✅ Migration system
- ✅ UUID primary keys
- ✅ Support OAuth dan Email/Password
- ✅ Indexed queries untuk performance

## 🗂️ Project Structure

```
AuthSession/
├── src/
│   ├── main.rs              # Entry point
│   ├── lib.rs               # Module exports
│   ├── server.rs            # Server setup
│   ├── config.rs            # Configuration
│   ├── error.rs             # Error handling
│   ├── schema.rs            # Diesel schema
│   ├── handlers/            # Request handlers
│   │   ├── auth_handler.rs  # Auth logic
│   │   └── user_handler.rs  # User endpoints
│   ├── routes/              # Route definitions
│   │   ├── auth.rs          # Auth routes
│   │   ├── home.rs          # Home routes
│   │   └── profile.rs       # Profile routes
│   ├── models/              # Data models
│   │   └── user.rs          # User model
│   ├── db/                  # Database layer
│   │   ├── pool.rs          # Connection pool
│   │   ├── store.rs         # In-memory store
│   │   └── diesel_store.rs  # Diesel implementation
│   ├── middleware/          # Middleware
│   │   └── auth_middleware.rs
│   ├── utils/               # Utilities
│   │   ├── hashing.rs       # Password hashing
│   │   └── jwt.rs           # JWT tokens
│   └── static/              # Frontend
│       ├── register.html
│       ├── login.html
│       └── dashboard.html
├── migrations/              # Database migrations
│   └── 2025-10-07-xxx_create_users_table/
│       ├── up.sql
│       └── down.sql
├── config/
│   └── default.toml         # Default config
├── diesel-run.ps1           # Helper script (PowerShell)
├── diesel-run.bat           # Helper script (CMD)
├── diesel-run.sh            # Helper script (Bash)
├── .env                     # Environment config
├── diesel.toml              # Diesel config
└── Cargo.toml               # Dependencies
```

## 🔧 Configuration

### Environment Variables

File `.env` berisi:

```env
# Diesel CLI (untuk migrations)
DATABASE_URL=postgres://user:pass@host:port/db

# Application config
APP_SERVER__HOST=0.0.0.0
APP_SERVER__PORT=8000
APP_DATABASE__URL=postgres://user:pass@host:port/db
APP_DATABASE__MAX_CONNECTIONS=10
APP_JWT__SECRET=your-secret-key
APP_JWT__EXPIRATION=1200
APP_GOOGLE_OAUTH__CLIENT_ID=your-client-id
APP_GOOGLE_OAUTH__CLIENT_SECRET=your-client-secret
APP_GOOGLE_OAUTH__REDIRECT_URL=http://localhost:8000/api/auth/google/callback
APP_GOOGLE_OAUTH__AUTH_URL=https://accounts.google.com/o/oauth2/v2/auth
APP_GOOGLE_OAUTH__TOKEN_URL=https://oauth2.googleapis.com/token
```

**Note**: `DATABASE_URL` digunakan oleh Diesel CLI, sedangkan `APP_DATABASE__URL` digunakan oleh aplikasi.

## 🛠️ Development

### Running Migrations

Diesel CLI otomatis membaca `DATABASE_URL` dari `.env`:

```bash
# Run migrations
diesel migration run

# List migrations
diesel migration list

# Revert last migration
diesel migration revert

# Create new migration
diesel migration generate migration_name
```

### Build & Run

```bash
# Development build
cargo build
cargo run

# Release build
cargo build --release
./target/release/auth_session

# Check code
cargo check

# Run tests
cargo test
```

## 📡 API Endpoints

### Authentication

#### Register with Email
```http
POST /api/auth/register
Content-Type: application/json

{
  "name": "John Doe",
  "email": "john@example.com",
  "password": "password123"
}
```

#### Login with Email
```http
POST /api/auth/login
Content-Type: application/json

{
  "email": "john@example.com",
  "password": "password123"
}
```

#### Google OAuth Login
```http
GET /api/auth/google
```
Redirects to Google consent screen

#### Google OAuth Callback
```http
GET /api/auth/google/callback?code=xxx&state=xxx
```
Handled automatically by Google

#### Logout
```http
POST /api/auth/logout
```

### Response Format

```json
{
  "token": "eyJ0eXAiOiJKV1QiLCJhbGc...",
  "user": {
    "id": "uuid-here",
    "email": "john@example.com",
    "name": "John Doe"
  }
}
```

## 🗄️ Database Schema

### Users Table

```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    password_hash VARCHAR(255),              -- NULL for OAuth
    oauth_provider VARCHAR(50),              -- 'google'
    oauth_id VARCHAR(255),                   -- Google user ID
    created_at TIMESTAMP NOT NULL DEFAULT NOW,
    updated_at TIMESTAMP NOT NULL DEFAULT NOW,
    
    CONSTRAINT check_auth_method CHECK (
        (password_hash IS NOT NULL AND oauth_provider IS NULL) OR
        (password_hash IS NULL AND oauth_provider IS NOT NULL)
    )
);

CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_oauth ON users(oauth_provider, oauth_id);
```

## 🔐 Security

- ✅ Passwords hashed dengan Argon2
- ✅ JWT dengan expiration (20 menit default)
- ✅ CORS configured
- ✅ Environment variables untuk sensitive data
- ⚠️ Change JWT secret di production!
- ⚠️ Gunakan HTTPS di production!

## 🐛 Troubleshooting

### Migration Failed

```bash
# Check connection
psql -h 13.212.194.164 -p 2314 -U postgres -d cobafitur

# Revert migrations
diesel migration revert

# Try again
diesel migration run
```

### Google OAuth Error

Pastikan di Google Cloud Console:
1. Authorized redirect URIs sudah benar: `http://localhost:8000/api/auth/google/callback`
2. Client ID dan Secret correct
3. OAuth consent screen configured

### Port Already in Use

Ubah port di `.env`:
```env
APP_SERVER__PORT=9000
```

## 📝 License

This project is for educational purposes.

## 🤝 Contributing

1. Fork the repository
2. Create feature branch
3. Commit changes
4. Push to branch
5. Create Pull Request

## 📞 Support

Untuk pertanyaan atau issues, lihat:
- [SETUP.md](SETUP.md) - Setup guide
- [MIGRATION_GUIDE.md](MIGRATION_GUIDE.md) - Database migrations
- Issues section di GitHub

---

**Built with ❤️ using Rust, Axum, and PostgreSQL**
