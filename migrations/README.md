# Database Migrations

This directory contains all Diesel database migrations for the AuthSession project.

## Prerequisites

Before running migrations, ensure you have:
1. PostgreSQL installed and running
2. Database created: `authsession`
3. Diesel CLI installed: `cargo install diesel_cli --no-default-features --features postgres`

## Setup Database

### 1. Database Already Configured

Your database is already set up:
- **Host**: 13.212.194.164:2314 (Remote PostgreSQL)
- **Database**: cobafitur
- **User**: postgres
- **Password**: password123

If you need to connect manually:

```bash
# Connect to remote PostgreSQL
psql -h 13.212.194.164 -p 2314 -U postgres -d cobafitur

# Enter password when prompted: password123
```

### 2. Configure Database URL

Your `.env` file already contains the database URL:

```bash
APP_DATABASE__URL=postgres://postgres:password123@13.212.194.164:2314/cobafitur
```

Set the `DATABASE_URL` environment variable for Diesel CLI using your existing config:

```bash
# Windows PowerShell
$env:DATABASE_URL="postgres://postgres:password123@13.212.194.164:2314/cobafitur"

# Windows CMD
set DATABASE_URL=postgres://postgres:password123@13.212.194.164:2314/cobafitur

# Linux/Mac
export DATABASE_URL=postgres://postgres:password123@13.212.194.164:2314/cobafitur
```

**Note**: Your database is hosted remotely at `13.212.194.164:2314`

## Running Migrations

### Run all pending migrations
```bash
diesel migration run
```

### Revert the last migration
```bash
diesel migration revert
```

### Redo the last migration (revert + run)
```bash
diesel migration redo
```

### Check migration status
```bash
diesel migration list
```

## Migration Files

### 2025-10-07-111620-0000_create_users_table

Creates the `users` table with the following schema:

**Columns:**
- `id` (UUID, Primary Key) - Unique user identifier
- `email` (VARCHAR(255), NOT NULL, UNIQUE) - User's email address
- `name` (VARCHAR(255), NOT NULL) - User's full name
- `password_hash` (VARCHAR(255), NULLABLE) - Hashed password for email/password auth
- `oauth_provider` (VARCHAR(50), NULLABLE) - OAuth provider name (e.g., "google")
- `oauth_id` (VARCHAR(255), NULLABLE) - OAuth provider's user ID
- `created_at` (TIMESTAMP, NOT NULL) - Record creation timestamp
- `updated_at` (TIMESTAMP, NOT NULL) - Record last update timestamp

**Indexes:**
- `idx_users_email` - Index on email for faster lookups
- `idx_users_oauth` - Composite index on (oauth_provider, oauth_id) for OAuth lookups

**Constraints:**
- `check_auth_method` - Ensures either password_hash OR (oauth_provider + oauth_id) is present

This design supports two authentication methods:
1. **Email/Password**: `password_hash` is set, OAuth fields are NULL
2. **OAuth (Google)**: `oauth_provider` and `oauth_id` are set, `password_hash` is NULL

## Creating New Migrations

To create a new migration:

```bash
diesel migration generate migration_name
```

This will create a new directory in `migrations/` with two files:
- `up.sql` - SQL to apply the migration
- `down.sql` - SQL to revert the migration

## Updating Schema

After running migrations, update the Rust schema:

```bash
diesel print-schema > src/schema.rs
```

This generates the Diesel schema that matches your database structure.

## Troubleshooting

### Connection Failed

If you get connection errors:
1. Check PostgreSQL is running: `pg_isready`
2. Verify database exists: `psql -U postgres -l | grep authsession`
3. Test connection: `psql -U authuser -d authsession`

### Migration Already Applied

If you need to reset migrations:
```bash
# Revert all migrations
diesel migration revert --all

# Or manually reset
psql -U authuser -d authsession -c "DROP TABLE IF EXISTS __diesel_schema_migrations, users;"

# Then run migrations again
diesel migration run
```

### UUID Extension Not Available

If you get errors about `gen_random_uuid()`:
```bash
psql -U authuser -d authsession -c "CREATE EXTENSION IF NOT EXISTS pgcrypto;"
```

## Schema Updates

When modifying the `users` table:
1. Create a new migration: `diesel migration generate update_users_table`
2. Write SQL in `up.sql` and `down.sql`
3. Test: `diesel migration run`
4. If issues occur: `diesel migration revert`
5. Update schema: `diesel print-schema > src/schema.rs`
6. Update Rust models in `src/models/user.rs`
