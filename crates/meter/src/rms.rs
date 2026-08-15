use audio_types::AudioBuffer;

/// RMS of this block in dBFS.
///
/// Mean-square of every sample, then `20 * log10`. Silence is `-inf`; DC at
/// full scale is `0`; a full-scale sine is about `-3`. This is “how loud is
/// this chunk” without LUFS windows or K-weighting.
pub fn rms_dbfs(buffer: &AudioBuffer<'_>) -> f32 {
    let samples = buffer.as_slice();
    if samples.is_empty() {
        return f32::NEG_INFINITY;
    }

    let mut sum_squares = 0.0_f32;
    for sample in samples {
        sum_squares += *sample * *sample;
    }
    let rms = (sum_squares / samples.len() as f32).sqrt();
    if rms == 0.0 {
        f32::NEG_INFINITY
    } else {
        20.0 * rms.log10()
    }
}

#[cfg(test)]
mod tests {
    use super::rms_dbfs;
    use audio_types::AudioBuffer;

    /// Wraps `samples` as an [`AudioBuffer`] and returns RMS.
    fn measure(samples: &mut [f32], channels: usize) -> f32 {
        let buffer = AudioBuffer::interleaved(samples, channels).expect("layout");
        rms_dbfs(&buffer)
    }

    #[test]
    fn silence_is_negative_infinity() {
        let mut samples = [0.0_f32; 8];
        assert_eq!(measure(&mut samples, 2), f32::NEG_INFINITY);
    }

    #[test]
    fn dc_full_scale_is_zero_dbfs() {
        let mut samples = [1.0_f32, -1.0, 1.0, -1.0];
        let measured = measure(&mut samples, 2);
        assert!(measured.abs() < 1.0e-6, "measured {measured}");
    }

    #[test]
    fn full_scale_sine_is_near_minus_three() {
        let mut samples = [0.0_f32; 256];
        for (index, sample) in samples.iter_mut().enumerate() {
            let phase = index as f32 * std::f32::consts::TAU / 256.0;
            *sample = phase.sin();
        }
        let measured = measure(&mut samples, 1);
        assert!((measured - (-3.0103)).abs() < 0.05, "measured {measured}");
    }
}
