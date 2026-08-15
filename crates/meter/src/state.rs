use audio_types::AudioFormat;

use crate::loudness::LoudnessState;
use crate::true_peak::TruePeakState;

/// All running meter memory for one stream.
///
/// Create once per capture session and pass it to [`crate::measure`] on every
/// block. True-peak FIR delay and LUFS windows live here so they survive
/// across buffers.
#[derive(Clone, Debug)]
pub struct MeterState {
    /// Format this meter was built for (sample rate, channels, layout).
    pub format: AudioFormat,
    /// 4× interpolator delay lines for true-peak.
    pub(crate) true_peak: TruePeakState,
    /// K-weighting filters and 400 ms / 3 s energy windows.
    pub(crate) loudness: LoudnessState,
}

/// Construction of per-stream meter memory.
impl MeterState {
    /// Allocates true-peak and LUFS state for `format`.
    pub fn new(format: AudioFormat) -> Self {
        Self {
            format,
            true_peak: TruePeakState::new(format.channels),
            loudness: LoudnessState::new(format),
        }
    }
}
