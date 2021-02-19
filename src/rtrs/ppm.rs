use std::fs::File;
use std::io::Write;
use crate::rtrs::Color;


pub struct Image {
    file: File,
}

impl Image {
    pub fn new(filename: &str, width: i32, height: i32) -> Image {
        let mut file = File::create(filename).unwrap();
        file.write(format!("P3\n{} {}\n{}\n", width, height, 255).as_bytes()).unwrap();
        Image {
            file: file,
        }
    }

    pub fn write_pixel(&mut self, color: Color) {
        self.file.write_all(format!("{} {} {}\n", color.r, color.g, color.b).as_bytes()).unwrap();
    }
}
