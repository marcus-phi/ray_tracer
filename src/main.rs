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

    render(world, Box::new(camera), image_dim, 128);
}

fn random_world() -> Box<dyn Hitable> {
    let mut world = HitableList::new();

    world.push(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(ConstantTexture::new(Color3::new(0.7, 0.7, 0.7))),
    ));
    world.push(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Metal::new(Color3::new(0.7, 0.6, 0.5), 0.0),
    ));
    world.push(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Lambertian::new(ConstantTexture::new(Color3::new(0.4, 0.2, 0.1))),
    ));
    world.push(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Dielectric::new(1.5),
    ));

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3::new(
                (a as f64) + 0.9 * rng.gen::<f64>(),
                0.3,
                (b as f64) + 0.9 * rng.gen::<f64>(),
            );
            if (center - Vec3::new(4.0, 0.3, 0.0)).len() > 0.9 {
                let choose: f64 = rng.gen();
                let mat: Box<dyn Material> = if choose < 0.8 {
                    Lambertian::new(ConstantTexture::new(Color3::new(
                        rng.gen(),
                        rng.gen(),
                        rng.gen(),
                    )))
                } else if choose < 0.95 {
                    Metal::new(
                        Color3::new(
                            0.5 * (1.0 + rng.gen::<f64>()),
                            0.5 * (1.0 + rng.gen::<f64>()),
                            0.5 * (1.0 + rng.gen::<f64>()),
                        ),
                        0.5 * rng.gen::<f64>(),
                    )
                } else {
                    Dielectric::new(1.5)
                };
                let sphere = Sphere::new(center, 0.3, mat);
                world.push(sphere);
            }
        }
    }

    Box::new(world)
}
