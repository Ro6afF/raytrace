use super::super::Point;
use super::super::Ray;
use super::super::Vector;

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vector,
    vertical: Vector,
    u: Vector,
    v: Vector,
    lens_radius: f64,
    time0: f64,
    time1: f64,
}

impl Camera {
    pub fn new(
        look_from: Point,
        look_at: Point,
        vup: Vector,
        vfov: f64,
        aspect_ratio: f64,
        apreture: f64,
        focus_dist: f64,
        time0: f64,
        time1: f64,
    ) -> Camera {
        let theta = vfov * std::f64::consts::PI / 180.0;
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).normailzed();
        let u = (vup ^ w).normailzed();
        let v = w ^ u;

        let horizontal = focus_dist * viewport_width * u;
        let vertical = -focus_dist * viewport_height * v;

        Camera {
            origin: look_from,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: look_from - horizontal / 2.0 - vertical / 2.0 - focus_dist * w,
            u: u,
            v: v,
            lens_radius: apreture / 2.0,
            time0: time0,
            time1: time1,
        }
    }

    pub fn get_ray(&self, x: f64, y: f64) -> Ray {
        let rd = self.lens_radius * Vector::random_in_unit_disc_xy();
        let offset = self.u * rd.x + self.v * rd.y;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + x * self.horizontal + y * self.vertical - self.origin - offset,
            self.time0 + (self.time1 - self.time0) * fastrand::f64(),
        )
    }
}
