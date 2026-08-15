use crate::ChannelLayout;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AudioFormat {
    pub sample_rate: u32,
    pub channels: usize,
    pub layout: ChannelLayout,
}

impl AudioFormat {
    pub fn new(sample_rate: u32, layout: ChannelLayout) -> Self {
        Self {
            sample_rate,
            channels: layout.channel_count(),
            layout,
        }
    }
}
