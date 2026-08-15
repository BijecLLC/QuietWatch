use qw_core::StreamConfig;

/// Delay line so detectors can see upcoming peaks. Identity until allocated.
#[derive(Clone, Debug)]
pub struct Lookahead {
    delay_ms: f32,
    sample_rate: u32,
}

impl Lookahead {
    pub fn new(delay_ms: f32, stream: StreamConfig) -> Self {
        Self {
            delay_ms,
            sample_rate: stream.sample_rate,
        }
    }

    pub fn delay_samples(&self) -> usize {
        let seconds = self.delay_ms.max(0.0) / 1000.0;
        (seconds * self.sample_rate as f32).round() as usize
    }

    pub fn process(&mut self, samples: &mut [f32]) {
        let _ = samples;
    }
}
