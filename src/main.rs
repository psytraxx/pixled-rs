mod camera;
mod image_processing;
mod led;

use anyhow::Result;
use crossterm::event::{poll, read, Event};
use std::env;
use std::time::Duration;

const DDP_HOST: &str = "192.168.178.63:4048"; // matrix

//const DDP_HOST: &str = "192.168.178.69:4048";
const OUTPUT_SIZE: u32 = 64;
const GAMMA: f32 = 2.0;
const FRAME_INTERVAL_MS: u64 = 10;
const PREVIEW_PATH: &str = "webcam_capture.png";

fn main() -> Result<()> {
    let preview = env::args().any(|a| a == "--preview");

    let mut led = led::LedOutput::try_new(DDP_HOST)?;
    let gamma_lut = image_processing::build_gamma_lut(GAMMA);

    let mut cam = camera::open_first_camera()?;
    camera::print_compatible_formats(&mut cam);

    cam.open_stream()?;
    println!("Camera {} stream started", cam.index());
    println!("Camera format: {:?}", cam.camera_format());
    if preview {
        println!("Preview enabled — saving each frame to {PREVIEW_PATH}");
    }

    loop {
        if poll(Duration::from_millis(FRAME_INTERVAL_MS))? {
            if let Event::Key(_) = read()? {
                println!("Stopping camera stream...");
                break;
            }
        }

        let format = cam.camera_format();
        match camera::capture_rgb_frame(&mut cam) {
            Ok(raw) => {
                match image_processing::process_frame(
                    raw,
                    format.width(),
                    format.height(),
                    OUTPUT_SIZE,
                    &gamma_lut,
                ) {
                    Ok((pixels, img)) => {
                        led.send(&pixels)?;
                        if preview {
                            if let Err(e) = img.save(PREVIEW_PATH) {
                                println!("Error saving preview: {e}");
                            }
                        }
                    }
                    Err(e) => println!("Error processing frame: {e}"),
                }
            }
            Err(e) => println!("Error capturing frame: {e}"),
        }
    }

    if let Err(e) = cam.stop_stream() {
        println!("Error stopping stream: {e}");
    }
    println!("Stream stopped");

    Ok(())
}
