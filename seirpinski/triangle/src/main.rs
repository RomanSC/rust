//! seirpinski triangle | Mon, Feb 20, 2017 | Roman S. Collins

/// Not my code. Thanks to Dan Mack for showing me how:
/// https://www.youtube.com/watch?v=T2Hwu-XiVkA

extern crate image;
extern crate rand;
use rand::Rng;
use std::fs::File;
use std::path::Path;

/// points used to plot triangle
pub struct Point {
    x: u32, // image library uses unsigned int
    y: u32,
}

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;


/// main program
pub fn main() {
    let mut img = image::ImageBuffer::from_fn(WIDTH, HEIGHT, |x, y| {
        if x == 0 && y == 0 {
            image::Luma([0u8])
        } else {
            image::Luma([255u8])
        }
    });

    let mut cnt: u32 = 1_000_000;

    // Points of triangle
    let pts: [Point; 3] = [
        Point {x: WIDTH / 2, y: 0}, // Top
        Point {x: 0, y: HEIGHT}, // Middle
        Point {x: WIDTH, y: HEIGHT}, // Bottom
    ];

    let mut num: usize;

    let mut p = Point {x: 350, y: 350}; // "Pen" for starting

    let pixel = img[(0, 0)];

    while cnt > 0 {
        cnt -= 1;
        num = rand::thread_rng().gen_range(0, 3);
        p.x = (p.x + pts[num].x) / 2;
        p.y = (p.y + pts[num].y) / 2;
        img.put_pixel(p.x, p.y, pixel);
    }
    let ref mut fileout = File::create(&Path::new("tri.png")).unwrap(); // Create the png

    let _ = image::ImageLuma8(img).save(fileout, image::PNG);
}
