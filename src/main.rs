pub mod matrices;
pub mod scenes;
pub mod shapes;
pub mod surfaces;
pub mod rays;
pub mod utils;

mod prelude {
    pub use crate::matrices::{tuples::*, matrix4::*, matrix3::*, matrix2::*, transformations::*};
    pub use crate::surfaces::{patterns::*, materials::*, colors::*};
    pub use crate::scenes::{camera::*, lights::*, world::*, canvas::*};
    pub use crate::shapes::{cones::*, cubes::*, cylinders::*, objects::*, planes::*, spheres::*};
    pub use crate::shapes::{cones, cubes, cylinders, objects, planes, spheres};
    pub use crate::rays::*;
    pub use crate::utils::*;
    pub use crate::{matrices, scenes, shapes, surfaces, rays, utils};
    pub use std::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_2, FRAC_PI_4, PI, SQRT_2};
}

use crate::prelude::*;

fn main() {
    let mut floor = planes::new();
    floor.set_pattern(checker_3d(white(), black()));
    floor.set_reflective(0.15);

    let mut right_wall = planes::new();
    right_wall.set_pattern(solid(color(0.6, 0.9, 0.5)));
    right_wall.set_reflective(0.15);
    right_wall.set_transform(Matrix4::identity()
        .rotate_x(PI/2.0)
        .rotate_y(PI/4.0)
        .translate(0.0, 0.0, 5.0));

    let mut left_wall = planes::new();
    left_wall.set_pattern(solid(color(0.8, 0.8, 0.8)));
    left_wall.set_reflective(0.15);
    left_wall.set_transform(Matrix4::identity()
        .rotate_x(-PI/2.0)
        .rotate_y(-PI/4.0)
        .translate(0.0, 0.0, 5.0));

    let mut back_wall = planes::new();
    back_wall.set_pattern(checker_3d(white(), black()));
    back_wall.set_transform(Matrix4::identity()
        .rotate_x(PI/2.0)
        .translate(0.0, 0.0, 5.0));

    let mut ceiling = planes::new();
    ceiling.set_pattern(checker_3d(white(), color(0.6, 0.6, 0.9)));
    ceiling.set_reflective(0.0);
    ceiling.set_transform(translation(0.0, 13.0, 0.0));

    let mut middle = spheres::glass_sphere();
    middle.set_transform(translation(0.0, 1.0, 0.5));

    let mut right = cubes::new();
    right.set_transform(Matrix4::identity()
        .scale(0.5, 0.5, 0.5)
        .translate(1.5, 0.5, -0.5));
    right.set_pattern(solid(color(0.5, 1.0, 1.0)));
    right.set_diffuse(0.7);
    right.set_specular(0.3);
    right.set_shininess(300.0);
    right.set_reflective(0.0);

    let mut left = cylinders::new(0.0, 3.0, true);
    left.set_transform(Matrix4::identity()
        .scale(0.33, 0.33, 0.33)
        .translate(-1.5, 0.0, -0.75));
    left.set_pattern(solid(color(1.0, 0.8, 0.1)));
    left.set_diffuse(0.7);
    left.set_specular(0.3);
    left.set_reflective(0.04);

    let objects = vec![floor, ceiling, middle, right, left];

    let lights = vec![Light::new(point(-10.0, 10.0, -10.0), white())];

    let mut camera = Camera::new(1200, 800, PI/3.0);
    camera.set_transform(view_transform(
        point(0.0, 1.5, -5.0),
        point(0.0, 1.5, 0.0),
        vector(0.0, 1.0, 0.0)));

    let world = World::new(objects, lights);

    let image = camera.parallel_render(&world);

    image.canvas_to_png("image.png");
}
