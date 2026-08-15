use crate::ChannelLayout;

/// Stream format: sample rate, channel count, and speaker layout.
///
/// This is what meters and later DSP stages need to size state. It does not
/// describe a device or an OS audio API.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AudioFormat {
    /// Samples per second (e.g. 48000).
    pub sample_rate: u32,
    /// Channel count; should match `layout.channel_count()`.
    pub channels: usize,
    /// Speaker map used for loudness weights and similar.
    pub layout: ChannelLayout,
}

/// Constructors for [`AudioFormat`].
impl AudioFormat {
    /// Builds a format and sets `channels` from the layout.
    pub fn new(sample_rate: u32, layout: ChannelLayout) -> Self {
        Self {
            sample_rate,
            channels: layout.channel_count(),
            layout,
        }
    }
}
