use anyhow::{Context, Result};
use image::{
    imageops::{crop, thumbnail, unsharpen},
    ImageBuffer, Rgb,
};

type RgbImage = ImageBuffer<Rgb<u8>, Vec<u8>>;

pub fn build_gamma_lut(gamma: f32) -> Vec<u8> {
    (0..256)
        .map(|i| ((i as f32 / 255.0).powf(gamma) * 255.0) as u8)
        .collect()
}

pub fn process_frame(
    raw: Vec<u8>,
    src_width: u32,
    src_height: u32,
    output_size: u32,
    gamma_lut: &[u8],
) -> Result<(Vec<u8>, RgbImage)> {
    let mut image: RgbImage = ImageBuffer::from_raw(src_width, src_height, raw)
        .context("Frame buffer size does not match camera dimensions")?;

    let size = src_width.min(src_height);
    let image = crop(
        &mut image,
        (src_width - size) / 2,
        (src_height - size) / 2,
        size,
        size,
    )
    .to_image();

    let image = thumbnail(&image, output_size, output_size);
    let image = unsharpen(&image, 0.5, 5);

    let pixels: Vec<u8> = image.iter().map(|&v| gamma_lut[v as usize]).collect();
    let preview = ImageBuffer::from_raw(output_size, output_size, pixels.clone())
        .context("Failed to build preview image buffer")?;

    Ok((pixels, preview))
}
