use geometry::*;
use canvas::*;
use colors::Color;
use geometry::sphere::*;
use tuples::*;
use geometry::rays::*;
use matrices::*;
use crate::lights::*;
use crate::materials::Material;

mod tuples;
mod canvas;
mod matrices;
mod geometry;
mod lights;
mod materials;
mod colors;

fn main() {
    let mut canvas = Canvas::new(100, 100);
    let wall_size = 7.0;
    let pixel_size = wall_size / 100.0;
    let half = wall_size / 2.0;
    let mut sphere = Sphere::new(1.0, Tuple::point(0.0, 0.0, 0.0));
    sphere.material.color = Color::new(1.0, 0.2, 1.0);
    let color = Color::new(1.0, 0.0, 0.0);
    let wall_z = 10.0;
    let ray_origin = Tuple::point(0.0, 0.0, -5.0);
    let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    for y in 0..100 {
        let world_y = half - pixel_size * y as f64;
        for x in 0..100 {
            let world_x = -half + pixel_size * x as f64;
            let position = Tuple::point(world_x, world_y, wall_z);
            let ray = Ray::new(ray_origin, position.subtract(&ray_origin).normalize());
            let xs = ray.sphere_intersect(&sphere);
            if let Some(Intersection) = rays::hit(&xs) {
                let point = ray.position(Intersection.t_value);
                let normal = sphere.normal_at(point);
                let eye = ray.direction.negate();
                let new_color = lighting(&Intersection.object.material, &point, &light,&eye, &normal);
                canvas.write_pixel(x, y, new_color);
            }
        }
    }
    canvas.canvas_to_ppm("./image.ppm");
}