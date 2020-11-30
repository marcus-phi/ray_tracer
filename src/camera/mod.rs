use crate::math;
use crate::math::Ray;
use crate::math::Vec3;
use std::f64::consts;

pub trait Camera {
    fn get_ray(&self, s: f64, t: f64) -> Ray;
}

#[derive(Debug)]
pub struct PerspectiveCamera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lens_radius: f64,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl PerspectiveCamera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        up: Vec3,
        vertical_fov: f64,
        aspect: f64,
        aperture: f64,
        focus_distance: f64,
    ) -> PerspectiveCamera {
        let theta: f64 = vertical_fov * consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let origin = look_from;
        let w = (look_from - look_at).unit();
        let u = up.cross(w).unit();
        let v = w.cross(u);
        let lower_left_corner = origin
            - half_width * focus_distance * u
            - half_height * focus_distance * v
            - focus_distance * w;
        let horizontal = 2.0 * half_width * focus_distance * u;
        let vertical = 2.0 * half_height * focus_distance * v;
        let lens_radius = aperture / 2.0;
        PerspectiveCamera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            lens_radius,
            u,
            v,
            w,
        }
    }
}

impl Camera for PerspectiveCamera {
    fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * math::random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
