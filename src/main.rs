mod rtrs;

use rayon::prelude::*;
use rtrs::materials::Dielectric;
use rtrs::materials::DiffuseLight;
use rtrs::materials::Lambertian;
use rtrs::materials::Material;
use rtrs::materials::Metal;
use rtrs::objects::BhvNode;
use rtrs::objects::Camera;
use rtrs::objects::HitRecord;
use rtrs::objects::Hitable;
use rtrs::objects::HitableList;
use rtrs::objects::MovingSphere;
use rtrs::objects::Sphere;
use rtrs::textures::CheckerTexture;
use rtrs::textures::SolidColor;
use rtrs::Color;
use rtrs::Image;
use rtrs::Point;
use rtrs::Ray;
use rtrs::Vector;
use std::sync::Arc;
use std::sync::Mutex;

fn calc_color(r: &Ray, scene: &dyn Hitable, background: &Color, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut rec = HitRecord::blank();

    if (*scene).hit(r, 0.001, f64::INFINITY, &mut rec) {
        let mut scattered = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 0.0), 0.0);
        let mut attenuation = Color::new(0.0, 0.0, 0.0);
        match &rec.material {
            Some(x) => {
                let emited = x.emitted(rec.u, rec.v, &rec.p);

                if !x.scatter(r, &rec, &mut attenuation, &mut scattered) {
                    return emited;
                }
                return emited + attenuation * calc_color(&scattered, scene, background, depth - 1);
            }
            _ => {}
        }
        return Color::new(0.0, 0.0, 0.0);
    }

    *background
}

fn random_scene() -> HitableList {
    let mut world = HitableList::new();
    // Lamp
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, 100.0, 0.0),
        50.0,
        Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(Color::new(
            1.0, 1.0, 0.98,
        ))))),
    )));

    // Ground
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(Arc::new(CheckerTexture::new(
            Arc::new(SolidColor::new(Color::new(0.2, 0.3, 0.1))),
            Arc::new(SolidColor::new(Color::new(0.9, 0.9, 0.9))),
        )))),
    )));

    for i in -13..13 {
        for j in -13..13 {
            let mat_choise = fastrand::f64();
            let material: Arc<dyn Material>;
            let albedo = Arc::new(SolidColor::new(Color::random()));

            if mat_choise < 0.7 {
                material = Arc::new(Lambertian::new(albedo));
            } else if mat_choise < 0.9 {
                let fuzz = fastrand::f64() * 0.5;
                material = Arc::new(Metal::new(albedo, fuzz));
            } else {
                material = Arc::new(Dielectric::new(1.2 + (1.8 - 1.2) * fastrand::f64(), albedo));
            }

            if fastrand::f64() < 0.7 {
                world.add(Arc::new(Sphere::new(
                    Point::new(
                        i as f64 + 0.9 * fastrand::f64(),
                        0.1 + (0.3 - 0.1) * fastrand::f64(),
                        j as f64 + 0.9 * fastrand::f64(),
                    ),
                    0.1 + (0.3 - 0.1) * fastrand::f64(),
                    material,
                )));
            } else {
                let center0 = Point::new(
                    i as f64 + 0.9 * fastrand::f64(),
                    0.1 + (0.3 - 0.1) * fastrand::f64(),
                    j as f64 + 0.9 * fastrand::f64(),
                );
                let center1 = center0 + Point::new(0.0, fastrand::f64() * 0.5, 0.0);
                world.add(Arc::new(MovingSphere::new(
                    center0,
                    center1,
                    0.0,
                    1.0,
                    0.1 + (0.3 - 0.1) * fastrand::f64(),
                    material,
                )));
            }
        }
    }

    world.add(Arc::new(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Dielectric::new(
            1.5,
            Arc::new(SolidColor::new(Color::new(0.5, 0.5, 0.0))),
        )),
    )));

    world.add(Arc::new(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Lambertian::new(Arc::new(SolidColor::new(Color::new(
            0.4, 0.2, 0.1,
        ))))),
    )));

    world.add(Arc::new(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal::new(
            Arc::new(SolidColor::new(Color::new(0.7, 0.6, 0.5))),
            0.0,
        )),
    )));

    world
}

fn main() {
    // Image properties
    let aspect_ratio = 16.0 / 9.0;
    let height = 600;
    let width = (height as f64 * aspect_ratio) as i32;
    let img = Mutex::new(Image::new("asdf.ppm", width, height));
    let spp = 10;
    let max_depth = 50;

    let cam = Camera::new(
        Point::new(13.0, 2.0, 3.0),
        Point::new(0.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        0.1,
        10.0,
        0.0,
        1.0,
    );

    // Scene
    let balls = random_scene();
    let scene = BhvNode::new(&balls, 0.0, 1.0);

    // Rendering
    for i in 0..height {
        println!("Line {} / {}", i + 1, height);
        (0..width).into_par_iter().for_each(|j| {
            let mut color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..spp {
                let ray = cam.get_ray(
                    (j as f64 + fastrand::f64()) / (width - 1) as f64,
                    (i as f64 + fastrand::f64()) / (height - 1) as f64,
                );
                color += calc_color(&ray, &scene, &(Color::new(0.78, 0.87, 1.0) * 0.3), max_depth);
            }
            img.lock().unwrap().set_pixel(j, i, color / spp as f32);
        });
    }
    img.lock().unwrap().write();
}
