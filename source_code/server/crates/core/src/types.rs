//! Common types used across the application

use serde::{Deserialize, Serialize};

/// User status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserStatus {
    Online,
    Idle,
    Dnd,
    Offline,
}

impl Default for UserStatus {
    fn default() -> Self {
        Self::Offline
    }
}

/// KYC verification status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum KycStatus {
    None,
    Pending,
    Approved,
    Rejected,
}

impl Default for KycStatus {
    fn default() -> Self {
        Self::None
    }
}

/// Channel type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChannelType {
    Text = 0,
    Voice = 1,
    Dm = 2,
    GroupDm = 3,
    Category = 4,
}

impl From<i16> for ChannelType {
    fn from(value: i16) -> Self {
        match value {
            0 => Self::Text,
            1 => Self::Voice,
            2 => Self::Dm,
            3 => Self::GroupDm,
            4 => Self::Category,
            _ => Self::Text,
        }
    }
}

/// Message type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    Default = 0,
    Reply = 1,
    System = 2,
}

impl From<i16> for MessageType {
    fn from(value: i16) -> Self {
        match value {
            0 => Self::Default,
            1 => Self::Reply,
            2 => Self::System,
            _ => Self::Default,
        }
    }
}

/// Permission bitflags
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Permissions(pub i64);

impl Permissions {
    pub const VIEW_CHANNEL: i64 = 1 << 0;
    pub const SEND_MESSAGES: i64 = 1 << 1;
    pub const MANAGE_MESSAGES: i64 = 1 << 2;
    pub const MANAGE_CHANNEL: i64 = 1 << 3;
    pub const CONNECT: i64 = 1 << 4;
    pub const SPEAK: i64 = 1 << 5;
    pub const MUTE_MEMBERS: i64 = 1 << 6;
    pub const DEAFEN_MEMBERS: i64 = 1 << 7;
    pub const MANAGE_GUILD: i64 = 1 << 8;
    pub const KICK_MEMBERS: i64 = 1 << 9;
    pub const BAN_MEMBERS: i64 = 1 << 10;
    pub const ADMINISTRATOR: i64 = 1 << 11;

    pub fn has(&self, permission: i64) -> bool {
        (self.0 & permission) == permission || (self.0 & Self::ADMINISTRATOR) == Self::ADMINISTRATOR
    }

    pub fn add(&mut self, permission: i64) {
        self.0 |= permission;
    }

    pub fn remove(&mut self, permission: i64) {
        self.0 &= !permission;
    }
}

/// Pagination parameters
#[derive(Debug, Clone, Deserialize)]
pub struct Pagination {
    #[serde(default)]
    pub before: Option<i64>,
    #[serde(default)]
    pub after: Option<i64>,
    #[serde(default = "default_limit")]
    pub limit: u32,
}

fn default_limit() -> u32 {
    50
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            before: None,
            after: None,
            limit: 50,
        }
    }
}
