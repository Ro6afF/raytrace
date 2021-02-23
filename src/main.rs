mod rtrs;

use rayon::prelude::*;
use rtrs::materials::Dielectric;
use rtrs::materials::Lambertian;
use rtrs::materials::Metal;
use rtrs::objects::Camera;
use rtrs::objects::HitRecord;
use rtrs::objects::HitableList;
use rtrs::objects::Sphere;
use rtrs::Color;
use rtrs::Image;
use rtrs::Point;
use rtrs::Ray;
use rtrs::Vector;
use std::sync::Arc;
use std::sync::Mutex;

fn calc_color(r: &Ray, scene: &HitableList, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut rec = HitRecord::blank();

    if scene.hit(r, 0.001, f64::INFINITY, &mut rec) {
        let mut scattered = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 0.0));
        let mut attenuation = Color::new(0.0, 0.0, 0.0);
        match &rec.material {
            Some(x) => {
                if x.scatter(r, &rec, &mut attenuation, &mut scattered) {
                    return attenuation * calc_color(&scattered, scene, depth - 1);
                }
            }
            _ => {}
        }
        return Color::new(0.0, 0.0, 0.0);
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
    let img = Mutex::new(Image::new("asdf.ppm", width, height));
    let spp = 100;
    let max_depth = 50;

    let cam = Camera::new();

    // Scene
    let mut scene = HitableList::new();
    scene.add(Arc::new(Sphere::new(
        Point::new(0.0, -100.5, -1.0),
        100.0,
        Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0))),
    )));
    scene.add(Arc::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        0.5,
        Arc::new(Dielectric::new(1.5)),
    )));
    scene.add(Arc::new(Sphere::new(
        Point::new(0.0, 0.0, -1.0),
        0.5,
        Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.1)),
    )));
    scene.add(Arc::new(Sphere::new(
        Point::new(1.0, 0.0, -1.0),
        0.5,
        Arc::new(Lambertian::new(Color::new(0.8, 0.6, 0.2))),
    )));

    // Rendering
    for i in 0..height {
        println!("Line {} / {}", i + 1, height);
        (0..width).into_par_iter().for_each(|j| {
            let mut color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..spp {
                let ray = cam.get_ray(
                    (j as f64 + rand::random::<f64>()) / (width - 1) as f64,
                    (i as f64 + rand::random::<f64>()) / (height - 1) as f64,
                );
                color += calc_color(&ray, &scene, max_depth);
            }
            img.lock().unwrap().set_pixel(j, i, color / spp as f32);
        });
    }
    img.lock().unwrap().write();
}
