use nokhwa::{
    pixel_format::RgbFormat,
    query,
    utils::{frame_formats, RequestedFormat, RequestedFormatType, Resolution},
    Camera,
};
use std::error::Error;

pub fn open_first_camera() -> Result<Camera, Box<dyn Error>> {
    let backend = nokhwa::native_api_backend().ok_or("No native camera backend available")?;
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
            Ok(camera) => {
                println!("Camera {} opened successfully", index);
                return Ok(camera);
            }
            Err(e) => println!("Error opening camera {}: {}", index, e),
        }
    }

    Err("No usable camera found".into())
}

pub fn print_compatible_formats(cam: &mut Camera) {
    for ffmt in frame_formats() {
        if let Ok(compatible) = cam.compatible_list_by_resolution(*ffmt) {
            println!("{ffmt}:");
            let mut formats: Vec<_> = compatible.into_iter().collect();
            formats.sort_by_key(|a| a.0);
            for (resolution, fps) in formats {
                println!(" - {resolution}: {fps:?}");
            }
        }
    }
}

pub fn capture_rgb_frame(cam: &mut Camera) -> Result<Vec<u8>, Box<dyn Error>> {
    let frame = cam.frame()?;
    let image = frame.decode_image::<RgbFormat>()?;
    Ok(image.into_raw())
}
