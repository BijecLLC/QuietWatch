/// How many channels the stream has, and what they mean.
///
/// Used for loudness channel weights and for filling in [`AudioFormat::channels`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChannelLayout {
    /// One channel.
    Mono,
    /// Two channels (left, right).
    Stereo,
    /// Six channels: L, R, C, LFE, Ls, Rs.
    Surround51,
    /// Eight channels: L, R, C, LFE, plus four surrounds.
    Surround71,
    /// Channel count is known but the speaker map is not.
    Unknown(usize),
}

/// Channel-count lookup for each layout variant.
impl ChannelLayout {
    /// Number of channels this layout occupies.
    pub fn channel_count(self) -> usize {
        match self {
            Self::Mono => 1,
            Self::Stereo => 2,
            Self::Surround51 => 6,
            Self::Surround71 => 8,
            Self::Unknown(count) => count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ChannelLayout;

    #[test]
    fn known_layouts_have_fixed_counts() {
        assert_eq!(ChannelLayout::Mono.channel_count(), 1);
        assert_eq!(ChannelLayout::Stereo.channel_count(), 2);
        assert_eq!(ChannelLayout::Surround51.channel_count(), 6);
        assert_eq!(ChannelLayout::Surround71.channel_count(), 8);
    }
}
