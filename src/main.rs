mod rtrs;

use rtrs::objects::Camera;
use rtrs::objects::HitRecord;
use rtrs::objects::HitableList;
use rtrs::objects::Sphere;
use rtrs::Color;
use rtrs::Image;
use rtrs::Point;
use rtrs::Ray;

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
    let aspect_ratio = 16.0 / 9.0;
    let height = 600;
    let width = (height as f64 * aspect_ratio) as i32;
    let mut img = Image::new("asdf.ppm", width, height);
    let spp = 100;
    let cam = Camera::new();

    // Scene
    let mut scene = HitableList::new();
    scene.add(Box::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    scene.add(Box::new(Sphere::new(Point::new(0.0, 100.5, -1.0), 100.0)));
    // Rendering
    for i in 0..height {
        println!("Line {} / {}", i + 1, height);
        for j in 0..width {
            let mut color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..spp {
                let ray = cam.get_ray(
                    (j as f64 + rand::random::<f64>()) / (width - 1) as f64,
                    (i as f64 + rand::random::<f64>()) / (height - 1) as f64,
                );
                color += calc_color(&ray, &scene);
            }
            img.write_pixel(color / spp as f32);
        }
    }
}
