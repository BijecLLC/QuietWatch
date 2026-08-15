use audio_types::AudioFormat;

/// Running meter state (window history, true-peak oversampling, and so on).
#[derive(Clone, Debug)]
pub struct MeterState {
    pub format: AudioFormat,
}

impl MeterState {
    pub fn new(format: AudioFormat) -> Self {
        Self { format }
    }
}
