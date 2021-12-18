use std::error::Error;
use std::f64::consts::PI;
use {matrices::*, shapes::*};
use matrices::tuples::*;
use rays::*;
use scenes::camera::*;
use scenes::lights::*;
use scenes::world::*;
use surfaces::colors::*;
use surfaces::materials::*;
use crate::matrix4::Matrix4;
use crate::transformations::view_transform;

mod matrices;
mod shapes;
mod scenes;
pub mod rays;
mod surfaces;


fn main() -> Result<(), Box<dyn Error>> {
    let mut floor = planes::new();
    floor.set_color(color(1.0, 0.9, 0.9));
    floor.get_material().set_specular(0.0);

    let mut middle = spheres::new();
    middle.set_transform(Matrix4::identity()
        .translate(-0.5, 1.0, 0.5));
    middle.set_color(color(0.1, 1.0, 0.5));
    middle.get_material().set_diffuse(0.7);
    middle.get_material().set_specular(0.3);

    let mut right = spheres::new();
    right.set_transform(Matrix4::identity()
        .scale(0.5, 0.5, 0.5)
        .translate(1.5, 0.5, -0.5));
    right.set_color(color(0.5, 1.0, 0.1));
    right.get_material().set_diffuse(0.7);
    right.get_material().set_specular(0.3);

    let mut left = spheres::new();
    left.set_transform(Matrix4::identity()
        .scale(0.33, 0.33, 0.33)
        .translate(-1.5, 0.33, -0.75));
    left.set_color(color(1.0, 0.8, 0.1));
    left.get_material().set_diffuse(0.7);
    left.get_material().set_specular(0.3);

    let objects = vec![floor, middle, right, left];

    let lights = vec![Light::new(point(-10.0, 10.0, -10.0), white())];

    let mut camera = Camera::new(1024, 768, PI/3.0);
    camera.set_transform(view_transform(
        point(0.0, 1.5, -5.0),
        point(0.0, 1.0, 0.0),
        vector(0.0, 1.0, 0.0)));

    let world = World::new(objects, lights);

    let image = camera.render(&world);

    image.canvas_to_ppm("./image.ppm")?;

    Ok(())
}
