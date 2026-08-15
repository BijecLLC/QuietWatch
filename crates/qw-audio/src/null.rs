use qw_core::{QuietWatchError, Result, SessionConfig, StreamConfig};

use crate::backend::{AudioBackend, AudioSession, ProcessCallback};
use crate::device::{AudioDevice, DeviceKind};

/// In-process backend that never touches hardware.
pub struct NullBackend;

pub struct NullSession {
    stream: StreamConfig,
    running: bool,
    _process: ProcessCallback,
}

impl AudioBackend for NullBackend {
    fn name(&self) -> &'static str {
        "null"
    }

    fn list_devices(&self) -> Result<Vec<AudioDevice>> {
        Ok(vec![
            AudioDevice::new("null-in", "Null Input", DeviceKind::Input, true),
            AudioDevice::new("null-out", "Null Output", DeviceKind::Output, true),
            AudioDevice::new(
                "null-loopback",
                "Null Loopback",
                DeviceKind::Loopback,
                false,
            ),
        ])
    }

    fn default_output(&self) -> Result<AudioDevice> {
        self.list_devices()?
            .into_iter()
            .find(|device| device.kind == DeviceKind::Output)
            .ok_or_else(|| QuietWatchError::DeviceNotFound("null-out".into()))
    }

    fn default_input(&self) -> Result<AudioDevice> {
        self.list_devices()?
            .into_iter()
            .find(|device| device.kind == DeviceKind::Input)
            .ok_or_else(|| QuietWatchError::DeviceNotFound("null-in".into()))
    }

    fn open_session(
        &self,
        config: &SessionConfig,
        process: ProcessCallback,
    ) -> Result<Box<dyn AudioSession>> {
        Ok(Box::new(NullSession {
            stream: config.stream,
            running: false,
            _process: process,
        }))
    }
}

impl AudioSession for NullSession {
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

#[cfg(test)]
mod tests {
    use super::NullBackend;
    use crate::backend::AudioBackend;
    use qw_core::SessionConfig;

    #[test]
    fn lists_stub_devices_and_opens_a_session() {
        let backend = NullBackend;
        let devices = backend.list_devices().expect("devices");
        assert_eq!(devices.len(), 3);

        let mut session = backend
            .open_session(&SessionConfig::default(), Box::new(|_samples| {}))
            .expect("session");
        session.start().expect("start");
        assert!(session.is_running());
        session.stop().expect("stop");
        assert!(!session.is_running());
    }
}
