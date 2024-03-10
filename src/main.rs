use std::fs::{File, remove_file};
use std::{env, io};
use std::io::{BufWriter, Write};
use image::{self, DynamicImage, imageops::*, GenericImageView, GrayAlphaImage};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

fn main() {
    println!("Welcome to ASCII Creator by Xyehtz");

    loop {
        println!("Please enter the name of the image you want to convert (type \"quit\" to exit): ");

        let mut raw_path: String = String::new();
        io::stdin()
            .read_line(&mut raw_path)
            .expect("Error obtaining path");
        let path: &str = raw_path.as_str().trim();

        if path.to_lowercase() == "quit" {
            break;
        }

        let result = pixels_to_ascii(img_to_grayscale(resize_img(path)));

        let current_dir = env::current_dir();
        let current_dir: String = current_dir.unwrap().display().to_string();

        println!("Exited process with: {:?}\n\n", result.0);
        println!("Output file: {}\\{}\n\n", current_dir, result.1);
    }
}

fn resize_img(original_img_path: &str) -> String {
    println!("Please choose one of the following options for the quality of the image (Medium or Low quality is highly recommended):\n1. Very high\n2. High\n3. Medium\n4. Low\n5. Very low");
    let mut option_str: String = String::new();
    io::stdin()
        .read_line(&mut option_str)
        .expect("Error obtaining option");
    let option: u32 = option_str.trim().parse().expect("Error parsing option");

    let option: u32 = match option {
        1 => 1,
        2 => 2,
        3 => 4,
        4 => 8,
        5 => 16,
        _ => {
            0;
            panic!("Please choose a valid number");
        }
    };

    let img: DynamicImage = image::open(original_img_path).expect("Failed to open");
    let (width, height): (u32, u32) = img.dimensions();
    let img = resize(&img, width / option, height / option, Triangle);

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

fn pixels_to_ascii(img_path: String) -> (io::Result<()>, String) {
    let binding: String = img_path.replace(&img_path[img_path.len() - 3..img_path.len()], "txt");
    let file_path: &str = binding.as_str();

    let ascii_chars: [char; 79] = [
    ' ', '.', ',', '-', '_', '`', '^', '´', '\"', ':', '*', '!', '?', '¡', '¿', '(', ')', '[', ']', '{', '}', '|', '\\', '/', '>', '<', '=', '@', '#', '$', '%', '&', '*', '(', ')', '-', '_', '+', '=', '[', ']', '{', '}', '\\', '|', '/', '>', '<', '?', '¡', '¿', '~', '`', '^', '\"', ':', ';', ',', '.', '-', '_', '=', '+', '*', '&', '%', '$', '#', '@', '!', '?', '¿', '(', ')', ' ', ' ', ' ', ' ', ' '];

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

    writer.flush().expect("Couldn't flush writer");
    remove_file(&img_path).expect("Couldn't remove file");

    return (Ok(()), file_path.to_string());
}
