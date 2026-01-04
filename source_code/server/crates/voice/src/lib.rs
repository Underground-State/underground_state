//! Voice module (WebRTC/mediasoup integration)
//!
//! Note: Full mediasoup integration requires additional setup.
//! This module provides the signaling infrastructure.

pub mod signaling;

pub use signaling::VoiceSignaling;
