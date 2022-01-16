use image::{DynamicImage, GenericImageView};

pub fn save_text_to_alpha(image: &DynamicImage, cipher_message: &[u8]) -> image::RgbaImage {
    let (width, height) = image.dimensions();
    let mut scale_rgba = image.to_rgba8();

    let cipher_message_len = cipher_message.len();
    if cipher_message_len > 255 {
        panic!("Not support longer than 255 characters");
    }

    let mut i = 0;
    for y in 0..height {
        for x in 0..width {
            let alpha: u8;

            if i == 0 {
                alpha = cipher_message_len as u8;
            } else {
                alpha = cipher_message[i - 1];
            }

            let new_pixel = [
                scale_rgba.get_pixel(x, y)[0],
                scale_rgba.get_pixel(x, y)[1],
                scale_rgba.get_pixel(x, y)[2],
                alpha,
            ];

            scale_rgba.put_pixel(x, y, image::Rgba(new_pixel));

            i += 1;
            if i > cipher_message_len {
                return scale_rgba;
            }
        }
    }

    return scale_rgba;
}

pub fn get_text_from_alpha(image: &DynamicImage) -> Vec<u8> {
    let (width, height) = image.dimensions();
    let scale_rgba = image;

    let mut result = Vec::new();

    let mut i = 0;
    let mut len = 0;
    for y in 0..height {
        for x in 0..width {
            if i == 0 {
                len = scale_rgba.get_pixel(x, y)[3];
            } else {
                if i <= len {
                    result.push(scale_rgba.get_pixel(x, y)[3])
                } else {
                    return result;
                }
            }

            i += 1;
        }
    }

    return result;
}
