//! Portable DSP for QuietWatch.
//!
//! Pipeline order (stubs today, real algorithms later):
//! loudness → silence gate → dialogue → adaptive gain → compressor → limiter,
//! with a lookahead delay so detectors can see upcoming peaks.

mod adaptive_gain;
mod compressor;
mod dialogue;
mod gate;
mod limiter;
mod lookahead;
mod loudness;
mod processor;

pub use adaptive_gain::AdaptiveGain;
pub use compressor::Compressor;
pub use dialogue::{DialogueLogic, DialoguePresence};
pub use gate::SilenceGate;
pub use limiter::Limiter;
pub use lookahead::Lookahead;
pub use loudness::LoudnessMeter;
pub use processor::{Processor, ProcessorStatus};
