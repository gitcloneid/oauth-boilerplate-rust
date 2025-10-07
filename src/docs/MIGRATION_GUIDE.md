# Migration Guide - Menjalankan Database Migrations

## Setup Cepat

Konfigurasi Anda sudah lengkap berdasarkan `.env`:
- **Server**: 0.0.0.0:8000
- **Database**: postgres://postgres:password123@13.212.194.164:2314/cobafitur
- **OAuth Redirect**: http://localhost:8000/api/auth/google/callback

## Langkah-langkah Menjalankan Migrations

Diesel CLI otomatis membaca `DATABASE_URL` dari file `.env` yang sudah dikonfigurasi!

### Jalankan Migrations

```bash
# Jalankan semua pending migrations
diesel migration run

# List status migrations
diesel migration list

# Revert migration terakhir
diesel migration revert

# Redo migration terakhir
diesel migration redo
```

**Note**: Diesel CLI secara otomatis membaca `DATABASE_URL` dari file `.env` di root project.

Output yang diharapkan:
```
Running migration 2025-10-07-111620-0000_create_users_table
```

### 2. Verifikasi Migration Berhasil

Cek tabel yang dibuat:

```bash
# Connect ke database
psql -h 13.212.194.164 -p 2314 -U postgres -d cobafitur

# List tables
\dt

# Cek struktur tabel users
\d users

# Exit
\q
```

### 3. Update Server Configuration (Opsional)

Jika ingin menggunakan DieselStore untuk database persistence, update `src/server.rs`:

```rust
// Ganti dari Store (in-memory) ke DieselStore (PostgreSQL)
use crate::db::{DieselStore, create_pool};

pub async fn run(config: AppConfig) -> Result<(), AppError> {
    // Create database connection pool
    let pool = create_pool(
        &config.database.url,
        config.database.max_connections
    ).await?;
    
    let diesel_store = DieselStore::new(pool);
    
    let app_state = AppState {
        config: config.clone(),
        store: diesel_store, // Gunakan DieselStore
    };
    
    // ... rest of code
}
```

## Perintah Diesel CLI Berguna

### Melihat Status Migrations
```bash
diesel migration list
```

### Rollback Migration Terakhir
```bash
diesel migration revert
```

### Redo Migration Terakhir
```bash
diesel migration redo
```

### Generate Migration Baru
```bash
diesel migration generate nama_migration_baru
```

### Update Rust Schema
```bash
diesel print-schema > src/schema.rs
```

## Troubleshooting

### Error: Connection Failed

Pastikan:
1. PostgreSQL server di `13.212.194.164:2314` sedang running
2. Firewall mengizinkan koneksi ke port 2314
3. Password correct: `password123`
4. Database `cobafitur` sudah ada

Test koneksi:
```bash
psql -h 13.212.194.164 -p 2314 -U postgres -d cobafitur -c "SELECT version();"
```

### Error: UUID Extension Not Available

Jika mendapat error tentang `gen_random_uuid()`:
```bash
psql -h 13.212.194.164 -p 2314 -U postgres -d cobafitur -c "CREATE EXTENSION IF NOT EXISTS pgcrypto;"
```

### Error: Permission Denied

Pastikan user `postgres` memiliki permission untuk membuat tabel:
```sql
-- Connect as superuser jika perlu
GRANT ALL PRIVILEGES ON DATABASE cobafitur TO postgres;
GRANT ALL PRIVILEGES ON SCHEMA public TO postgres;
```

## Struktur Tabel Users yang Dibuat

```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    password_hash VARCHAR(255),              -- NULL untuk OAuth users
    oauth_provider VARCHAR(50),              -- 'google', 'github', etc
    oauth_id VARCHAR(255),                   -- OAuth provider's user ID
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Indexes
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_oauth ON users(oauth_provider, oauth_id) 
    WHERE oauth_provider IS NOT NULL;

-- Constraint: User harus punya password ATAU OAuth
ALTER TABLE users ADD CONSTRAINT check_auth_method 
    CHECK (
        (password_hash IS NOT NULL AND oauth_provider IS NULL AND oauth_id IS NULL) OR
        (password_hash IS NULL AND oauth_provider IS NOT NULL AND oauth_id IS NOT NULL)
    );
```

## Catatan Penting Google OAuth

Pastikan di Google Cloud Console, Authorized redirect URIs sudah disesuaikan:
- Development: `http://localhost:8000/api/auth/google/callback`
- Production: `http://your-domain.com/api/auth/google/callback`

**PENTING**: URL redirect harus PERSIS sama, termasuk:
- Protocol (http/https)
- Domain/IP
- Port
- Path

## Next Steps

Setelah migrations berhasil:

1. ✅ Jalankan server: `cargo run`
2. ✅ Server akan listen di: `0.0.0.0:8000`
3. ✅ Akses UI:
   - Registration: `http://localhost:8000/static/register.html`
   - Login: `http://localhost:8000/static/login.html`
   - Dashboard: `http://localhost:8000/static/dashboard.html`

4. ✅ Test API endpoints:
   - POST `/api/auth/register` - Email registration
   - POST `/api/auth/login` - Email login
   - GET `/api/auth/google` - Google OAuth
   - GET `/api/auth/google/callback` - OAuth callback

## Status Saat Ini

- ✅ Database: Remote PostgreSQL di 13.212.194.164:2314
- ✅ Database Name: cobafitur
- ✅ Server Port: 8000
- ✅ Google OAuth: Configured
- ✅ JWT: 1200 seconds (20 minutes)
- ✅ Migrations: Ready to run

Jalankan: `diesel migration run` untuk membuat tabel users!
