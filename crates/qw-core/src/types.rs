pub const DEFAULT_SAMPLE_RATE: u32 = 48_000;
pub const DEFAULT_CHANNELS: u16 = 2;
pub const DEFAULT_BUFFER_FRAMES: u32 = 512;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StreamConfig {
    pub sample_rate: u32,
    pub channels: u16,
    pub buffer_frames: u32,
}

impl Default for StreamConfig {
    fn default() -> Self {
        Self {
            sample_rate: DEFAULT_SAMPLE_RATE,
            channels: DEFAULT_CHANNELS,
            buffer_frames: DEFAULT_BUFFER_FRAMES,
        }
    }
}

impl StreamConfig {
    pub fn frame_count(self, interleaved_len: usize) -> usize {
        let channels = self.channels.max(1) as usize;
        interleaved_len / channels
    }
}

/// Interleaved `f32` samples in −1.0…1.0, channel-major within each frame.
#[derive(Clone, Copy, Debug)]
pub struct InterleavedBuffer<'a> {
    pub samples: &'a [f32],
    pub channels: u16,
}

impl<'a> InterleavedBuffer<'a> {
    pub fn frame_count(self) -> usize {
        let channels = self.channels.max(1) as usize;
        self.samples.len() / channels
    }
}
