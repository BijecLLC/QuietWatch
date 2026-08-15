//! Platform-agnostic audio capture and playback for QuietWatch.
//!
//! Platform crates (`qw-macos`, `qw-windows`, `qw-linux`) implement [`AudioBackend`].
//! [`NullBackend`] is available everywhere for tests and dry runs.

mod backend;
mod device;
mod null;

pub use backend::{AudioBackend, AudioSession, ProcessCallback};
pub use device::{AudioDevice, DeviceKind};
pub use null::{NullBackend, NullSession};

pub fn platform_backend_name() -> &'static str {
    if cfg!(target_os = "macos") {
        "coreaudio"
    } else if cfg!(target_os = "windows") {
        "wasapi"
    } else if cfg!(target_os = "linux") {
        "pipewire"
    } else {
        "unsupported"
    }
}
