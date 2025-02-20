use crossterm::event::{poll, read, Event};
use ddp_rs::{connection, protocol};
use image::imageops::{crop, thumbnail, unsharpen};
use nokhwa::{
    pixel_format::RgbFormat,
    query,
    utils::{frame_formats, RequestedFormat, RequestedFormatType, Resolution},
    Camera,
};
use std::{error::Error, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
    let mut conn = connection::DDPConnection::try_new(
        "192.168.178.63:4048",
        protocol::PixelConfig::default(),
        protocol::ID::Default,
        std::net::UdpSocket::bind("0.0.0.0:4048").unwrap(),
    )?;

    let gamma = 2.0;
    let gamma_lut: Vec<u8> = (0..256)
        .map(|i| ((i as f32 / 255.0).powf(gamma) * 255.0) as u8)
        .collect();

    if let Some(backend) = nokhwa::native_api_backend() {
        println!("Using native backend: {}", backend);
        let devices = query(backend)?;
        for device in devices {
            println!("-------------> Device: {}", device.description());
            let index = device.index();
            match Camera::new(
                index.clone(),
                RequestedFormat::new::<RgbFormat>(RequestedFormatType::HighestResolution(
                    Resolution::new(640, 480),
                )),
            ) {
                Ok(mut camera) => {
                    println!("Camera {} opened successfully", index);
                    camera_print_controls(&camera);
                    camera_compatible_formats(&mut camera);

                    match camera.open_stream() {
                        Ok(_) => {
                            println!("Camera {} stream started", index);

                            println!("Camera format: {:?}", camera.camera_format());
                            loop {
                                // Check for key press
                                if poll(Duration::from_millis(10))? {
                                    if let Event::Key(_) = read()? {
                                        println!("Stopping camera stream...");
                                        break;
                                    }
                                }

                                let frame = camera.frame();
                                match frame {
                                    Ok(frame) => {
                                        //println!("Frame captured successfully");
                                        //println!("Frame resolution: {:?}", frame.resolution());
                                        //println!("Frame format: {:?}", frame.source_frame_format());
                                        //println!("Frame data length: {}", frame.buffer().len());

                                        // Convert to RGB image
                                        let mut image = frame.decode_image::<RgbFormat>()?;

                                        // Crop to square
                                        let (width, height) = image.dimensions();
                                        let size = width.min(height);

                                        let image = crop(
                                            &mut image,
                                            (width - size) / 2,
                                            (height - size) / 2,
                                            size,
                                            size,
                                        )
                                        .to_image();

                                        let image = thumbnail(&image, 64, 64);

                                        let image = unsharpen(&image, 0.5, 5);

                                        let pixel_values: Vec<u8> = image
                                            .into_vec()
                                            .into_iter()
                                            .map(|v| gamma_lut[v as usize])
                                            .collect();

                                        conn.write(&pixel_values)?;

                                        // Save as PNG
                                        //image.save("webcam_capture.png")?;
                                    }
                                    Err(e) => {
                                        println!("Error capturing frame: {}", e);
                                    }
                                }

                                std::thread::sleep(std::time::Duration::from_millis(10));
                            }

                            camera.stop_stream().unwrap();
                            println!("Camera {} stream stopped", index);
                        }
                        Err(e) => {
                            println!("Error starting stream: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("Error opening camera {}: {}", index, e);
                }
            }
        }
    } else {
        println!("No native backend available, using dummy backend");
    }
    Ok(())
}

fn camera_print_controls(cam: &Camera) {
    let ctrls = cam.camera_controls().unwrap();
    let index = cam.index();
    println!("Controls for camera {index}");
    for ctrl in ctrls {
        println!("{ctrl}")
    }
}

fn camera_compatible_formats(cam: &mut Camera) {
    for ffmt in frame_formats() {
        if let Ok(compatible) = cam.compatible_list_by_resolution(*ffmt) {
            println!("{ffmt}:");
            let mut formats = Vec::new();
            for (resolution, fps) in compatible {
                formats.push((resolution, fps));
            }
            formats.sort_by(|a, b| a.0.cmp(&b.0));
            for fmt in formats {
                let (resolution, res) = fmt;
                println!(" - {resolution}: {res:?}")
            }
        }
    }
}
