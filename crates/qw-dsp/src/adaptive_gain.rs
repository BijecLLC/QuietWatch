use qw_core::ProcessorConfig;

/// Chooses a make-up / cut toward the target loudness. Does not yet apply it.
#[derive(Clone, Debug)]
pub struct AdaptiveGain {
    target_loudness_lufs: f32,
    max_gain_db: f32,
    min_gain_db: f32,
}

impl AdaptiveGain {
    pub fn new(config: &ProcessorConfig) -> Self {
        Self {
            target_loudness_lufs: config.target_loudness_lufs,
            max_gain_db: config.max_gain_db,
            min_gain_db: config.min_gain_db,
        }
    }

    pub fn suggested_gain_db(&self, measured_loudness_db: f32) -> f32 {
        let delta = self.target_loudness_lufs - measured_loudness_db;
        delta.clamp(self.min_gain_db, self.max_gain_db)
    }

    pub fn process(&self, samples: &mut [f32], measured_loudness_db: f32) -> f32 {
        let _ = samples;
        self.suggested_gain_db(measured_loudness_db)
    }
}

#[cfg(test)]
mod tests {
    use super::AdaptiveGain;
    use qw_core::ProcessorConfig;

    #[test]
    fn quiet_material_suggests_a_boost() {
        let gain = AdaptiveGain::new(&ProcessorConfig::default());
        let suggested = gain.suggested_gain_db(-36.0);
        assert!(suggested > 0.0);
        assert!(suggested <= 12.0);
    }

    #[test]
    fn loud_material_suggests_a_cut() {
        let gain = AdaptiveGain::new(&ProcessorConfig::default());
        let suggested = gain.suggested_gain_db(-6.0);
        assert!(suggested < 0.0);
    }
}
