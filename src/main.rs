mod rtrs;

use rtrs::Color;
use rtrs::Image;
use rtrs::Point;
use rtrs::Ray;
use rtrs::Vector;

fn hit_sphere(center: &Point, radius: f64, ray: &Ray) -> bool {
    let oc = ray.origin - *center;
    let a = ray.direction * ray.direction;
    let b = 2.0 * oc * ray.direction;
    let c = oc * oc - radius * radius;

    let discriminant = b * b - 4.0 * a * c;

    discriminant >= 0.0 
}

fn calc_color(r: &Ray) -> Color {
    if hit_sphere(&Point::new(0.0, 0.0, -1.0), 0.5, r) {
        return Color::new(255.0, 0.0, 0.0);
    }
    let mut unit = r.direction.clone();
    unit.normailze();
    let t = (unit.x + 1.0) / 2.0;
    t as f32 * Color::new(0.0, 0.0, 255.0) + (1.0 - t) as f32 * Color::new(0.0, 255.0, 0.0)
}

fn main() {
    // Image properties
    let width = 800;
    let height = 600;
    let mut img = Image::new("asdf.ppm", width, height);

    // Camera properties
    let viewport_width = 2.0;
    let viewport_height = (2 * height) as f64 / width as f64;
    let focal_lenght = 1.0;
    let origin = Point::new(0.0, 0.0, 0.0);
    let horizontal = Vector::new(viewport_width, 0.0, 0.0);
    let vertical = Vector::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin.clone()
        - horizontal.clone() / 2.0
        - vertical.clone() / 2.0
        - Vector::new(0.0, 0.0, focal_lenght);

    // Rendering
    for i in 0..height {
        println!("Line {} / {}", i + 1, height);
        for j in 0..width {
            let ray = Ray::new(
                origin,
                lower_left_corner
                    + (j as f64 / (width - 1) as f64) * horizontal
                    + (i as f64 / (height - 1) as f64) * vertical
                    - origin,
            );
            img.write_pixel(calc_color(&ray));
        }
    }
}
