use crate::rtrs::Color;
use std::fs::File;
use std::io::Write;

pub struct Image {
    file: File,
    image: Vec<Vec<Color>>,
    width: i32,
    height: i32,
}

impl Image {
    pub fn new(filename: &str, width: i32, height: i32) -> Image {
        let mut file = File::create(filename).unwrap();
        file.write(format!("P3\n{} {}\n{}\n", width, height, 255).as_bytes())
            .unwrap();

        let mut img = vec![];

        for i in 0..width {
            img.push(vec![]);
            for _j in 0..height {
                img[i as usize].push(Color::new(0.0, 0.0, 0.0));
            }
        }

        Image {
            file: file,
            image: img,
            width: width,
            height: height,
        }
    }

    pub fn set_pixel(&mut self, i: i32, j: i32, color: Color) {
        self.image[i as usize][j as usize] = color;
    }

    pub fn write(&mut self) {
        for i in 0..self.height {
            for j in 0..self.width {
                self.image[j as usize][i as usize].clamp();
                self.file
                    .write_all(
                        format!(
                            "{} {} {}\n",
                            (255.0 * self.image[j as usize][i as usize].r.sqrt()) as i32,
                            (255.0 * self.image[j as usize][i as usize].g.sqrt()) as i32,
                            (255.0 * self.image[j as usize][i as usize].b.sqrt()) as i32
                        )
                        .as_bytes(),
                    )
                    .unwrap();
            }
        }
    }
}
