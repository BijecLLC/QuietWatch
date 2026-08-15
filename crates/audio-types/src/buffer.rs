#[derive(Debug)]
pub struct AudioBuffer<'a> {
    pub samples: &'a mut [f32],
    pub frames: usize,
    pub channels: usize,
}

impl<'a> AudioBuffer<'a> {
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

    pub fn as_slice(&self) -> &[f32] {
        self.samples
    }

    pub fn as_mut_slice(&mut self) -> &mut [f32] {
        self.samples
    }
}
