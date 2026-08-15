use audio_types::AudioBuffer;

use crate::MeterState;

pub fn momentary_lufs(_state: &mut MeterState, _buffer: &AudioBuffer<'_>) -> f32 {
    f32::NEG_INFINITY
}

pub fn short_term_lufs(_state: &mut MeterState, _buffer: &AudioBuffer<'_>) -> f32 {
    f32::NEG_INFINITY
}
