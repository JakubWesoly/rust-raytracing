use image::{ImageBuffer, Rgb};
use std::env;
use std::process::Command;

pub mod vectors;
pub mod ray;
use vectors::{Vec3, Point3, Color};
use ray::Ray;

fn ray_color(r: &Ray) -> Color {
    let unit_direction = r.direction.unit_vector();
    let t = (unit_direction.y + 1.0) * 0.5;
    Color::from(1.0, 1.0, 1.0) * (1.0 - t) + Color::from(0.5, 0.7, 1.0) * t
}

fn main() {
    // IMAGE
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO as f64) as u32;

    // CAMERA
    let viewpoint_height = 2.0;
    let viewpoint_width = ASPECT_RATIO * viewpoint_height;
    let focal_length = 1.0;

    let origin = Point3::new();
    let horizontal = Vec3::from(viewpoint_width, 0.0, 0.0);
    let vertical = Vec3::from(0.0, viewpoint_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::from(0.0, 0.0, focal_length);

    // RENDERING
    let mut img = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let u  = (x as f64) / (IMAGE_WIDTH - 1) as f64;
        let v  = (y as f64) / (IMAGE_HEIGHT - 1) as f64;
        
        let r = Ray::from(origin, lower_left_corner + horizontal * u + vertical * v - origin);
        let color = ray_color(&r);

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
