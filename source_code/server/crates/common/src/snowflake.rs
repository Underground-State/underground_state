//! Snowflake ID generator singleton

use std::sync::atomic::{AtomicU16, AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

const EPOCH: u64 = 1704067200000; // 2024-01-01 00:00:00 UTC
const SEQUENCE_BITS: u8 = 12;
const MAX_SEQUENCE: u16 = (1 << SEQUENCE_BITS) - 1;

pub struct SnowflakeGenerator {
    worker_id: u16,
    sequence: AtomicU16,
    last_timestamp: AtomicU64,
}

impl SnowflakeGenerator {
    pub const fn new(worker_id: u16) -> Self {
        Self {
            worker_id,
            sequence: AtomicU16::new(0),
            last_timestamp: AtomicU64::new(0),
        }
    }

    pub fn generate(&self) -> i64 {
        let mut timestamp = self.current_timestamp();
        let last_ts = self.last_timestamp.load(Ordering::Acquire);

        if timestamp == last_ts {
            let seq = self.sequence.fetch_add(1, Ordering::SeqCst);
            if seq > MAX_SEQUENCE {
                // Wait for next millisecond
                while timestamp <= last_ts {
                    timestamp = self.current_timestamp();
                }
                self.sequence.store(0, Ordering::Release);
            }
        } else {
            self.sequence.store(0, Ordering::Release);
        }

        self.last_timestamp.store(timestamp, Ordering::Release);
        let sequence = self.sequence.load(Ordering::Acquire);

        (((timestamp - EPOCH) as i64) << 22)
            | ((self.worker_id as i64) << 12)
            | (sequence as i64)
    }

    fn current_timestamp(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }
}

// Global generator instance
static GENERATOR: SnowflakeGenerator = SnowflakeGenerator::new(1);

/// Generate a new snowflake ID
pub fn generate_id() -> i64 {
    GENERATOR.generate()
}
