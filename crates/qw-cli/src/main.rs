use std::env;
use std::process::ExitCode;

use qw_audio::{AudioBackend, NullBackend};
use qw_core::{ProcessorConfig, SessionConfig};
use qw_dsp::Processor;

fn native_backend_name() -> &'static str {
    #[cfg(target_os = "macos")]
    {
        qw_macos::CoreAudioBackend::new().name()
    }
    #[cfg(target_os = "linux")]
    {
        qw_linux::PipeWireBackend::new().name()
    }
    #[cfg(target_os = "windows")]
    {
        qw_windows::WasapiBackend::new().name()
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        NullBackend.name()
    }
}

fn print_usage() {
    eprintln!(
        "\
QuietWatch — keep movie dialogue audible without blasting the loud parts

Usage:
  quietwatch              Show status and default leveling config
  quietwatch --help       Show this help
  quietwatch --devices    List devices from the null backend (hardware later)

The DSP pipeline is portable Rust. Device I/O is provided by:
  macOS    CoreAudio  (qw-macos)
  Windows  WASAPI     (qw-windows)
  Linux    PipeWire   (qw-linux)
"
    );
}

fn print_status() {
    let config = ProcessorConfig::default();
    let session = SessionConfig::default();
    let processor = Processor::new(config.clone(), session.stream);

    println!("QuietWatch");
    println!("  platform backend : {}", native_backend_name());
    println!(
        "  stream           : {} Hz, {} ch, {} frames",
        processor.stream_config().sample_rate,
        processor.stream_config().channels,
        processor.stream_config().buffer_frames
    );
    println!(
        "  target loudness  : {:.1} LUFS",
        config.target_loudness_lufs
    );
    println!(
        "  gain range       : {:+.1} … {:+.1} dB",
        config.min_gain_db, config.max_gain_db
    );
    println!("  lookahead        : {:.1} ms", config.lookahead_ms);
    println!("  compressor ratio : {}:1", config.compressor.ratio);
    println!("  limiter ceiling  : {:.1} dB", config.limiter.ceiling_db);
    println!("  gate threshold   : {:.1} dB", config.gate.threshold_db);
    println!(
        "  dialogue boost   : {:.1} dB (enabled={})",
        config.dialogue.speech_boost_db, config.dialogue.enabled
    );
    println!();
    println!("Live audio I/O is not wired yet. DSP stages are stubs besides RMS metering and gain suggestion.");
}

fn print_devices() -> qw_core::Result<()> {
    let backend = NullBackend;
    println!("Devices via `{}` backend:", backend.name());
    for device in backend.list_devices()? {
        let default = if device.is_default { " (default)" } else { "" };
        println!(
            "  [{:?}] {} — {}{default}",
            device.kind, device.id, device.name
        );
    }
    Ok(())
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.iter().map(String::as_str).collect::<Vec<_>>().as_slice() {
        [] => {
            print_status();
            ExitCode::SUCCESS
        }
        ["--help"] | ["-h"] => {
            print_usage();
            ExitCode::SUCCESS
        }
        ["--devices"] => match print_devices() {
            Ok(()) => ExitCode::SUCCESS,
            Err(error) => {
                eprintln!("{error}");
                ExitCode::FAILURE
            }
        },
        other => {
            eprintln!("unknown arguments: {}", other.join(" "));
            print_usage();
            ExitCode::FAILURE
        }
    }
}
