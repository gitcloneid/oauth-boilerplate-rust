use std::sync::Arc;
use dashmap::DashMap;
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct RateLimiter {
    // Track login attempts per IP
    ip_attempts: Arc<DashMap<String, Vec<Instant>>>,
    // Track login attempts per email
    email_attempts: Arc<DashMap<String, Vec<Instant>>>,
    
    // Configuration
    max_attempts_per_ip: usize,
    max_attempts_per_email: usize,
    window_duration: Duration,
}

impl RateLimiter {
    pub fn new(
        max_attempts_per_ip: usize,
        max_attempts_per_email: usize,
        window_seconds: u64,
    ) -> Self {
        Self {
            ip_attempts: Arc::new(DashMap::new()),
            email_attempts: Arc::new(DashMap::new()),
            max_attempts_per_ip,
            max_attempts_per_email,
            window_duration: Duration::from_secs(window_seconds),
        }
    }

    pub fn check_ip_limit(&self, ip: &str) -> Result<(), String> {
        let now = Instant::now();
        
        let mut entry = self.ip_attempts.entry(ip.to_string()).or_insert_with(Vec::new);
        
        // Remove old attempts outside the window
        entry.retain(|&time| now.duration_since(time) < self.window_duration);
        
        if entry.len() >= self.max_attempts_per_ip {
            let oldest = entry.first().unwrap();
            let wait_time = self.window_duration.saturating_sub(now.duration_since(*oldest));
            return Err(format!(
                "Too many login attempts from this IP. Try again in {} seconds",
                wait_time.as_secs()
            ));
        }
        
        entry.push(now);
        Ok(())
    }

    pub fn check_email_limit(&self, email: &str) -> Result<(), String> {
        let now = Instant::now();
        
        let mut entry = self.email_attempts.entry(email.to_string()).or_insert_with(Vec::new);
        
        // Remove old attempts outside the window
        entry.retain(|&time| now.duration_since(time) < self.window_duration);
        
        if entry.len() >= self.max_attempts_per_email {
            let oldest = entry.first().unwrap();
            let wait_time = self.window_duration.saturating_sub(now.duration_since(*oldest));
            return Err(format!(
                "Too many failed login attempts for this account. Try again in {} seconds",
                wait_time.as_secs()
            ));
        }
        
        entry.push(now);
        Ok(())
    }

    pub fn reset_email_limit(&self, email: &str) {
        self.email_attempts.remove(email);
    }

    pub fn record_failed_attempt(&self, _email: &str) {
        // This is already done in check_email_limit
        // But we keep this for explicit failed attempt tracking
    }

    // Cleanup old entries periodically
    pub fn cleanup(&self) {
        let now = Instant::now();
        
        self.ip_attempts.retain(|_, attempts| {
            attempts.retain(|&time| now.duration_since(time) < self.window_duration);
            !attempts.is_empty()
        });
        
        self.email_attempts.retain(|_, attempts| {
            attempts.retain(|&time| now.duration_since(time) < self.window_duration);
            !attempts.is_empty()
        });
    }
}


