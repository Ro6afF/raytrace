use crate::rtrs::Color;
use std::fs::File;
use std::io::Write;

pub struct Image {
    file: File,
}

impl Image {
    pub fn new(filename: &str, width: i32, height: i32) -> Image {
        let mut file = File::create(filename).unwrap();
        file.write(format!("P3\n{} {}\n{}\n", width, height, 255).as_bytes())
            .unwrap();
        Image { file: file }
    }

    pub fn write_pixel(&mut self, color: Color) {
        self.file
            .write_all(format!("{} {} {}\n", color.r as i32, color.g as i32, color.b as i32).as_bytes())
            .unwrap();
    }
}
