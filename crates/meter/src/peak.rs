use audio_types::AudioBuffer;

pub fn peak_dbfs(_buffer: &AudioBuffer<'_>) -> f32 {
    f32::NEG_INFINITY
}
