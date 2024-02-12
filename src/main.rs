use std::fs::{File, remove_file};
use std::io;
use std::io::{BufWriter, Write};
use image::{self, DynamicImage, GrayImage, imageops::*, GenericImageView, GrayAlphaImage};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

fn main() {
    println!("Welcome to ASCII Creator by Xyehtz");
    println!("Please enter the name of the image you want to convert: ");

    let mut raw_path: String = String::new();
    io::stdin()
        .read_line(&mut raw_path)
        .expect("Error obtaining path");
    let path: &str = raw_path.as_str().trim();

    let result = pixels_to_ascii(img_to_grayscale(resize_img(path)));

    println!("Exited process with: {:?}\n\n", result);
}

fn resize_img(original_img_path: &str) -> String {
    let img: DynamicImage = image::open(original_img_path).expect("Failed to open");
    let (width, height): (u32, u32) = img.dimensions();
    let img = resize(&img, width / 2, height / 2, Triangle);

    let mut rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    rand_string.push_str(".png");

    img.save(&rand_string).unwrap();
    return rand_string;
}

fn img_to_grayscale(img_path: String) -> String {
    let img: DynamicImage = image::open(&img_path).expect("Error opening the image");
    let img: DynamicImage = img.grayscale();
    let img: &GrayAlphaImage = img.as_luma_alpha8().unwrap();
    img.save(&img_path).unwrap();
    return img_path;
}

fn pixels_to_ascii(img_path: String) -> io::Result<()> {
    let binding: String = img_path.replace(&img_path[img_path.len() - 3..img_path.len()], "txt");
    let file_path: &str = binding.as_str();

    let ascii_chars: [char; 9] = ['@', '#', '8', '&', 'o', ':', '*', '.', ' '];
    let img: DynamicImage = image::open(&img_path).expect("Error opening the image");

    let file: File = File::create(&file_path).expect("Error creating the file");
    let mut writer: BufWriter<&File> = BufWriter::new(&file);

    for y in 0..img.height() {
        for x in 0..img.width() {
            let intensity: usize = img.get_pixel(x, y)[0] as usize;
            let ascii_index: usize = (intensity as f32 / 255.0 * (ascii_chars.len() - 1) as f32).round() as usize;
            let ascii_char: char = ascii_chars[ascii_index];
            let ascii_char: &[u8] = &[ascii_char as u8];

            writer.write_all(ascii_char).expect("Error writing ascii character");
        }
        writer.write_all("\n".as_bytes()).expect("Error creating new line")
    }
    remove_file(&img_path).expect("Couldn't remove file");

    writer.flush()?;
    Ok(())
}
