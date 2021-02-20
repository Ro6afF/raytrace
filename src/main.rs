mod rtrs;

use rtrs::objects::HitRecord;
use rtrs::objects::HitableList;
use rtrs::objects::Sphere;
use rtrs::Color;
use rtrs::Image;
use rtrs::Point;
use rtrs::Ray;
use rtrs::Vector;

fn calc_color(r: &Ray, scene: &HitableList) -> Color {
    let mut rec = HitRecord::blank();

    if scene.hit(r, 0.0, f64::INFINITY, &mut rec) {
        return 255.0
            * 0.5
            * Color::new(
                rec.normal.x as f32 + 1.0,
                rec.normal.y as f32 + 1.0,
                rec.normal.z as f32 + 1.0,
            );
    }
    let mut unit = r.direction.clone();
    unit.normailze();
    let t = (unit.x + 1.0) / 2.0;
    t as f32 * Color::new(255.0, 255.0, 0.0) + (1.0 - t) as f32 * Color::new(0.0, 255.0, 255.0)
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

    // Scene
    let mut scene = HitableList::new();
    scene.add(Box::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    scene.add(Box::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

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
            img.write_pixel(calc_color(&ray, &scene));
        }
    }
}
