//! CoreAudio backend stub. Capture of system output and playback to the
//! selected device will live here.

use qw_audio::{AudioBackend, AudioDevice, AudioSession, ProcessCallback};
use qw_core::{QuietWatchError, Result, SessionConfig, StreamConfig};

pub struct CoreAudioBackend;

impl CoreAudioBackend {
    pub fn new() -> Self {
        Self
    }
}

impl Default for CoreAudioBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioBackend for CoreAudioBackend {
    fn name(&self) -> &'static str {
        "coreaudio"
    }

    fn list_devices(&self) -> Result<Vec<AudioDevice>> {
        if cfg!(target_os = "macos") {
            Ok(Vec::new())
        } else {
            Err(QuietWatchError::UnsupportedPlatform("coreaudio"))
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
        if cfg!(target_os = "macos") {
            Err(QuietWatchError::Backend(
                "CoreAudio session is not wired yet".into(),
            ))
        } else {
            Err(QuietWatchError::UnsupportedPlatform("coreaudio"))
        }
    }
}

/// Placeholder so the session type exists before HAL I/O is implemented.
pub struct CoreAudioSession {
    stream: StreamConfig,
    running: bool,
}

impl CoreAudioSession {
    pub fn new(stream: StreamConfig) -> Self {
        Self {
            stream,
            running: false,
        }
    }
}

impl AudioSession for CoreAudioSession {
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
