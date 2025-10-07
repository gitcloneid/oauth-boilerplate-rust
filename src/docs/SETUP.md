# AuthSession - Setup Guide

## Overview
This project implements a Rust authentication system with both email/password registration and Google OAuth login using Axum framework.

## Features
- Email/Password Registration & Login
- Google OAuth 2.0 Login
- JWT Token Authentication
- Password hashing with Argon2
- In-memory user storage (can be replaced with PostgreSQL)
- Beautiful HTML registration/login pages

## Prerequisites
- Rust (latest stable version)
- Google Cloud Platform account for OAuth credentials

## Setting Up Google OAuth

### 1. Create Google OAuth Credentials

1. Go to [Google Cloud Console](https://console.cloud.google.com/)
2. Create a new project or select existing one
3. Navigate to "APIs & Services" > "Credentials"
4. Click "Create Credentials" > "OAuth 2.0 Client ID"
5. Configure OAuth consent screen if not done already
6. Select "Web application" as application type
7. Add authorized redirect URIs:
   - `http://127.0.0.1:3000/api/auth/google/callback`
   - `http://localhost:3000/api/auth/google/callback`
8. Save and copy your `Client ID` and `Client Secret`

### 2. Configure Environment

Create a `.env` file in the project root (copy from `.env.example`):

```bash
APP_SERVER__HOST=127.0.0.1
APP_SERVER__PORT=3000
APP_DATABASE__URL=postgres://user:password@localhost/authsession
APP_DATABASE__MAX_CONNECTIONS=10
APP_JWT__SECRET=your-secret-key-here-change-this-in-production
APP_JWT__EXPIRATION=86400
APP_GOOGLE_OAUTH__CLIENT_ID=your-google-client-id-here
APP_GOOGLE_OAUTH__CLIENT_SECRET=your-google-client-secret-here
APP_GOOGLE_OAUTH__REDIRECT_URL=http://127.0.0.1:3000/api/auth/google/callback
APP_GOOGLE_OAUTH__AUTH_URL=https://accounts.google.com/o/oauth2/v2/auth
APP_GOOGLE_OAUTH__TOKEN_URL=https://oauth2.googleapis.com/token
```

**Important:** Replace `your-google-client-id-here` and `your-google-client-secret-here` with your actual Google OAuth credentials.

### 3. Update Default Configuration (Optional)

You can also update `config/default.toml` for default values:

```toml
[google_oauth]
client_id = "your-google-client-id"
client_secret = "your-google-client-secret"
redirect_url = "http://127.0.0.1:3000/api/auth/google/callback"
auth_url = "https://accounts.google.com/o/oauth2/v2/auth"
token_url = "https://oauth2.googleapis.com/token"
```

## Running the Application

### 1. Build the project

```bash
cargo build
```

### 2. Run the server

```bash
cargo run
```

The server will start on `http://127.0.0.1:3000`

### 3. Access the application

- Registration page: `http://127.0.0.1:3000/static/register.html`
- Login page: `http://127.0.0.1:3000/static/login.html`
- Dashboard (after login): `http://127.0.0.1:3000/static/dashboard.html`

## API Endpoints

### Authentication

#### Register with Email
```
POST /api/auth/register
Content-Type: application/json

{
  "name": "John Doe",
  "email": "john@example.com",
  "password": "password123"
}
```

Response:
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

#### Login with Email
```
POST /api/auth/login
Content-Type: application/json

{
  "email": "john@example.com",
  "password": "password123"
}
```

Response: Same as register

#### Google OAuth Login
```
GET /api/auth/google
```
Redirects to Google consent screen

#### Google OAuth Callback
```
GET /api/auth/google/callback?code=...&state=...
```
Automatically handled by Google, returns JWT token

#### Logout
```
POST /api/auth/logout
```

## Architecture

### Project Structure
```
src/
├── config.rs          # Configuration management
├── server.rs          # Server initialization
├── error.rs           # Custom error types
├── models/            # Data models
│   └── user.rs        # User model with OAuth support
├── handlers/          # Request handlers
│   └── auth_handler.rs # Auth logic (register, login, OAuth)
├── routes/            # Route definitions
│   ├── auth.rs        # Auth routes
│   ├── home.rs        # Home route
│   └── profile.rs     # Profile routes
├── db/                # Database layer
│   ├── pool.rs        # Connection pool
│   └── store.rs       # In-memory user store
├── middleware/        # Middleware
│   └── auth_middleware.rs
├── utils/             # Utilities
│   ├── hashing.rs     # Password hashing with Argon2
│   └── jwt.rs         # JWT token generation/verification
└── static/            # Frontend HTML pages
    ├── register.html
    ├── login.html
    └── dashboard.html
```

### User Model
```rust
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub password_hash: Option<String>,      // None for OAuth users
    pub oauth_provider: Option<String>,     // "google" for OAuth users
    pub oauth_id: Option<String>,           // Google user ID
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
```

## Security Notes

1. **JWT Secret**: Change `APP_JWT__SECRET` to a strong random string in production
2. **HTTPS**: Use HTTPS in production, update redirect URLs accordingly
3. **CORS**: Currently set to permissive, configure properly for production
4. **Password Requirements**: Consider adding password strength validation
5. **Rate Limiting**: Add rate limiting to prevent brute force attacks
6. **Database**: Replace in-memory storage with PostgreSQL for production

## Next Steps

1. **Database Migration**: Create PostgreSQL migration for users table
2. **Refresh Tokens**: Implement refresh token mechanism
3. **Email Verification**: Add email verification flow
4. **Password Reset**: Implement forgot password functionality
5. **More OAuth Providers**: Add GitHub, Facebook, etc.
6. **Protected Routes**: Add middleware for JWT verification on protected routes

## Troubleshooting

### OAuth Redirect Mismatch
- Ensure redirect URI in Google Console matches exactly: `http://127.0.0.1:3000/api/auth/google/callback`
- Don't use `localhost` if you configured `127.0.0.1` or vice versa

### Compilation Errors
- Run `cargo clean` and `cargo build` again
- Ensure Rust toolchain is up to date: `rustup update`

### Port Already in Use
- Change `APP_SERVER__PORT` to a different port

## License
This project is for educational purposes.
