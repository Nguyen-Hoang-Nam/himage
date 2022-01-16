use clap::{App, Arg};
use image::io::Reader as ImageReader;
use image::GenericImageView;

use aes::Aes128;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};

use crate::utils::get_text_from_alpha;

mod utils;

fn main() -> std::io::Result<()> {
    let matches = App::new("dot-image")
        .version("0.1.0")
        .author("N.H Nam <nguyenhoangnam.dev@gmail.com>")
        .about("Change image to dot")
        .arg(
            Arg::with_name("image")
                .short("i")
                .long("image")
                .help("Import image")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("message")
                .short("m")
                .long("message")
                .help("Set message")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("key")
                .short("k")
                .long("key")
                .help("Set secret key")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("decode")
                .short("d")
                .long("decode")
                .help("Decode text in image"),
        )
        .get_matches();

    let image_path = match matches.value_of("image") {
        Some(path) => path,
        None => panic!("Missing path of image"),
    };

    let message = match matches.value_of("message") {
        Some(h) => h.to_string(),
        None => "".to_string(),
    };

    let decode = match matches.occurrences_of("decode") {
        0 => false,
        _ => true,
    };

    let mut key = match matches.value_of("key") {
        Some(h) => h.to_string(),
        None => panic!("Missing secret key"),
    };

    let key_len = key.len();
    if key_len > 16 {
        panic!("Too long secret key")
    } else if key_len < 16 {
        for _ in 0..(16 - key_len) {
            key = key + "0";
        }
    }

    let image = match ImageReader::open(image_path) {
        Ok(img) => img.decode().unwrap(),
        Err(_) => panic!("Can not open image"),
    };

    type Aes128Cbc = Cbc<Aes128, Pkcs7>;

    let cipher = Aes128Cbc::new_from_slices(key.as_bytes(), key.as_bytes()).unwrap();

    if decode {
        let result = get_text_from_alpha(&image);
        if result.is_empty() {
            panic!("Does not have message");
        }

        let message = cipher.decrypt_vec(&result).unwrap();

        println!("{}", String::from_utf8(message.to_vec()).unwrap());
    } else {
        let mut buffer = [0u8; 32];

        let message_bytes = message.as_bytes();
        let pos = message_bytes.len();
        buffer[..pos].copy_from_slice(message_bytes);

        let ciphertext = cipher.encrypt(&mut buffer, pos).unwrap();

        let result = utils::save_text_to_alpha(&image, ciphertext);

        let (width, height) = &image.dimensions();

        image::save_buffer(
            "result.png",
            &result,
            *width,
            *height,
            image::ColorType::Rgba8,
        )
        .unwrap();
    }

    Ok(())
}
