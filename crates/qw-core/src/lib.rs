//! QuietWatch portable core: shared configuration, stream types, and units.
//!
//! QuietWatch levels movie and TV audio so loud action and quiet dialogue sit
//! at a chosen listening loudness. This crate holds the types every other crate
//! shares; DSP lives in `qw-dsp` and device I/O lives in `qw-audio`.

pub mod config;
pub mod error;
pub mod types;
pub mod units;

pub use config::{
    CompressorConfig, DialogueConfig, GateConfig, LimiterConfig, ProcessorConfig, SessionConfig,
};
pub use error::QuietWatchError;
pub use types::{
    InterleavedBuffer, StreamConfig, DEFAULT_BUFFER_FRAMES, DEFAULT_CHANNELS, DEFAULT_SAMPLE_RATE,
};
pub use units::{db_to_linear, linear_to_db};

pub type Result<T> = std::result::Result<T, QuietWatchError>;
