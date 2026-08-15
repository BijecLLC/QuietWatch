<p align="center">
  <img src="assets/brand/quietwatch-readme-banner.png" alt="QuietWatch logo" width="900">
</p>

# QuietWatch

[![license](https://img.shields.io/badge/license-GPL--3.0--or--later-blue)](LICENSE.md)

A small process that levels movie and TV audio so quiet dialogue and loud action sit at the same listening volume.

Lots of movies have loud fights and then whispered dialogue. You either keep the volume low and miss the talking, or turn it up and cannot watch in a quieter place. QuietWatch reads the output level and dynamically boosts the quiet parts and cuts the loud parts toward a target loudness you set.

**This is not a full DAW or equalizer.** QuietWatch does one job: keep program loudness in a comfortable band so you can listen at night without riding the volume knob.

### Principles

- **Small** - One job, done well. No feature creep.
- **Portable** - DSP is Rust and shared. Device I/O is isolated per platform.
- **Predictable** - You pick a target loudness. Everything moves toward that level.
- **Safe** - Silence is not boosted. Peaks are limited before they hit the speakers.

## How It Works

1. Capture system audio (loopback / virtual device, once the platform backends are wired)
2. Measure short-window loudness
3. Skip true silence (pauses, black frames, room tone)
4. Suggest or apply gain so quiet dialogue comes up and loud action comes down
5. Compress, limit, and play the result to your speakers or headphones

```
QuietWatch
                    │
        ┌───────────┴────────────┐
        │                        │
   Portable Core            Platform Layer
      Rust                        │
        │            ┌───────────┼───────────┐
        │            │           │           │
        │          macOS       Windows      Linux
        │        CoreAudio     WASAPI       PipeWire
        │
        ├── loudness measurement
        ├── adaptive gain
        ├── compressor
        ├── limiter
        ├── silence gating
        ├── lookahead
        └── dialogue logic
```

## Building

> **Requires:** Rust 1.74 or newer (`rustup`).

```sh
git clone <repo-url>
cd QuietWatch

cargo test --workspace
cargo run                  # CLI status
cargo run -- --devices     # stub device list
cargo run -p quietwatch-desktop
```

Output binaries are placed in `target/debug/` (or `target/release/` with `--release`).

| Crate | Role |
|-------|------|
| `qw-core` | Shared config, stream types, units |
| `qw-dsp` | Loudness, gain, compressor, limiter, gate, lookahead, dialogue |
| `qw-audio` | Device/session traits and a null backend for tests |
| `qw-macos` | CoreAudio backend |
| `qw-windows` | WASAPI backend |
| `qw-linux` | PipeWire backend |
| `qw-cli` | `quietwatch` command |
| `quietwatch-desktop` | Desktop UI stub |

Drop local listening fixtures in `test-audio/` (gitignored).

## Logo Assets

- [README banner](assets/brand/quietwatch-readme-banner.png)
- [Horizontal logo](assets/brand/quietwatch-logo.png)
- [App mark](assets/brand/quietwatch-mark.png)

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for architecture, coding standards, and PR guidelines.

## Contributors

- **Chris Perriello** - Creator, developer & maintainer

## License

GPL-3.0-or-later - See [LICENSE.md](LICENSE.md) for details.
