use qw_core::{Result, SessionConfig, StreamConfig};

use crate::device::AudioDevice;

/// Called with each interleaved capture block. Mutate `samples` in place;
/// the backend plays the result.
pub type ProcessCallback = Box<dyn FnMut(&mut [f32]) + Send>;

pub trait AudioBackend {
    fn name(&self) -> &'static str;

    fn list_devices(&self) -> Result<Vec<AudioDevice>>;

    fn default_output(&self) -> Result<AudioDevice>;

    fn default_input(&self) -> Result<AudioDevice>;

    /// Opens a live capture → process → playback session.
    fn open_session(
        &self,
        config: &SessionConfig,
        process: ProcessCallback,
    ) -> Result<Box<dyn AudioSession>>;
}

pub trait AudioSession: Send {
    fn stream_config(&self) -> StreamConfig;

    fn start(&mut self) -> Result<()>;

    fn stop(&mut self) -> Result<()>;

    fn is_running(&self) -> bool;
}
