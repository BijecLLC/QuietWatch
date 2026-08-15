//! WASAPI backend stub. Shared-mode loopback capture and exclusive or shared
//! playback will live here.

use qw_audio::{AudioBackend, AudioDevice, AudioSession, ProcessCallback};
use qw_core::{QuietWatchError, Result, SessionConfig, StreamConfig};

pub struct WasapiBackend;

impl WasapiBackend {
    pub fn new() -> Self {
        Self
    }
}

impl Default for WasapiBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioBackend for WasapiBackend {
    fn name(&self) -> &'static str {
        "wasapi"
    }

    fn list_devices(&self) -> Result<Vec<AudioDevice>> {
        if cfg!(target_os = "windows") {
            Ok(Vec::new())
        } else {
            Err(QuietWatchError::UnsupportedPlatform("wasapi"))
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
        if cfg!(target_os = "windows") {
            Err(QuietWatchError::Backend(
                "WASAPI session is not wired yet".into(),
            ))
        } else {
            Err(QuietWatchError::UnsupportedPlatform("wasapi"))
        }
    }
}

pub struct WasapiSession {
    stream: StreamConfig,
    running: bool,
}

impl WasapiSession {
    pub fn new(stream: StreamConfig) -> Self {
        Self {
            stream,
            running: false,
        }
    }
}

impl AudioSession for WasapiSession {
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
