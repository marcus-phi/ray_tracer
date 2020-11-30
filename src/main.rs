use ray_tracer::camera::PerspectiveCamera;
use ray_tracer::hit::HitableList;
use ray_tracer::material::Lambertian;
use ray_tracer::math::Color3;
use ray_tracer::math::Vec3;
use ray_tracer::render;
use ray_tracer::shape::Sphere;
use ray_tracer::texture::ConstantTexture;

fn main() {
    let mut world = HitableList::new();

    let gray_tex = ConstantTexture::new(Color3::new(0.7, 0.7, 0.7));
    let red_tex = ConstantTexture::new(Color3::new(1.0, 0.0, 0.0));
    let green_tex = ConstantTexture::new(Color3::new(0.0, 1.0, 0.0));

    let gray_mat = Lambertian::new(&gray_tex);
    let red_mat = Lambertian::new(&red_tex);
    let green_mat = Lambertian::new(&green_tex);

    let ground = Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, &gray_mat);
    let red = Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, &red_mat);
    let green = Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, &green_mat);

    world.push(&ground);
    world.push(&red);
    world.push(&green);

    let image_dim = (720, 480);

    let camera = PerspectiveCamera::new(
        Vec3::new(13.0, 2.0, 3.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        (image_dim.0 as f64) / (image_dim.1 as f64),
        0.05,
        10.0,
    );

    render(&world, &camera, image_dim, 64);
}
