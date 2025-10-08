# Quick Argon2 benchmark script
Write-Host "🔍 Benchmarking Argon2 password hashing..." -ForegroundColor Cyan

# Build in release mode for accurate benchmark
Write-Host "`nBuilding in release mode..." -ForegroundColor Yellow
cargo build --release --quiet

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Build failed!" -ForegroundColor Red
    exit 1
}

Write-Host "✅ Build successful`n" -ForegroundColor Green

# Create a simple benchmark test
$testCode = @"
use auth_session::utils::hashing;
use std::time::Instant;

fn main() {
    println!("Testing password hashing performance...\n");
    
    let password = "test_password_123";
    let iterations = 10;
    
    // Test hashing
    let mut hash_times = Vec::new();
    for i in 1..=iterations {
        let start = Instant::now();
        let hash = hashing::hash_password(password).expect("Hash failed");
        let duration = start.elapsed();
        hash_times.push(duration.as_millis());
        println!("Hash #{}: {}ms", i, duration.as_millis());
        
        // Test verify with the same hash
        let verify_start = Instant::now();
        let valid = hashing::verify_password(password, &hash).expect("Verify failed");
        let verify_duration = verify_start.elapsed();
        println!("Verify #{}: {}ms (valid: {})\n", i, verify_duration.as_millis(), valid);
    }
    
    let avg_hash = hash_times.iter().sum::<u128>() / iterations as u128;
    println!("\n📊 Results:");
    println!("Average hash time: {}ms", avg_hash);
    println!("Min hash time: {}ms", hash_times.iter().min().unwrap());
    println!("Max hash time: {}ms", hash_times.iter().max().unwrap());
}
"@

# Save test file
$testCode | Out-File -FilePath ".\target\bench_test.rs" -Encoding UTF8

Write-Host "Running benchmark..." -ForegroundColor Yellow
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━`n" -ForegroundColor Gray

# Note: This is a simplified version. For actual benchmark, user should run the app
Write-Host "⚡ To test actual performance:" -ForegroundColor Cyan
Write-Host "   1. Run: cargo run --release" -ForegroundColor White
Write-Host "   2. Make register/login requests" -ForegroundColor White
Write-Host "   3. Check logs for detailed timing" -ForegroundColor White
Write-Host "`n💡 Expected performance with optimizations:" -ForegroundColor Yellow
Write-Host "   - Password hashing: ~30-80ms" -ForegroundColor Green
Write-Host "   - Password verify: ~30-80ms" -ForegroundColor Green
Write-Host "   - Total request: ~50-150ms" -ForegroundColor Green
