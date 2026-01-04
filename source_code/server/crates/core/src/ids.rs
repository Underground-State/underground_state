//! Snowflake ID generation and typed IDs

use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU16, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

/// Custom epoch: 2024-01-01 00:00:00 UTC
const EPOCH: u64 = 1704067200000;
const WORKER_ID_BITS: u8 = 10;
const SEQUENCE_BITS: u8 = 12;
const MAX_SEQUENCE: u16 = (1 << SEQUENCE_BITS) - 1;

/// Snowflake ID generator
pub struct SnowflakeGenerator {
    worker_id: u16,
    sequence: AtomicU16,
}

impl SnowflakeGenerator {
    pub fn new(worker_id: u16) -> Self {
        assert!(worker_id < (1 << WORKER_ID_BITS), "Worker ID too large");
        Self {
            worker_id,
            sequence: AtomicU16::new(0),
        }
    }

    pub fn generate(&self) -> i64 {
        let timestamp = self.current_timestamp();
        let sequence = self.sequence.fetch_add(1, Ordering::SeqCst) & MAX_SEQUENCE;

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

/// Extract timestamp from Snowflake ID
pub fn snowflake_timestamp(id: i64) -> u64 {
    ((id >> 22) as u64) + EPOCH
}

// Typed IDs for type safety
macro_rules! define_id {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(pub i64);

        impl $name {
            pub fn new(id: i64) -> Self {
                Self(id)
            }

            pub fn as_i64(&self) -> i64 {
                self.0
            }
        }

        impl From<i64> for $name {
            fn from(id: i64) -> Self {
                Self(id)
            }
        }

        impl From<$name> for i64 {
            fn from(id: $name) -> Self {
                id.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

define_id!(UserId);
define_id!(GuildId);
define_id!(ChannelId);
define_id!(MessageId);
define_id!(RoleId);
define_id!(DocumentId);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snowflake_generation() {
        let gen = SnowflakeGenerator::new(1);
        let id1 = gen.generate();
        let id2 = gen.generate();
        assert_ne!(id1, id2);
        assert!(id2 > id1);
    }

    #[test]
    fn test_typed_ids() {
        let user_id = UserId::new(123456789);
        assert_eq!(user_id.as_i64(), 123456789);
    }
}
