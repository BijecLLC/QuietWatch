use audio_types::AudioBuffer;

use crate::MeterState;

pub fn true_peak_dbtp(_state: &mut MeterState, _buffer: &AudioBuffer<'_>) -> f32 {
    f32::NEG_INFINITY
}
