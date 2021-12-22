use std::{error::Error, f64::consts::PI};
use {matrices::*, shapes::*};
use matrices::tuples::*;
use scenes::{camera::*, lights::*, world::*};
use surfaces::{materials::*, colors::*, patterns::*};
use crate::{matrix4::*, rays::*};
use surfaces::patterns::checker_3d;
use crate::transformations::*;

mod matrices;
mod shapes;
mod scenes;
pub mod rays;
mod surfaces;


fn main() -> Result<(), Box<dyn Error>> {
    let mut floor = planes::new();
    floor.set_pattern(checker_3d(white(), black()));
    floor.set_reflective(0.15);

    let mut right_wall = planes::new();
    right_wall.set_pattern(checker_3d(color(0.0, 0.0, 0.8), black()));
    right_wall.set_reflective(0.15);
    right_wall.set_transform(Matrix4::identity()
        .rotate_x(PI/2.0)
        .rotate_y(PI/4.0)
        .translate(0.0, 0.0, 5.0));

    let mut left_wall = planes::new();
    left_wall.set_pattern(checker_3d(color(0.8, 0.0, 0.0), black()));
    left_wall.set_reflective(0.15);
    left_wall.set_transform(Matrix4::identity()
        .rotate_x(-PI/2.0)
        .rotate_y(-PI/4.0)
        .translate(0.0, 0.0, 5.0));

    let mut ceiling = planes::new();
    ceiling.set_pattern(checker_3d(white(), black()));
    ceiling.set_reflective(0.3);
    ceiling.set_transform(translation(0.0, 13.0, 0.0));

    let mut middle = spheres::glass_sphere();
    middle.set_transform(translation(0.0, 1.5, 0.5));

    let mut right = spheres::new();
    right.set_transform(Matrix4::identity()
        .scale(0.5, 0.5, 0.5)
        .translate(1.5, 0.5, -0.5));
    right.set_pattern(solid(color(0.5, 1.0, 1.0)));
    right.set_diffuse(0.7);
    right.set_specular(0.3);
    right.set_reflective(0.04);

    let mut left = spheres::new();
    left.set_transform(Matrix4::identity()
        .scale(0.33, 0.33, 0.33)
        .translate(-1.5, 0.33, -0.75));
    left.set_pattern(solid(color(1.0, 0.8, 0.1)));
    left.set_diffuse(0.7);
    left.set_specular(0.3);
    left.set_reflective(0.04);

    let objects = vec![floor, right_wall, left_wall, ceiling, middle, right, left];

    let lights = vec![Light::new(point(-10.0, 10.0, -10.0), white())];

    let mut camera = Camera::new(1000, 1000, PI/3.0);
    camera.set_transform(view_transform(
        point(0.0, 1.5, -5.0),
        point(0.0, 1.0, 0.0),
        vector(0.0, 1.0, 0.0)));

    let world = World::new(objects, lights);

    let image = camera.parallel_render(&world);

    image.canvas_to_ppm("./image.ppm")?;

    Ok(())
}
