//! Foundational audio types. No device backends, no DSP, no platform APIs.

mod buffer;
mod format;
mod layout;

pub use buffer::AudioBuffer;
pub use format::AudioFormat;
pub use layout::ChannelLayout;
