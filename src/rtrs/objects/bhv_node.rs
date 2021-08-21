use crate::rtrs::objects::Aabb;
use crate::HitRecord;
use crate::Hitable;
use crate::HitableList;
use crate::Ray;
use std::cmp::Ordering;
use std::sync::Arc;

#[derive(Debug)]
pub struct BhvNode {
    left: Arc<dyn Hitable>,
    right: Arc<dyn Hitable>,
    aabb: Aabb,
}

impl BhvNode {
    pub fn new(list: &HitableList, time0: f64, time1: f64) -> Self {
        let len = list.objects.len();
        Self::new_vec(&list.objects, 0, len, time0, time1)
    }

    fn box_compare(a: &Arc<dyn Hitable>, b: &Arc<dyn Hitable>, axis: i32) -> Ordering {
        let mut boxa = Aabb::blank();
        let mut boxb = Aabb::blank();

        a.bounding_box(0.0, 0.0, &mut boxa);
        b.bounding_box(0.0, 0.0, &mut boxb);

        match {
            match axis {
                0 => boxa.min.x.partial_cmp(&boxb.min.x),
                1 => boxa.min.y.partial_cmp(&boxb.min.y),
                _ => boxa.min.z.partial_cmp(&boxb.min.z),
            }
        } {
            Some(i) => i,
            _ => Ordering::Equal,
        }
    }

    fn box_x_compare(a: &Arc<dyn Hitable>, b: &Arc<dyn Hitable>) -> Ordering {
        Self::box_compare(a, b, 0)
    }

    fn box_y_compare(a: &Arc<dyn Hitable>, b: &Arc<dyn Hitable>) -> Ordering {
        Self::box_compare(a, b, 1)
    }

    fn box_z_compare(a: &Arc<dyn Hitable>, b: &Arc<dyn Hitable>) -> Ordering {
        Self::box_compare(a, b, 2)
    }

    pub fn new_vec(
        list: &Vec<Arc<dyn Hitable>>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> Self {
        let left;
        let right;
        let aabb;
        let mut list = list[start..end].to_vec();
        let end = list.len();
        let start = 0;

        let axis = fastrand::i32(0..3);
        let comparator = if axis == 0 {
            Self::box_x_compare
        } else if axis == 1 {
            Self::box_y_compare
        } else {
            Self::box_z_compare
        };

        if end == 1 {
            left = list[start].clone();
            right = list[start].clone();
        } else {
            list.sort_by(|a, b| comparator(a, b));

            let mid = (start + end) / 2;
            left = Arc::new(Self::new_vec(&list, start, mid, time0, time1));
            right = Arc::new(Self::new_vec(&list, mid, end, time0, time1));
        }

        let mut lbox = Aabb::blank();
        let mut rbox = Aabb::blank();
        left.bounding_box(time0, time1, &mut lbox);
        right.bounding_box(time0, time1, &mut rbox);
        aabb = Aabb::surrounding_box(&lbox, &rbox);

        Self {
            left: left,
            right: right,
            aabb: aabb,
        }
    }
}

impl Hitable for BhvNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        if !self.aabb.hit(ray, t_min, t_max) {
            return false;
        }
        let lhit = self.left.hit(ray, t_min, t_max, record);
        let rhit = self
            .right
            .hit(ray, t_min, if lhit { record.t } else { t_max }, record);
        lhit || rhit
    }
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        output_box.min = self.aabb.min;
        output_box.max = self.aabb.max;

        true
    }
}
