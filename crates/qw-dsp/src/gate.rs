use qw_core::GateConfig;

/// Holds gain at unity when the program is below the noise/silence floor.
#[derive(Clone, Debug)]
pub struct SilenceGate {
    config: GateConfig,
    open: bool,
}

impl SilenceGate {
    pub fn new(config: GateConfig) -> Self {
        Self {
            config,
            open: true,
        }
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    /// Returns whether boost is allowed for this block.
    pub fn process(&mut self, samples: &mut [f32], measured_loudness_db: f32) -> bool {
        let _ = samples;
        self.open = measured_loudness_db >= self.config.threshold_db;
        self.open
    }

    pub fn config(&self) -> &GateConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::SilenceGate;
    use qw_core::GateConfig;

    #[test]
    fn closes_on_silence() {
        let mut gate = SilenceGate::new(GateConfig::default());
        let mut samples = [0.0_f32; 32];
        assert!(!gate.process(&mut samples, -90.0));
    }

    #[test]
    fn stays_open_on_speech_level() {
        let mut gate = SilenceGate::new(GateConfig::default());
        let mut samples = [0.1_f32; 32];
        assert!(gate.process(&mut samples, -20.0));
    }
}
