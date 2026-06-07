use image::{
    imageops::{crop, thumbnail, unsharpen},
    ImageBuffer, Rgb,
};

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
) -> Vec<u8> {
    let mut image: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::from_raw(src_width, src_height, raw)
            .expect("raw buffer dimensions must match width * height * 3");

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

    image
        .into_raw()
        .into_iter()
        .map(|v| gamma_lut[v as usize])
        .collect()
}
