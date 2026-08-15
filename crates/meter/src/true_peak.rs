use audio_types::AudioBuffer;

/// Interpolation factor for true-peak reconstruction.
const OVERSAMPLE: usize = 4;
/// Length of the 4× FIR at the oversampled rate.
const FIR_TAPS: usize = 48;
/// Taps stored per channel at the original sample rate (`FIR_TAPS / 4`).
const TAPS_PER_PHASE: usize = FIR_TAPS / OVERSAMPLE;

/// Per-stream memory for the 4× true-peak interpolator.
///
/// Holds FIR coefficients and a per-channel delay line so peaks that straddle
/// buffer edges are still measured.
#[derive(Clone, Debug)]
pub(crate) struct TruePeakState {
    /// Polyphase FIR coefficients, DC-normalized per phase.
    coeffs: [f32; FIR_TAPS],
    /// Newest-first delay: `delay[channel * TAPS_PER_PHASE + tap]`.
    delay: Vec<f32>,
    /// Channel count the delay line is sized for.
    channels: usize,
}

/// Delay-line setup for the true-peak interpolator.
impl TruePeakState {
    /// Builds an interpolator for `channels` (at least 1).
    pub(crate) fn new(channels: usize) -> Self {
        let channels = channels.max(1);
        Self {
            coeffs: interpolation_coefficients(),
            delay: vec![0.0; channels * TAPS_PER_PHASE],
            channels,
        }
    }

    /// Rebuilds the delay line if the incoming buffer's channel count changed.
    fn ensure_channels(&mut self, channels: usize) {
        let channels = channels.max(1);
        if self.channels == channels && self.delay.len() == channels * TAPS_PER_PHASE {
            return;
        }
        self.channels = channels;
        self.delay.clear();
        self.delay.resize(channels * TAPS_PER_PHASE, 0.0);
    }
}

/// True-peak of this block in dBTP, using 4× oversampling.
///
/// Sample-peak misses values that occur between samples (a limiter still has
/// to catch those). Silence is `-inf`. A step up from silence can read above
/// 0 dBTP because the reconstruction filter rings.
pub fn true_peak_dbtp(state: &mut TruePeakState, buffer: &AudioBuffer<'_>) -> f32 {
    if buffer.channels == 0 || buffer.as_slice().is_empty() {
        return f32::NEG_INFINITY;
    }
    state.ensure_channels(buffer.channels);

    let samples = buffer.as_slice();
    let channels = buffer.channels;
    let mut peak = 0.0_f32;

    for frame in 0..buffer.frames {
        for channel in 0..channels {
            let sample = samples[frame * channels + channel];
            let delay_offset = channel * TAPS_PER_PHASE;
            for tap in (1..TAPS_PER_PHASE).rev() {
                state.delay[delay_offset + tap] = state.delay[delay_offset + tap - 1];
            }
            state.delay[delay_offset] = sample;

            for phase in 0..OVERSAMPLE {
                let mut interpolated = 0.0_f32;
                for tap in 0..TAPS_PER_PHASE {
                    interpolated +=
                        state.delay[delay_offset + tap] * state.coeffs[phase + tap * OVERSAMPLE];
                }
                peak = peak.max(interpolated.abs());
            }
        }
    }

    if peak == 0.0 {
        f32::NEG_INFINITY
    } else {
        20.0 * peak.log10()
    }
}

/// Windowed-sinc 4× interpolator, each polyphase branch summing to 1 (unity DC).
fn interpolation_coefficients() -> [f32; FIR_TAPS] {
    let mut coeffs = [0.0_f32; FIR_TAPS];
    let oversample = OVERSAMPLE as f64;
    let n_minus_1 = (FIR_TAPS - 1) as f64;

    for (index, coeff) in coeffs.iter_mut().enumerate() {
        let x = index as f64 - n_minus_1 / 2.0;
        let sinc = if x.abs() < 1.0e-12 {
            1.0
        } else {
            let argument = std::f64::consts::PI * x / oversample;
            argument.sin() / argument
        };
        let window =
            0.54 - 0.46 * (2.0 * std::f64::consts::PI * index as f64 / n_minus_1).cos();
        *coeff = (sinc * window) as f32;
    }

    for phase in 0..OVERSAMPLE {
        let mut phase_sum = 0.0_f32;
        for tap in 0..TAPS_PER_PHASE {
            phase_sum += coeffs[phase + tap * OVERSAMPLE];
        }
        if phase_sum.abs() > 0.0 {
            for tap in 0..TAPS_PER_PHASE {
                coeffs[phase + tap * OVERSAMPLE] /= phase_sum;
            }
        }
    }

    coeffs
}

#[cfg(test)]
mod tests {
    use super::{true_peak_dbtp, TruePeakState};
    use crate::peak::peak_dbfs;
    use audio_types::AudioBuffer;

    /// Fresh interpolator, one block, returns dBTP.
    fn measure_true_peak(samples: &mut [f32], channels: usize) -> f32 {
        let mut state = TruePeakState::new(channels);
        let buffer = AudioBuffer::interleaved(samples, channels).expect("layout");
        true_peak_dbtp(&mut state, &buffer)
    }

    #[test]
    fn silence_is_negative_infinity() {
        let mut samples = [0.0_f32; 64];
        assert_eq!(measure_true_peak(&mut samples, 2), f32::NEG_INFINITY);
    }

    #[test]
    fn dc_full_scale_is_near_zero_dbtp() {
        let mut state = TruePeakState::new(1);
        let mut warmup = [1.0_f32; 64];
        {
            let buffer = AudioBuffer::interleaved(&mut warmup, 1).expect("layout");
            let _warmup = true_peak_dbtp(&mut state, &buffer);
        }
        let mut samples = [1.0_f32; 64];
        let buffer = AudioBuffer::interleaved(&mut samples, 1).expect("layout");
        let measured = true_peak_dbtp(&mut state, &buffer);
        assert!(measured.abs() < 0.05, "measured {measured}");
    }

    #[test]
    fn step_from_silence_can_exceed_zero_dbtp() {
        let mut samples = [1.0_f32; 64];
        let measured = measure_true_peak(&mut samples, 1);
        assert!(measured > 0.05, "measured {measured}");
    }

    #[test]
    fn low_frequency_sine_true_peak_tracks_sample_peak() {
        let mut samples = [0.0_f32; 256];
        for (index, sample) in samples.iter_mut().enumerate() {
            let phase = index as f32 * std::f32::consts::TAU / 48.0;
            *sample = phase.sin();
        }
        let sample_peak = {
            let buffer = AudioBuffer::interleaved(&mut samples, 1).expect("layout");
            peak_dbfs(&buffer)
        };
        let true_peak = measure_true_peak(&mut samples, 1);
        assert!(
            true_peak >= sample_peak - 0.2,
            "true-peak {true_peak} sample-peak {sample_peak}"
        );
    }
}
