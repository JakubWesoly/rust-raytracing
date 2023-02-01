use image::{ImageBuffer, Rgb};
use std::env;
use std::process::Command;

pub mod vectors;
pub mod ray;
use vectors::{Vec3, Point3, Color};
fn main() {
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    let mut img = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let r = x as f64 / (IMAGE_WIDTH - 1) as f64;
        let g = y as f64 / (IMAGE_HEIGHT - 1) as f64;
        let b = 0.25;
        let color = Color::new(r, g, b);

        *pixel = Rgb([
            (255.999 * color.x) as u8,
            (255.999 * color.y) as u8,
            (255.999 * color.z) as u8,
        ]);
    }

    img.save("image.png").unwrap();
    
    if env::var("os").is_err() {
        Command::new("sh")
            .arg("-c")
            .arg("open image.png")
            .output()
            .expect("Failed to open image");
    } else {
        Command::new("cmd")
            .args(&["/C", "start", "image.png"])
            .output()
            .expect("Failed to open image");
    }
    // if env::var("os").unwrap() == "Windows_NT" {
    //     Command::new("cmd")
    //         .args(&["/C", "start", "image.png"])
    //         .output()
    //         .expect("Failed to open image");
    // } else {
    //     Command::new("sh")
    //         .arg("-c")
    //         .arg("xdg-open image.png")
    //         .output()
    //         .expect("Failed to open image");
    // }
}
