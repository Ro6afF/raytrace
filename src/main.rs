mod rtrs;

use rtrs::Image;
use rtrs::Color;

fn main() {
    let width = 255;
    let height = 255;
    let mut img = Image::new("asdf.ppm", width, height);
    
    for i in 0..height {
        for _j in 0..width {
            img.write_pixel(Color::new(i as f32, i as f32, i as f32));
        }
    }
}