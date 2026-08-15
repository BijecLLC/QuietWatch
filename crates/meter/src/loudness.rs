use audio_types::{AudioBuffer, AudioFormat, ChannelLayout};

/// Momentary loudness window (BS.1771).
const MOMENTARY_MS: u32 = 400;
/// Short-term loudness window (BS.1771).
const SHORT_TERM_MS: u32 = 3000;
/// Offset in the BS.1770 loudness equation: `-0.691 + 10 log10(mean square)`.
const LUFS_OFFSET: f64 = -0.691;
/// BS.1770 weight for surround channels (~+1.5 dB).
const SURROUND_WEIGHT: f32 = 1.41;

/// One Direct Form II biquad (K-weighting stage).
#[derive(Clone, Debug)]
struct Biquad {
    b0: f64,
    b1: f64,
    b2: f64,
    a1: f64,
    a2: f64,
    z1: f64,
    z2: f64,
}

/// Sample-by-sample filtering for one K-weighting stage.
impl Biquad {
    /// Filters one sample and updates the delay state.
    fn process(&mut self, input: f64) -> f64 {
        let delay = input - self.a1 * self.z1 - self.a2 * self.z2;
        let output = self.b0 * delay + self.b1 * self.z1 + self.b2 * self.z2;
        self.z2 = self.z1;
        self.z1 = delay;
        output
    }
}

/// Fixed-length ring of per-frame K-weighted energies, with a running sum.
#[derive(Clone, Debug)]
struct EnergyWindow {
    samples: Vec<f32>,
    index: usize,
    filled: usize,
    sum: f64,
}

/// Ring-buffer push/mean for one LUFS window.
impl EnergyWindow {
    /// Allocates a ring of `length` frames (at least 1).
    fn new(length: usize) -> Self {
        Self {
            samples: vec![0.0; length.max(1)],
            index: 0,
            filled: 0,
            sum: 0.0,
        }
    }

    /// Pushes one frame's energy, dropping the oldest once the window is full.
    fn push(&mut self, energy: f32) {
        if self.filled == self.samples.len() {
            self.sum -= self.samples[self.index] as f64;
        } else {
            self.filled += 1;
        }
        self.samples[self.index] = energy;
        self.sum += energy as f64;
        self.index += 1;
        if self.index == self.samples.len() {
            self.index = 0;
        }
    }

    /// Mean energy, or `None` until the window has been filled once.
    fn mean(&self) -> Option<f64> {
        if self.filled < self.samples.len() {
            None
        } else {
            Some(self.sum / self.samples.len() as f64)
        }
    }
}

/// K-weighting filters plus 400 ms and 3 s energy windows for one stream.
#[derive(Clone, Debug)]
pub(crate) struct LoudnessState {
    format: AudioFormat,
    shelves: Vec<Biquad>,
    highpasses: Vec<Biquad>,
    momentary: EnergyWindow,
    short_term: EnergyWindow,
}

/// Construction and channel-count repair for the LUFS meter.
impl LoudnessState {
    /// Builds per-channel K-filters and empty loudness windows for `format`.
    pub(crate) fn new(format: AudioFormat) -> Self {
        let channels = format.channels.max(1);
        let (shelf, highpass) = k_weighting_filters(format.sample_rate);
        Self {
            format,
            shelves: vec![shelf.clone(); channels],
            highpasses: vec![highpass.clone(); channels],
            momentary: EnergyWindow::new(window_frames(format.sample_rate, MOMENTARY_MS)),
            short_term: EnergyWindow::new(window_frames(format.sample_rate, SHORT_TERM_MS)),
        }
    }

    /// Recreates filters if the buffer's channel count does not match.
    fn ensure_channels(&mut self, channels: usize) {
        let channels = channels.max(1);
        if self.shelves.len() == channels {
            return;
        }
        let (shelf, highpass) = k_weighting_filters(self.format.sample_rate);
        self.shelves = vec![shelf.clone(); channels];
        self.highpasses = vec![highpass.clone(); channels];
    }
}

/// K-weights `buffer` and pushes each frame's energy into both LUFS windows.
pub fn process(state: &mut LoudnessState, buffer: &AudioBuffer<'_>) {
    if buffer.channels == 0 || buffer.as_slice().is_empty() {
        return;
    }
    state.ensure_channels(buffer.channels);

    let samples = buffer.as_slice();
    let channels = buffer.channels;
    for frame in 0..buffer.frames {
        let mut energy = 0.0_f32;
        for channel in 0..channels {
            let input = samples[frame * channels + channel] as f64;
            let weighted = state.highpasses[channel].process(state.shelves[channel].process(input));
            let channel_energy = (weighted * weighted) as f32;
            energy += channel_weight(state.format.layout, channel) * channel_energy;
        }
        state.momentary.push(energy);
        state.short_term.push(energy);
    }
}

/// Momentary loudness in LUFS, or `-inf` until 400 ms of audio has been seen.
pub fn momentary_lufs(state: &LoudnessState) -> f32 {
    energy_to_lufs(state.momentary.mean())
}

/// Short-term loudness in LUFS, or `-inf` until 3 s of audio has been seen.
pub fn short_term_lufs(state: &LoudnessState) -> f32 {
    energy_to_lufs(state.short_term.mean())
}

/// Converts a mean-square energy to LUFS. Zero or missing energy is `-inf`.
fn energy_to_lufs(mean_square: Option<f64>) -> f32 {
    match mean_square {
        Some(mean) if mean > 0.0 => (LUFS_OFFSET + 10.0 * mean.log10()) as f32,
        _ => f32::NEG_INFINITY,
    }
}

/// How many frames sit in a window of `window_ms` at `sample_rate`.
fn window_frames(sample_rate: u32, window_ms: u32) -> usize {
    let sample_rate = sample_rate.max(1) as u64;
    ((sample_rate * u64::from(window_ms)) / 1000).max(1) as usize
}

/// BS.1770 channel gain. LFE (index 3 on 5.1/7.1) is left out of the sum.
fn channel_weight(layout: ChannelLayout, channel: usize) -> f32 {
    match layout {
        ChannelLayout::Mono | ChannelLayout::Stereo | ChannelLayout::Unknown(_) => 1.0,
        ChannelLayout::Surround51 => match channel {
            0 | 1 | 2 => 1.0,
            3 => 0.0,
            4 | 5 => SURROUND_WEIGHT,
            _ => 1.0,
        },
        ChannelLayout::Surround71 => match channel {
            0 | 1 | 2 => 1.0,
            3 => 0.0,
            4 | 5 | 6 | 7 => SURROUND_WEIGHT,
            _ => 1.0,
        },
    }
}

/// BS.1770 K-weighting pair: high shelf then high-pass, at `sample_rate`.
fn k_weighting_filters(sample_rate: u32) -> (Biquad, Biquad) {
    let sample_rate = sample_rate.max(1) as f64;
    (high_shelf(sample_rate), highpass(sample_rate))
}

/// High-frequency shelf (~+4 dB) used as the first K-weighting stage.
fn high_shelf(sample_rate: f64) -> Biquad {
    let f0 = 1681.974_450_955_533;
    let gain_db = 3.999_843_853_973_347;
    let quality = 0.707_175_236_955_419_6;
    let k = (std::f64::consts::PI * f0 / sample_rate).tan();
    let v_h = 10.0_f64.powf(gain_db / 20.0);
    let v_b = v_h.powf(0.499_666_774_154_541_6);
    let a0 = 1.0 + k / quality + k * k;
    Biquad {
        b0: (v_h + v_b * k / quality + k * k) / a0,
        b1: 2.0 * (k * k - v_h) / a0,
        b2: (v_h - v_b * k / quality + k * k) / a0,
        a1: 2.0 * (k * k - 1.0) / a0,
        a2: (1.0 - k / quality + k * k) / a0,
        z1: 0.0,
        z2: 0.0,
    }
}

/// High-pass (~38 Hz) used as the second K-weighting stage.
fn highpass(sample_rate: f64) -> Biquad {
    let f0 = 38.135_470_876_024_44;
    let quality = 0.500_327_037_323_877_3;
    let k = (std::f64::consts::PI * f0 / sample_rate).tan();
    let a0 = 1.0 + k / quality + k * k;
    Biquad {
        b0: 1.0,
        b1: -2.0,
        b2: 1.0,
        a1: 2.0 * (k * k - 1.0) / a0,
        a2: (1.0 - k / quality + k * k) / a0,
        z1: 0.0,
        z2: 0.0,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        momentary_lufs, process, short_term_lufs, window_frames, LoudnessState, MOMENTARY_MS,
        SHORT_TERM_MS,
    };
    use audio_types::{AudioBuffer, AudioFormat, ChannelLayout};

    const SAMPLE_RATE: u32 = 48_000;

    /// Pushes `frames` of a constant mono value through the loudness meter.
    fn feed_constant(state: &mut LoudnessState, frames: usize, value: f32) {
        let mut samples = vec![value; frames];
        let buffer = AudioBuffer::interleaved(&mut samples, 1).expect("layout");
        process(state, &buffer);
    }

    /// Pushes `frames` of a 1 kHz full-scale mono sine through the loudness meter.
    fn feed_sine(state: &mut LoudnessState, frames: usize) {
        let mut samples = vec![0.0_f32; frames];
        for (index, sample) in samples.iter_mut().enumerate() {
            let phase = index as f32 * 1000.0 * std::f32::consts::TAU / SAMPLE_RATE as f32;
            *sample = phase.sin();
        }
        let buffer = AudioBuffer::interleaved(&mut samples, 1).expect("layout");
        process(state, &buffer);
    }

    #[test]
    fn windows_match_bs1771_durations() {
        assert_eq!(window_frames(SAMPLE_RATE, MOMENTARY_MS), 19_200);
        assert_eq!(window_frames(SAMPLE_RATE, SHORT_TERM_MS), 144_000);
    }

    #[test]
    fn silence_stays_undefined_until_the_window_fills() {
        let mut state = LoudnessState::new(AudioFormat::new(SAMPLE_RATE, ChannelLayout::Mono));
        feed_constant(&mut state, 1_000, 0.0);
        assert_eq!(momentary_lufs(&state), f32::NEG_INFINITY);
        assert_eq!(short_term_lufs(&state), f32::NEG_INFINITY);
    }

    #[test]
    fn filled_silence_is_negative_infinity() {
        let mut state = LoudnessState::new(AudioFormat::new(SAMPLE_RATE, ChannelLayout::Mono));
        feed_constant(&mut state, window_frames(SAMPLE_RATE, MOMENTARY_MS), 0.0);
        assert_eq!(momentary_lufs(&state), f32::NEG_INFINITY);
    }

    #[test]
    fn mono_1khz_sine_momentary_is_near_minus_three_point_seven() {
        let mut state = LoudnessState::new(AudioFormat::new(SAMPLE_RATE, ChannelLayout::Mono));
        feed_sine(&mut state, window_frames(SAMPLE_RATE, MOMENTARY_MS));
        let measured = momentary_lufs(&state);
        assert!(
            (measured - (-3.7)).abs() < 1.5,
            "momentary {measured}"
        );
        assert_eq!(short_term_lufs(&state), f32::NEG_INFINITY);
    }

    #[test]
    fn short_term_tracks_momentary_after_three_seconds() {
        let mut state = LoudnessState::new(AudioFormat::new(SAMPLE_RATE, ChannelLayout::Mono));
        feed_sine(&mut state, window_frames(SAMPLE_RATE, SHORT_TERM_MS));
        let momentary = momentary_lufs(&state);
        let short_term = short_term_lufs(&state);
        assert!(momentary.is_finite(), "momentary {momentary}");
        assert!(
            (short_term - momentary).abs() < 0.2,
            "short-term {short_term} momentary {momentary}"
        );
    }
}
