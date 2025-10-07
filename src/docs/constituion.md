my_backend/
├── Cargo.toml
├── Cargo.lock
├── src/
│   ├── main.rs           // Entry point aplikasi
│   ├── lib.rs            // Root library, re-export modul, public API
│   ├── config.rs         // Konfigurasi (env, setup)
│   ├── server.rs         // Inisialisasi dan menjalankan server
│   ├── error.rs          // Modul error handling custom
|   |-- docs/             // Dokumentasi API Swagger/Docs
|   |-- static/           // Untuk menyimpan file/gambar static(Jika Ada)
│   ├── routes/           // Routing utama dan endpoint grouping
│   │     ├── mod.rs
│   │     ├── auth.rs
│   │     ├── home.rs
│   │     └── profile.rs
│   ├── handlers/         // Implementasi logic endpoint
│   │     ├── mod.rs
│   │     ├── auth_handler.rs
│   │     └── user_handler.rs
│   ├── models/           // Struct entitas/database & DTO
│   │     ├── mod.rs
│   │     ├── user.rs
│   │     └── profile.rs
│   ├── db/               // Code koneksi, migrasi, helper DB
│   │     ├── mod.rs
│   │     └── pool.rs
│   ├── middleware/       // Middleware khusus (auth, logging, dll)
│   │     ├── mod.rs
│   │     └── auth_middleware.rs
│   └── utils/            // Helper fungsi umum (validasi, dsb)
│         ├── mod.rs
│         └── hashing.rs

tests/                   // Integration test
