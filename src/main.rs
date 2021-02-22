mod rtrs;

use rtrs::objects::Camera;
use rtrs::objects::HitRecord;
use rtrs::objects::HitableList;
use rtrs::objects::Sphere;
use rtrs::Color;
use rtrs::Image;
use rtrs::Point;
use rtrs::Ray;
use rtrs::Vector;

fn random_unit_vector() -> Vector {
    let mut vec = Vector::new(rand::random::<f64>() * 2.0 - 1.0, rand::random::<f64>() * 2.0 - 1.0, rand::random::<f64>() * 2.0 - 1.0);

    while vec.lenght() > 1.0 {
        vec = Vector::new(rand::random::<f64>() * 2.0 - 1.0, rand::random::<f64>() * 2.0 - 1.0, rand::random::<f64>() * 2.0 - 1.0);
    }
    vec.normailze();

    vec
}

fn calc_color(r: &Ray, scene: &HitableList, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut rec = HitRecord::blank();

    if scene.hit(r, 0.001, f64::INFINITY, &mut rec) {
        let target = rec.p + rec.normal + random_unit_vector();

        return 0.5 * calc_color(&Ray::new(rec.p, target - rec.p), scene, depth - 1);
    }
    let mut unit = r.direction.clone();
    unit.normailze();
    let t = (unit.x + 1.0) / 2.0;
    t as f32 * Color::new(1.0, 1.0, 0.0) + (1.0 - t) as f32 * Color::new(0.0, 1.0, 1.0)
}

fn main() {
    // Image properties
    let aspect_ratio = 16.0 / 9.0;
    let height = 600;
    let width = (height as f64 * aspect_ratio) as i32;
    let mut img = Image::new("asdf.ppm", width, height);
    let spp = 100;
    let max_depth = 50;

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
                color += calc_color(&ray, &scene, max_depth);
            }
            img.write_pixel(color / spp as f32);
        }
    }
}
