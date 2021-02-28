use super::super::Point;
use super::super::Ray;

#[derive(Debug)]
pub struct Aabb {
    pub max: Point,
    pub min: Point,
}

impl Aabb {
    pub fn new(min: Point, max: Point) -> Aabb {
        Aabb { min: min, max: max }
    }

    pub fn blank() -> Aabb {
        Aabb::new(Point::new(0.0, 0.0, 0.0), Point::new(0.0, 0.0, 0.0))
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        macro_rules! check {
            ($d: ident) => {
                let invd = 1.0 / r.direction.$d;
                let mut t0 = (self.min.$d - r.origin.$d) * invd;
                let mut t1 = (self.max.$d - r.origin.$d) * invd;
                if invd < 0.0 {
                    std::mem::swap(&mut t0, &mut t1);
                }

                let t_min = if t0 > t_min { t0 } else { t_min };
                let t_max = if t1 < t_max { t1 } else { t_max };
                if t_min >= t_max {
                    return false;
                }
            };
        }
        check!(x);
        check!(y);
        check!(z);
        true
    }

    pub fn surrounding_box(a: &Aabb, b: &Aabb) -> Aabb {
        let mut small = a.min;
        let mut big = a.max;
        
        if small.x > b.min.x {
            small.x = b.min.x;
        }
        
        if small.y > b.min.y {
            small.y = b.min.y;
        }
        
        if small.z > b.min.z {
            small.z = b.min.z;
        }
        
        if big.x < b.max.x {
            big.x = b.max.x;
        }
        
        if big.y < b.max.y {
            big.y = b.max.y;
        }
        
        if big.z < b.max.z {
            big.z = b.max.z;
        }
        Aabb::new(small, big)
    }
}
