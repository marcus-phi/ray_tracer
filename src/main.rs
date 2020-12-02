use rand::prelude::*;
use ray_tracer::camera::*;
use ray_tracer::hit::Hitable;
use ray_tracer::hit::HitableList;
use ray_tracer::material::*;
use ray_tracer::math::*;
use ray_tracer::render;
use ray_tracer::shape::*;
use ray_tracer::texture::*;

fn main() {
    let image_dim = (720, 480);

    let world = random_world();

    let camera = PerspectiveCamera::new(
        Vec3::new(13.0, 2.0, 3.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        (image_dim.0 as f64) / (image_dim.1 as f64),
        0.05,
        10.0,
    );

    render(world, Box::new(camera), image_dim, 64);
}

fn random_world() -> Box<dyn Hitable> {
    let mut world = HitableList::new();

    let ground = Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(ConstantTexture::new(Color3::new(0.7, 0.7, 0.7))),
    );
    let red = Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Lambertian::new(ConstantTexture::new(Color3::new(1.0, 0.0, 0.0))),
    );
    let green = Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Lambertian::new(ConstantTexture::new(Color3::new(0.0, 1.0, 0.0))),
    );

    world.push(ground);
    world.push(red);
    world.push(green);

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3::new(
                (a as f64) + 0.9 * rng.gen::<f64>(),
                0.3,
                (b as f64) + 0.9 * rng.gen::<f64>(),
            );
            if (center - Vec3::new(4.0, 0.3, 0.0)).len() > 0.9 {
                let tex = ConstantTexture::new(Color3::new(rng.gen(), rng.gen(), rng.gen()));
                let mat = Lambertian::new(tex);
                let sphere = Sphere::new(center, 0.3, mat);
                world.push(sphere);
            }
        }
    }

    Box::new(world)
}
