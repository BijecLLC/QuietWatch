use qw_core::CompressorConfig;

/// Downward compressor. Passthrough until the gain computer is filled in.
#[derive(Clone, Debug)]
pub struct Compressor {
    config: CompressorConfig,
}

impl Compressor {
    pub fn new(config: CompressorConfig) -> Self {
        Self { config }
    }

    pub fn process(&mut self, samples: &mut [f32]) {
        let _ = (samples, &self.config);
    }

    pub fn config(&self) -> &CompressorConfig {
        &self.config
    }
}
