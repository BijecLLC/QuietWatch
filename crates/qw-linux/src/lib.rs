//! PipeWire backend stub. A filter graph (capture → QuietWatch → playback)
//! will live here.

use qw_audio::{AudioBackend, AudioDevice, AudioSession, ProcessCallback};
use qw_core::{QuietWatchError, Result, SessionConfig, StreamConfig};

pub struct PipeWireBackend;

impl PipeWireBackend {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PipeWireBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioBackend for PipeWireBackend {
    fn name(&self) -> &'static str {
        "pipewire"
    }

    fn list_devices(&self) -> Result<Vec<AudioDevice>> {
        if cfg!(target_os = "linux") {
            Ok(Vec::new())
        } else {
            Err(QuietWatchError::UnsupportedPlatform("pipewire"))
        }
    }

    fn default_output(&self) -> Result<AudioDevice> {
        Err(QuietWatchError::DeviceNotFound("default-output".into()))
    }

    fn default_input(&self) -> Result<AudioDevice> {
        Err(QuietWatchError::DeviceNotFound("default-input".into()))
    }

    fn open_session(
        &self,
        _config: &SessionConfig,
        _process: ProcessCallback,
    ) -> Result<Box<dyn AudioSession>> {
        if cfg!(target_os = "linux") {
            Err(QuietWatchError::Backend(
                "PipeWire session is not wired yet".into(),
            ))
        } else {
            Err(QuietWatchError::UnsupportedPlatform("pipewire"))
        }
    }
}

pub struct PipeWireSession {
    stream: StreamConfig,
    running: bool,
}

impl PipeWireSession {
    pub fn new(stream: StreamConfig) -> Self {
        Self {
            stream,
            running: false,
        }
    }
}

impl AudioSession for PipeWireSession {
    fn stream_config(&self) -> StreamConfig {
        self.stream
    }

    fn start(&mut self) -> Result<()> {
        self.running = true;
        Ok(())
    }

    fn stop(&mut self) -> Result<()> {
        self.running = false;
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.running
    }
}
