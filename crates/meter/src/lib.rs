//! Peak, RMS, true-peak, crest factor, and LUFS metering.
//!
//! Call [`measure`] once per audio block. Sample peak, RMS, and crest are
//! per-block. True-peak and LUFS keep running state on [`MeterState`].

mod loudness;
mod peak;
mod rms;
mod state;
mod true_peak;

pub use state::MeterState;

use audio_types::AudioBuffer;

/// One snapshot of every meter for the current block.
///
/// Use `short_term_lufs` plus `peak_dbfs` to tell quiet dialogue from a loud
/// explosion. Unfilled LUFS windows and digital silence report `-inf`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AudioMetrics {
    /// Highest absolute sample in this block, in dBFS.
    pub peak_dbfs: f32,
    /// Highest reconstructed inter-sample peak, in dBTP.
    pub true_peak_dbtp: f32,
    /// RMS of this block, in dBFS.
    pub rms_dbfs: f32,
    /// K-weighted loudness over the last 400 ms, in LUFS.
    pub momentary_lufs: f32,
    /// K-weighted loudness over the last 3 s, in LUFS.
    pub short_term_lufs: f32,
    /// `peak_dbfs - rms_dbfs`; how peaky the block is. Silence is 0, not NaN.
    pub crest_factor_db: f32,
}

/// Runs every meter on `buffer` and returns a combined snapshot.
///
/// Updates true-peak and LUFS state in `state`. Peak, RMS, and crest do not
/// need that state.
pub fn measure(state: &mut MeterState, buffer: &AudioBuffer<'_>) -> AudioMetrics {
    let peak_dbfs = peak::peak_dbfs(buffer);
    let rms_dbfs = rms::rms_dbfs(buffer);
    loudness::process(&mut state.loudness, buffer);
    AudioMetrics {
        peak_dbfs,
        true_peak_dbtp: true_peak::true_peak_dbtp(&mut state.true_peak, buffer),
        rms_dbfs,
        momentary_lufs: loudness::momentary_lufs(&state.loudness),
        short_term_lufs: loudness::short_term_lufs(&state.loudness),
        crest_factor_db: crest_factor_db(peak_dbfs, rms_dbfs),
    }
}

/// Peak minus RMS in dB. Returns 0 when either value is non-finite (silence).
fn crest_factor_db(peak_dbfs: f32, rms_dbfs: f32) -> f32 {
    if peak_dbfs.is_finite() && rms_dbfs.is_finite() {
        peak_dbfs - rms_dbfs
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::{measure, MeterState};
    use audio_types::{AudioBuffer, AudioFormat, ChannelLayout};

    /// Builds a default 48 kHz meter, wraps `samples`, and calls [`measure`].
    fn metrics(samples: &mut [f32], channels: usize) -> super::AudioMetrics {
        let format = AudioFormat::new(48_000, ChannelLayout::Unknown(channels));
        let mut state = MeterState::new(format);
        let buffer = AudioBuffer::interleaved(samples, channels).expect("layout");
        measure(&mut state, &buffer)
    }

    #[test]
    fn sine_has_about_three_db_crest() {
        let mut samples = [0.0_f32; 256];
        for (index, sample) in samples.iter_mut().enumerate() {
            let phase = index as f32 * std::f32::consts::TAU / 256.0;
            *sample = phase.sin();
        }
        let measured = metrics(&mut samples, 1);
        assert!(
            (measured.crest_factor_db - 3.0103).abs() < 0.05,
            "crest {}",
            measured.crest_factor_db
        );
    }

    #[test]
    fn dc_has_zero_crest() {
        let mut samples = [1.0_f32; 8];
        let measured = metrics(&mut samples, 2);
        assert!(measured.crest_factor_db.abs() < 1.0e-6);
    }

    #[test]
    fn silence_crest_is_zero_not_nan() {
        let mut samples = [0.0_f32; 8];
        let measured = metrics(&mut samples, 2);
        assert_eq!(measured.crest_factor_db, 0.0);
    }
}
