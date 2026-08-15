use audio_types::AudioBuffer;

/// Sample peak of this block in dBFS.
///
/// This is the largest `|sample|` converted with `20 * log10`. Silence is
/// `-inf`; a full-scale sample is `0`. It does not see peaks that happen
/// between samples — that is true-peak.
pub fn peak_dbfs(buffer: &AudioBuffer<'_>) -> f32 {
    let mut peak = 0.0_f32;
    for sample in buffer.as_slice() {
        peak = peak.max(sample.abs());
    }
    if peak == 0.0 {
        f32::NEG_INFINITY
    } else {
        20.0 * peak.log10()
    }
}

#[cfg(test)]
mod tests {
    use super::peak_dbfs;
    use audio_types::AudioBuffer;

    /// Wraps `samples` as an [`AudioBuffer`] and returns sample peak.
    fn measure(samples: &mut [f32], channels: usize) -> f32 {
        let buffer = AudioBuffer::interleaved(samples, channels).expect("layout");
        peak_dbfs(&buffer)
    }

    #[test]
    fn silence_is_negative_infinity() {
        let mut samples = [0.0_f32; 8];
        assert_eq!(measure(&mut samples, 2), f32::NEG_INFINITY);
    }

    #[test]
    fn full_scale_is_zero_dbfs() {
        let mut samples = [0.0, 1.0, 0.0, -0.5];
        let measured = measure(&mut samples, 2);
        assert!(measured.abs() < 1.0e-6, "measured {measured}");
    }

    #[test]
    fn half_scale_is_minus_six() {
        let mut samples = [0.5_f32, -0.5];
        let measured = measure(&mut samples, 1);
        assert!((measured - (-6.0206)).abs() < 0.01, "measured {measured}");
    }
}
