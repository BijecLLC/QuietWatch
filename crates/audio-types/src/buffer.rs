/// One block of interleaved `f32` audio, borrowed from the caller.
///
/// Layout is frame-major: `samples[frame * channels + channel]`. Values are
/// nominally in −1.0…1.0. The slice is mutable so later processors can work
/// in place; meters only read it.
#[derive(Debug)]
pub struct AudioBuffer<'a> {
    /// Interleaved sample storage for this block.
    pub samples: &'a mut [f32],
    /// Number of frames (one sample per channel) in the block.
    pub frames: usize,
    /// Channels per frame.
    pub channels: usize,
}

/// Constructors and slice access for an interleaved block.
impl<'a> AudioBuffer<'a> {
    /// Wraps an interleaved slice if `samples.len()` divides evenly by `channels`.
    pub fn interleaved(samples: &'a mut [f32], channels: usize) -> Option<Self> {
        if channels == 0 || samples.len() % channels != 0 {
            return None;
        }
        let frames = samples.len() / channels;
        Some(Self {
            samples,
            frames,
            channels,
        })
    }

    /// Read-only view of the interleaved samples.
    pub fn as_slice(&self) -> &[f32] {
        self.samples
    }

    /// Mutable view of the interleaved samples.
    pub fn as_mut_slice(&mut self) -> &mut [f32] {
        self.samples
    }
}
