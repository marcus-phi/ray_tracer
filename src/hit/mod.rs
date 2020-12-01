use crate::material::Material;
use crate::math::Ray;
use crate::math::Vec3;
use crate::math::AABB;

pub struct HitPoint<'a> {
    pub t: f64,
    pub p: Vec3,
    pub n: Vec3,
    pub mat: &'a Box<dyn Material>,
}

pub trait Hitable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitPoint>;
    fn bbox(&self) -> Option<AABB>;
}

pub struct HitableList {
    list: Vec<Box<dyn Hitable>>,
    bbox: Option<AABB>,
}

impl HitableList {
    pub fn new() -> HitableList {
        HitableList {
            list: Vec::new(),
            bbox: None,
        }
    }

    pub fn push(&mut self, hitable: Box<dyn Hitable>) {
        match hitable.bbox() {
            Some(b) => {
                if self.bbox.is_none() {
                    self.bbox = Some(b)
                } else {
                    self.bbox = Some(self.bbox.unwrap() + b)
                }
            }
            _ => {}
        }
        self.list.push(hitable);
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitPoint> {
        let mut closest = t_max;
        let mut result: Option<HitPoint> = None;
        for item in self.list.iter() {
            let temp = item.hit(r, t_min, closest);
            match temp {
                Some(hp) => {
                    closest = hp.t;
                    result = Some(hp);
                }
                _ => {}
            }
        }

        result
    }

    fn bbox(&self) -> Option<AABB> {
        self.bbox
    }
}
