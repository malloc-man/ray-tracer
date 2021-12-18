use std::{error::Error, f64::consts::PI};
use {matrices::*, shapes::*};
use matrices::tuples::*;
use scenes::{camera::*, lights::*, world::*};
use surfaces::{materials::*, colors::*, patterns::*};
use crate::{matrix4::*, rays::*};
use crate::transformations::{rotation_y, view_transform};

mod matrices;
mod shapes;
mod scenes;
pub mod rays;
mod surfaces;


fn main() -> Result<(), Box<dyn Error>> {
    let mut floor = planes::new();
    floor.set_color(color(1.0, 0.9, 0.9));
    floor.set_specular(0.0);
    floor.set_pattern(Pattern::stripe_pattern(color(0.8, 0.1, 0.1), white()));
    floor.set_pattern_transform(rotation_y(0.4));

    let mut middle = spheres::new();
    middle.set_transform(Matrix4::identity()
        .translate(-0.5, 1.0, 0.5));
    middle.set_color(color(1.0, 0.1, 0.5));
    middle.set_diffuse(0.7);
    middle.set_specular(0.3);
    middle.set_pattern(Pattern::stripe_pattern(white(), black()));
    middle.set_pattern_transform(Matrix4::identity()
        .scale(0.25, 0.25, 0.25)
        .rotate_y(0.5)
        .rotate_z(0.37)
    );

    let mut right = spheres::new();
    right.set_transform(Matrix4::identity()
        .scale(0.5, 0.5, 0.5)
        .translate(1.5, 0.5, -0.5));
    right.set_color(color(0.5, 1.0, 0.1));
    right.set_diffuse(0.7);
    right.set_specular(0.3);

    let mut left = spheres::new();
    left.set_transform(Matrix4::identity()
        .scale(0.33, 0.33, 0.33)
        .translate(-1.5, 0.33, -0.75));
    left.set_color(color(1.0, 0.8, 0.1));
    left.set_diffuse(0.7);
    left.set_specular(0.3);

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
