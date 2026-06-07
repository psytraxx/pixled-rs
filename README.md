# pixled-rs

Captures a webcam feed and streams it live to a 64×64 LED matrix display using the [DDP protocol](http://www.3waylabs.com/ddp/).

Tested with the [HD-WF2](https://hansonelectronics.com.au/product/hd-wf2-64x64-led-matrix-controller/) 64×64 LED matrix controller.

## How it works

1. Opens the first available V4L2 webcam
2. Crops each frame to a square and scales it down to 64×64
3. Applies an unsharp mask and gamma correction
4. Sends the pixel data over UDP via DDP to the LED matrix

## Configuration

Edit the constants at the top of `src/main.rs`:

| Constant | Default | Description |
|---|---|---|
| `DDP_HOST` | `192.168.178.69:4048` | IP address and port of the DDP display |
| `OUTPUT_SIZE` | `64` | Output resolution sent to the display (pixels) |
| `GAMMA` | `2.0` | Gamma correction exponent applied to each pixel |
| `FRAME_INTERVAL_MS` | `10` | Polling interval per frame loop iteration (ms) |

## Build and Run

```bash
cargo build --release
cargo run --release
```

Press any key to stop the stream.

## Project structure

- `src/main.rs` — entry point and main loop
- `src/camera.rs` — V4L2 camera discovery and frame capture
- `src/image_processing.rs` — crop, scale, sharpen, and gamma pipeline
- `src/led.rs` — DDP connection and pixel send

## Development

- Add a changelog entry to `CHANGELOG.md` after making changes.
- Run before committing:
  ```bash
  cargo fmt --all
  cargo clippy --all-targets --all-features -- -D warnings
  ```
