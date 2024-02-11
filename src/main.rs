use std::fs::{File, remove_file};
use std::io;
use std::io::{BufWriter, Write};
use image::{self, DynamicImage, GrayImage, imageops::*, GenericImageView};

fn main() {
    println!("Welcome to ASCII Creator by Xyehtz");
    println!("Please enter the name of the image you want to convert: ");

    let mut raw_path: String = String::new();
    io::stdin()
        .read_line(&mut raw_path)
        .expect("Error obtaining path");
    let path: &str = raw_path.as_str();

    let result = pixels_to_ascii(img_to_grayscale(resize_img("images/360_F_300473329_08cy1w5rbmzxLgCaOwgHIYEymVAAJTh9.jpg")));

    println!("Exited process with: {:?}\n\n", result);
}

fn resize_img(original_img_path: &str) -> String {
    let img: DynamicImage = image::open(original_img_path).expect("Failed to open");
    let (width, height): (u32, u32) = img.dimensions();
    let img = resize(&img, width / 2, height / 2, Triangle);

    let mut new_img_prefix: String = String::from("new_");
    new_img_prefix.push_str(&original_img_path);
    let mut new_path_string: String = String::new();

    for (i, c) in new_img_prefix.chars().enumerate() {
        if c == '/' {
            new_path_string = new_img_prefix.replace(&new_img_prefix[4..i + 1], "");
            break;
        }
    }

    img.save(&new_path_string).unwrap();
    return new_path_string.clone();
}

fn img_to_grayscale(img_path: String) -> String {
    let img: DynamicImage = image::open(img_path.clone()).expect("Error opening the image");
    let img: DynamicImage = img.grayscale();
    let img: &GrayImage = img.as_luma8().unwrap();
    img.save(img_path.clone()).unwrap();
    return img_path.clone();
}

fn pixels_to_ascii(img_path: String) -> std::io::Result<()> {
    let binding: String = img_path.replace(&img_path[img_path.len() - 3..img_path.len()], ".txt");
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
