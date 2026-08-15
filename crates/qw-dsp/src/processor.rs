use qw_core::{ProcessorConfig, StreamConfig};

use crate::{
    AdaptiveGain, Compressor, DialogueLogic, Limiter, Lookahead, LoudnessMeter, SilenceGate,
};

/// Runs the portable leveling chain over interleaved `f32` buffers.
pub struct Processor {
    stream: StreamConfig,
    loudness: LoudnessMeter,
    gate: SilenceGate,
    dialogue: DialogueLogic,
    adaptive_gain: AdaptiveGain,
    compressor: Compressor,
    limiter: Limiter,
    lookahead: Lookahead,
    last_loudness_db: f32,
    last_gain_db: f32,
    last_gate_open: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProcessorStatus {
    pub measured_loudness_db: f32,
    pub suggested_gain_db: f32,
    pub gate_open: bool,
}

impl Processor {
    pub fn new(config: ProcessorConfig, stream: StreamConfig) -> Self {
        Self {
            stream,
            loudness: LoudnessMeter::new(stream.channels),
            gate: SilenceGate::new(config.gate.clone()),
            dialogue: DialogueLogic::new(config.dialogue.clone()),
            adaptive_gain: AdaptiveGain::new(&config),
            compressor: Compressor::new(config.compressor.clone()),
            limiter: Limiter::new(config.limiter.clone()),
            lookahead: Lookahead::new(config.lookahead_ms, stream),
            last_loudness_db: f32::NEG_INFINITY,
            last_gain_db: 0.0,
            last_gate_open: true,
        }
    }

    pub fn stream_config(&self) -> StreamConfig {
        self.stream
    }

    pub fn status(&self) -> ProcessorStatus {
        ProcessorStatus {
            measured_loudness_db: self.last_loudness_db,
            suggested_gain_db: self.last_gain_db,
            gate_open: self.last_gate_open,
        }
    }

    /// Processes one interleaved block in place.
    ///
    /// Stages currently measure and decide gain but do not modify samples,
    /// except as each stub starts applying its algorithm.
    pub fn process_interleaved(&mut self, samples: &mut [f32]) -> ProcessorStatus {
        self.lookahead.process(samples);
        self.last_loudness_db = self.loudness.measure(samples);
        self.last_gate_open = self.gate.process(samples, self.last_loudness_db);
        self.dialogue.process(samples);

        let suggested = if self.last_gate_open {
            self.adaptive_gain.process(samples, self.last_loudness_db)
        } else {
            0.0
        };
        self.last_gain_db = suggested;

        self.compressor.process(samples);
        self.limiter.process(samples);

        self.status()
    }
}

#[cfg(test)]
mod tests {
    use super::Processor;
    use qw_core::{ProcessorConfig, StreamConfig};

    #[test]
    fn passthrough_leaves_samples_unchanged() {
        let mut processor = Processor::new(ProcessorConfig::default(), StreamConfig::default());
        let original = [0.25_f32, -0.5, 0.125, -0.125];
        let mut samples = original;
        processor.process_interleaved(&mut samples);
        assert_eq!(samples, original);
    }

    #[test]
    fn silence_closes_the_gate_and_holds_gain() {
        let mut processor = Processor::new(ProcessorConfig::default(), StreamConfig::default());
        let mut samples = [0.0_f32; 128];
        let status = processor.process_interleaved(&mut samples);
        assert!(!status.gate_open);
        assert_eq!(status.suggested_gain_db, 0.0);
    }
}
