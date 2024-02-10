use image::{self, DynamicImage, GrayImage, imageops::*, GenericImageView};


fn main() {
    img_to_grayscale(resize_img("images/360_F_300473329_08cy1w5rbmzxLgCaOwgHIYEymVAAJTh9.jpg"));
}

fn resize_img(original_img_path: &str) -> String {
    let img: DynamicImage = image::open(original_img_path).expect("Failed to open");
    let (width, height): (u32, u32) = img.dimensions();
    let mut img = resize(&img, width / 2, height / 2, Triangle);

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

fn img_to_grayscale(img_path: String) {
    let mut img: DynamicImage = image::open(img_path.clone()).expect("Error opening the image");
    let mut img: DynamicImage = img.grayscale();
    let img: &GrayImage = img.as_luma8().unwrap();
    img.save(img_path).unwrap();
}
