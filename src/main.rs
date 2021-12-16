use geometry::*;
use canvas::*;
use geometry::sphere::*;
use tuples::*;
use geometry::rays::*;

mod tuples;
mod canvas;
mod matrices;
mod geometry;

fn main() {
    let mut canvas = Canvas::new(100, 100);
    let wall_size = 7.0;
    let pixel_size = wall_size / 100.0;
    let half = wall_size / 2.0;
    let sphere = Sphere::new(1.0, Tuple::point(0.0, 0.0, 0.0));
    let color = Color::new(1.0, 0.0, 0.0);
    let wall_z = 10.0;
    let ray_origin = Tuple::point(0.0, 0.0, -5.0);
    for y in 0..100 {
        let world_y = half - pixel_size * y as f64;
        for x in 0..100 {
            let world_x = -half + pixel_size * x as f64;
            let position = Tuple::point(world_x, world_y, wall_z);
            let ray = Ray::new(ray_origin, position.subtract(&ray_origin).normalize());
            let xs = ray.sphere_intersect(&sphere);
            if let Some(Intersection) = rays::hit(&xs) {
                canvas.write_pixel(x, y, color);
            }
        }
    }
    canvas.canvas_to_ppm("./image.ppm");
}