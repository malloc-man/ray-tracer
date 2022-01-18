pub mod matrices;
pub mod scenes;
pub mod shapes;
pub mod surfaces;
pub mod rays;
pub mod utils;
mod app;

mod prelude {
    pub use crate::matrices::{
        tuples::*,
        matrix4::*,
        matrix3::*,
        matrix2::*,
        transformations::*
    };
    pub use crate::surfaces::{
        patterns::*,
        materials::*,
        colors::*
    };
    pub use crate::scenes::{
        camera::*,
        lights::*,
        world::*,
        canvas::*
    };
    pub use crate::shapes::{
        cones::*,
        cubes::*,
        cylinders::*,
        objects::*,
        planes::*,
        spheres::*,
        groups::*,
        objectholders::*,
        traits::*,
    };
    pub use crate::shapes::{
        cones,
        cubes,
        cylinders,
        objects,
        planes,
        spheres,
        groups,
        objectholders,
        traits,
    };
    pub use crate::rays::*;
    pub use crate::utils::*;
    pub use crate::{
        matrices,
        scenes,
        shapes,
        surfaces,
        rays,
        utils
    };
    pub use std::f64::consts::{
        FRAC_1_SQRT_2,
        FRAC_PI_2,
        FRAC_PI_4,
        PI,
        SQRT_2
    };
    pub use std::sync::atomic::{
        AtomicUsize,
        Ordering
    };
    pub use std::sync::{
        Arc,
        RwLock,
        RwLockReadGuard,
        Weak,
    };
}

use eframe::*;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app = crate::app::RayTracer::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
