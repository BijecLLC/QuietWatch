# Contributing to QuietWatch

Thanks for your interest in contributing. This document covers the ground rules.

## Project Philosophy

QuietWatch is a **listening tool**, not a DAW, equalizer, or media player. It exists so you can watch a movie at a single comfortable loudness: quiet dialogue comes up, loud action comes down.

These principles guide every contribution:

- **Small** - Resist feature creep. Every addition should earn its place.
- **Portable** - Algorithms live in `qw-dsp`. Platform crates only talk to CoreAudio, WASAPI, or PipeWire.
- **Predictable** - The user sets a target loudness. Processing should move the program toward that target, not "improve" the mix.
- **Safe** - Do not boost silence. Do not let peaks clip. Prefer a closed gate over a noisy boost.

If a proposed change conflicts with any of these, it will be declined regardless of how well it's implemented.

## What We're Looking For

- Bug fixes with clear reproduction steps
- DSP improvements (loudness metering, gain smoothing, limiter, dialogue detection) with before/after measurements
- Platform I/O (CoreAudio, WASAPI, PipeWire capture and playback)
- Accessibility and latency improvements

## What We're Not Looking For

- Built-in video players, codec stacks, or subtitle renderers
- Full parametric EQs, surround virtualizers, or "audio enhancer" effect chains
- Heavy UI frameworks, web views, or Electron-style dependencies
- Features that duplicate what the OS volume mixer already does

## Architecture

```
QuietWatch/
├── Cargo.toml                 # Workspace
├── crates/
│   ├── qw-core/               # Config, errors, stream types, dB units
│   ├── qw-dsp/                # Portable processing chain
│   │   ├── loudness.rs        # Short-window loudness (RMS stub → LUFS later)
│   │   ├── adaptive_gain.rs   # Make-up / cut toward target loudness
│   │   ├── compressor.rs      # Downward compressor
│   │   ├── limiter.rs         # Peak ceiling
│   │   ├── gate.rs            # Silence gate (do not boost pauses)
│   │   ├── lookahead.rs       # Delay so detectors can see upcoming peaks
│   │   ├── dialogue.rs        # Speech-band / dialogue logic
│   │   └── processor.rs       # Chains the stages over interleaved f32
│   ├── qw-audio/              # AudioBackend / AudioSession traits, NullBackend
│   ├── qw-macos/              # CoreAudio
│   ├── qw-windows/            # WASAPI
│   ├── qw-linux/              # PipeWire
│   └── qw-cli/                # quietwatch binary
├── apps/
│   └── desktop/               # Desktop UI stub
└── test-audio/                # Local fixtures (not committed)
```

Live path once I/O is wired: capture → `Processor::process_interleaved` → playback.

## Development Setup

### Prerequisites

- **Rust stable** (1.74+) via [rustup](https://rustup.rs)
- A host with working system audio if you are testing a platform backend

```sh
git clone <repo-url>
cd QuietWatch
cargo test --workspace
```

### Building

```sh
cargo run                      # CLI
cargo run -- --devices
cargo run -p quietwatch-desktop
cargo test --workspace
```

### Code Style

- Run `cargo fmt` before committing.
- Run `cargo clippy --workspace --all-targets` and fix warnings.
- Prefer `?` over `unwrap()`. Do not silently discard errors with `let _ =` on fallible calls.
- Keep DSP in `qw-dsp`. Do not put CoreAudio / WASAPI / PipeWire types in the portable crates.
- No unnecessary abstractions. Three similar lines are better than a premature generic.

## Pull Request Process

1. **Open an issue first** for anything non-trivial. Discuss the approach before writing code.
2. **One concern per PR.** Don't bundle unrelated changes.
3. **Build must pass** on your platform (`cargo test --workspace`). Test another OS if you touch a platform crate.
4. **Write a clear PR description** - what changed, why, and how to verify.
5. **No force-pushes** after review has started.

## Commit Messages

Keep them short and descriptive. Present tense, imperative mood:

```
Add RMS loudness meter for short windows
Fix silence gate opening on denormal noise
Remove unused lookahead allocation
```

## Maintainers

- **Chris Perriello** - Creator, developer & maintainer

## License

By contributing, you agree that your contributions are licensed under the [GPL-3.0-or-later](LICENSE.md), the same license as the project.
