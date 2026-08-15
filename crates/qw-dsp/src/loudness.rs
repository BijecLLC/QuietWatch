use qw_core::linear_to_db;

/// Short-window RMS meter. A BS.1770 LUFS meter replaces this later.
#[derive(Clone, Debug)]
pub struct LoudnessMeter {
    channels: u16,
}

impl LoudnessMeter {
    pub fn new(channels: u16) -> Self {
        Self {
            channels: channels.max(1),
        }
    }

    /// Returns RMS of the interleaved buffer in dBFS.
    pub fn measure(&self, samples: &[f32]) -> f32 {
        if samples.is_empty() {
            return linear_to_db(0.0);
        }

        let mut sum_squares = 0.0_f32;
        for sample in samples {
            sum_squares += *sample * *sample;
        }
        let mean_square = sum_squares / samples.len() as f32;
        linear_to_db(mean_square.sqrt())
    }

    pub fn channels(&self) -> u16 {
        self.channels
    }
}

#[cfg(test)]
mod tests {
    use super::LoudnessMeter;

    #[test]
    fn silence_is_below_gate_territory() {
        let meter = LoudnessMeter::new(2);
        let samples = [0.0_f32; 256];
        assert!(meter.measure(&samples) < -90.0);
    }

    #[test]
    fn full_scale_sine_is_near_minus_three() {
        let meter = LoudnessMeter::new(1);
        let mut samples = [0.0_f32; 256];
        for (index, sample) in samples.iter_mut().enumerate() {
            let phase = index as f32 * std::f32::consts::TAU / 256.0;
            *sample = phase.sin();
        }
        let measured = meter.measure(&samples);
        assert!((measured - (-3.0)).abs() < 0.2, "measured {measured}");
    }
}
