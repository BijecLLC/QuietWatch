use qw_core::LimiterConfig;

/// Brickwall limiter. Passthrough until lookahead peak control is filled in.
#[derive(Clone, Debug)]
pub struct Limiter {
    config: LimiterConfig,
}

impl Limiter {
    pub fn new(config: LimiterConfig) -> Self {
        Self { config }
    }

    pub fn process(&mut self, samples: &mut [f32]) {
        let _ = (samples, &self.config);
    }

    pub fn config(&self) -> &LimiterConfig {
        &self.config
    }
}
