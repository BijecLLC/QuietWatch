use crate::types::{DEFAULT_BUFFER_FRAMES, DEFAULT_CHANNELS, DEFAULT_SAMPLE_RATE};

/// End-to-end leveling settings. DSP stages read the nested blocks that apply
/// to them; unused fields stay here so a single config object can be saved later.
#[derive(Clone, Debug, PartialEq)]
pub struct ProcessorConfig {
    /// Target program loudness in LUFS (negative, e.g. -18 for night listening).
    pub target_loudness_lufs: f32,
    /// Ceiling on boost applied to quiet dialogue.
    pub max_gain_db: f32,
    /// Floor on cut applied to loud action.
    pub min_gain_db: f32,
    pub compressor: CompressorConfig,
    pub limiter: LimiterConfig,
    pub gate: GateConfig,
    /// How far ahead the detector may look, in milliseconds.
    pub lookahead_ms: f32,
    pub dialogue: DialogueConfig,
}

impl Default for ProcessorConfig {
    fn default() -> Self {
        Self {
            target_loudness_lufs: -18.0,
            max_gain_db: 12.0,
            min_gain_db: -24.0,
            compressor: CompressorConfig::default(),
            limiter: LimiterConfig::default(),
            gate: GateConfig::default(),
            lookahead_ms: 10.0,
            dialogue: DialogueConfig::default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CompressorConfig {
    pub threshold_db: f32,
    pub ratio: f32,
    pub attack_ms: f32,
    pub release_ms: f32,
    pub makeup_gain_db: f32,
}

impl Default for CompressorConfig {
    fn default() -> Self {
        Self {
            threshold_db: -24.0,
            ratio: 4.0,
            attack_ms: 15.0,
            release_ms: 150.0,
            makeup_gain_db: 0.0,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LimiterConfig {
    pub ceiling_db: f32,
    pub release_ms: f32,
}

impl Default for LimiterConfig {
    fn default() -> Self {
        Self {
            ceiling_db: -1.0,
            release_ms: 50.0,
        }
    }
}

/// Silence gate: do not boost true quiet (pauses, black frames, room tone).
#[derive(Clone, Debug, PartialEq)]
pub struct GateConfig {
    pub threshold_db: f32,
    pub attack_ms: f32,
    pub hold_ms: f32,
    pub release_ms: f32,
}

impl Default for GateConfig {
    fn default() -> Self {
        Self {
            threshold_db: -50.0,
            attack_ms: 5.0,
            hold_ms: 80.0,
            release_ms: 120.0,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DialogueConfig {
    pub enabled: bool,
    /// Extra gain reserved for speech-like material, in dB.
    pub speech_boost_db: f32,
}

impl Default for DialogueConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            speech_boost_db: 3.0,
        }
    }
}

impl ProcessorConfig {
    pub fn with_target_loudness_lufs(mut self, target_loudness_lufs: f32) -> Self {
        self.target_loudness_lufs = target_loudness_lufs;
        self
    }
}

/// Capture and playback devices plus the stream format for a live session.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SessionConfig {
    pub capture_device_id: Option<String>,
    pub playback_device_id: Option<String>,
    pub stream: crate::types::StreamConfig,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            capture_device_id: None,
            playback_device_id: None,
            stream: crate::types::StreamConfig {
                sample_rate: DEFAULT_SAMPLE_RATE,
                channels: DEFAULT_CHANNELS,
                buffer_frames: DEFAULT_BUFFER_FRAMES,
            },
        }
    }
}
