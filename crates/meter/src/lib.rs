//! Metering layer. Measurements are stubs until the algorithms are filled in.

mod loudness;
mod peak;
mod rms;
mod state;
mod true_peak;

pub use state::MeterState;

use audio_types::AudioBuffer;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AudioMetrics {
    pub peak_dbfs: f32,
    pub true_peak_dbtp: f32,
    pub rms_dbfs: f32,
    pub momentary_lufs: f32,
    pub short_term_lufs: f32,
    pub crest_factor_db: f32,
}

pub fn measure(state: &mut MeterState, buffer: &AudioBuffer<'_>) -> AudioMetrics {
    let peak_dbfs = peak::peak_dbfs(buffer);
    let rms_dbfs = rms::rms_dbfs(buffer);
    AudioMetrics {
        peak_dbfs,
        true_peak_dbtp: true_peak::true_peak_dbtp(state, buffer),
        rms_dbfs,
        momentary_lufs: loudness::momentary_lufs(state, buffer),
        short_term_lufs: loudness::short_term_lufs(state, buffer),
        crest_factor_db: 0.0,
    }
}
