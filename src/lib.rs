pub mod camera;
pub mod hit;
pub mod material;
pub mod math;
pub mod shape;
pub mod texture;

use camera::*;
use hit::*;
use math::*;
use rand::prelude::*;

pub fn render(world: &dyn Hitable, camera: &dyn Camera, image_dim: (u32, u32), n_samples: u32) {
    let mut rng = rand::thread_rng();

    println!("P3\n{} {}\n255", image_dim.0, image_dim.1);

    for j in (0..image_dim.1).rev() {
        for i in 0..image_dim.0 {
            let mut col = Color3::new(0.0, 0.0, 0.0);
            for _ in 0..n_samples {
                let u: f64 = ((i as f64) + rng.gen::<f64>()) / (image_dim.0 as f64);
                let v: f64 = ((j as f64) + rng.gen::<f64>()) / (image_dim.1 as f64);
                let ray = camera.get_ray(u, v);
                col = col + ray_hit_world(ray, world, 0);
            }

            col = (col / (n_samples as f64)).gamma();
            let ir = (col.r * 255.99).floor() as u32;
            let ig = (col.r * 255.99).floor() as u32;
            let ib = (col.r * 255.99).floor() as u32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}

fn ray_hit_world(r: Ray, world: &dyn Hitable, depth: u32) -> Color3 {
    let hit_result = world.hit(r, 0.001, f64::MAX);
    match hit_result {
        Some(hp) => {
            let scatter_result = hp.mat.scatter(r, hp);
            match scatter_result {
                Some(sr) => sr.attenuation * ray_hit_world(sr.scattered_ray, world, depth + 1),
                None => Color3::new(0.0, 0.0, 0.0),
            }
        }
        None => {
            let t = 0.5 * r.direction.y + 1.0;
            (1.0 - t) * Color3::new(1.0, 1.0, 1.0) + t * Color3::new(0.5, 0.7, 1.0)
        }
    }
}
