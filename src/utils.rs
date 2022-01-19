use image::{DynamicImage, GenericImageView};

fn number_to_vec(num: u32) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    if num < 256 {
        result.push(num as u8);
    } else if num < 65536 {
        result.push((0 | (num >> 8)) as u8);
        result.push((0 | (num & 0xFF)) as u8);
    } else if num < 16777216 {
        result.push((0 | (num >> 16)) as u8);
        result.push((0 | ((num >> 8) & 0xFF)) as u8);
        result.push((0 | (num & 0xFF)) as u8);
    } else {
        result.push((0 | (num >> 24)) as u8);
        result.push((0 | ((num >> 16) & 0xFF)) as u8);
        result.push((0 | ((num >> 8) & 0xFF)) as u8);
        result.push((0 | (num & 0xFF)) as u8);
    }

    return result;
}

fn vec_to_num(num_vec: &Vec<u8>) -> u32 {
    let mut result: u32 = 0;

    let mut i = 1;
    let mut time = 1;
    for num in num_vec {
        if i > 4 {
            break;
        }

        result = (*num as u32) * time;

        time *= 256;
        i += 1;
    }

    return result;
}

pub fn save_text_to_alpha(image: &DynamicImage, cipher_message: &[u8]) -> image::RgbaImage {
    let (width, height) = image.dimensions();
    let mut scale_rgba = image.to_rgba8();

    let cipher_message_len = cipher_message.len();

    let mut cipher_len_vec = number_to_vec(cipher_message_len as u32);
    cipher_len_vec.push(0);

    let mut i = 0;
    let mut j = 0;
    for y in 0..height {
        for x in 0..width {
            let alpha: u8;

            if i < cipher_len_vec.len() {
                alpha = cipher_len_vec[i];
                j += 1;
            } else {
                alpha = cipher_message[i - j];
            }

            let new_pixel = [
                scale_rgba.get_pixel(x, y)[0],
                scale_rgba.get_pixel(x, y)[1],
                scale_rgba.get_pixel(x, y)[2],
                alpha,
            ];

            scale_rgba.put_pixel(x, y, image::Rgba(new_pixel));

            i += 1;
            if i > cipher_message_len + j - 1 {
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
    let mut message_len = Vec::new();

    let mut find_message_len = false;

    let mut i = 0;
    let mut j = 0;
    let mut len = 0;
    for y in 0..height {
        for x in 0..width {
            let character = scale_rgba.get_pixel(x, y)[3];
            if !find_message_len {
                if character == 0 {
                    find_message_len = true;
                    len = vec_to_num(&message_len);
                } else {
                    message_len.push(character);
                }

                j += 1;
            } else {
                if i < len + j {
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
