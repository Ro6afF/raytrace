use super::super::Ray;
use super::Aabb;
use super::HitRecord;
use super::Hitable;
use super::HitableList;
use std::cmp::Ordering;
use std::sync::Arc;

#[derive(Debug)]
pub struct BhvNode {
    left: Arc<dyn Hitable>,
    right: Arc<dyn Hitable>,
    aabb: Aabb,
}

impl BhvNode {
    pub fn new(list: &mut HitableList, time0: f64, time1: f64) -> BhvNode {
        let len = list.objects.len();
        BhvNode::new_vec(&mut list.objects, 0, len, time0, time1)
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
        BhvNode::box_compare(a, b, 0)
    }

    fn box_y_compare(a: &Arc<dyn Hitable>, b: &Arc<dyn Hitable>) -> Ordering {
        BhvNode::box_compare(a, b, 1)
    }

    fn box_z_compare(a: &Arc<dyn Hitable>, b: &Arc<dyn Hitable>) -> Ordering {
        BhvNode::box_compare(a, b, 2)
    }

    pub fn new_vec(
        list: &mut Vec<Arc<dyn Hitable>>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> BhvNode {
        let left;
        let right;
        let aabb;

        let axis = fastrand::i32(0..3);
        let comparator = if axis == 0 {
            BhvNode::box_x_compare
        } else if axis == 1 {
            BhvNode::box_y_compare
        } else {
            BhvNode::box_z_compare
        };

        let object_span = end - start;
        if object_span == 1 {
            left = list[start].clone();
            right = list[start].clone();
        } else if object_span == 2 {
            if comparator(&list[start], &list[start + 1]) == Ordering::Less {
                left = list[start].clone();
                right = list[start + 1].clone();
            } else {
                right = list[start].clone();
                left = list[start + 1].clone();
            }
        } else {
            list.sort_by(|a, b| comparator(a, b));

            let mid = start + object_span / 2;
            left = Arc::new(BhvNode::new_vec(list, start, mid, time0, time1));
            right = Arc::new(BhvNode::new_vec(list, mid, end, time0, time1));
        }

        let mut lbox = Aabb::blank();
        let mut rbox = Aabb::blank();
        left.bounding_box(time0, time1, &mut lbox);
        right.bounding_box(time0, time1, &mut rbox);
        aabb = Aabb::surrounding_box(&lbox, &rbox);

        BhvNode {
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
