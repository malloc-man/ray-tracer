use rayon::prelude::*;
use crate::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Camera {
    hsize: usize,
    vsize: usize,
    field_of_view: f64,
    transform: Matrix4,
    transform_components: [Tuple; 3],
    inverse_transform: Matrix4,
    pixel_size: f64,
    half_width: f64,
    half_height: f64,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Self {
        let mut new = Self {
            hsize,
            vsize,
            field_of_view,
            transform: Matrix4::identity(),
            inverse_transform: Matrix4::identity(),
            transform_components: [
                point(0.0, 0.0, 0.0),
                point(0.0, 0.0, -1.0),
                vector(0.0, 1.0, 0.0)
            ],
            pixel_size: 0.0,
            half_width: 0.0,
            half_height: 0.0,
        };
        new.initialize();
        new.update_transformations();
        new
    }

    pub fn new_preview(camera: &Camera) -> Self {
        let ratio = camera.hsize as f32 / camera.vsize as f32;
        let mut new = Self {
            hsize: (400.0 * ratio) as usize,
            vsize: 400,
            field_of_view: camera.field_of_view,
            transform: camera.transform,
            inverse_transform: camera.inverse_transform,
            transform_components: camera.transform_components,
            pixel_size: 0.0,
            half_width: 0.0,
            half_height: 0.0,
        };
        new.initialize();
        new.update_transformations();
        new
    }

    pub fn get_hsize(&self) -> usize {
        self.hsize
    }

    pub fn get_vsize(&self) -> usize {
        self.vsize
    }

    pub fn get_fov(&self) -> f64 {
        self.field_of_view
    }

    pub fn set_hsize(&mut self, hsize: usize) -> &mut Self {
        self.hsize = hsize;
        self.initialize();
        self
    }

    pub fn set_vsize(&mut self, vsize: usize) -> &mut Self {
        self.vsize = vsize;
        self.initialize();
        self
    }

    pub fn set_fov(&mut self, fov: f64) -> &mut Self {
        self.field_of_view = fov;
        self.initialize();
        self
    }

    fn initialize(&mut self) -> &mut Self {
        let half_view = (self.field_of_view / 2.0).tan();
        let aspect = self.hsize as f64 / self.vsize as f64;
        if aspect >= 1.0 {
            self.half_width = half_view;
            self.half_height = half_view / aspect;
        } else {
            self.half_width = half_view * aspect;
            self.half_height = half_view;
        }
        self.pixel_size = (self.half_width * 2.0) / self.hsize as f64;
        self
    }

    pub fn set_transform(&mut self, transform: Matrix4) -> &Self {
        self.transform = transform;
        self.inverse_transform = transform.invert();
        self
    }

    pub fn get_transform(&mut self) -> Matrix4 {
        self.transform
    }

    pub fn get_from(&self) -> Tuple {
        self.transform_components[0]
    }

    pub fn get_to(&self) -> Tuple {
        self.transform_components[1]
    }

    pub fn get_up(&self) -> Tuple {
        self.transform_components[2]
    }

    pub fn set_from(&mut self, from: Tuple) {
        self.transform_components[0] = from;
        self.update_transformations();
    }

    pub fn set_to(&mut self, to: Tuple) {
        self.transform_components[1] = to;
        self.update_transformations();
    }

    pub fn set_up(&mut self, up: Tuple) {
        self.transform_components[2] = up;
        self.update_transformations();
    }

    fn update_transformations(&mut self) {
        let from = self.transform_components[0];
        let to = self.transform_components[1];
        let up = self.transform_components[2];

        let transform = view_transform(from, to, up);
        self.set_transform(transform);
    }

    fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        let xoffset = (x as f64 + 0.5) * self.pixel_size;
        let yoffset = (y as f64 + 0.5) * self.pixel_size;
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let pixel = self.inverse_transform * point(world_x, world_y, -1.0);
        let origin = self.inverse_transform * origin();
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);
        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(ray, DEFAULT_RECURSION_DEPTH);
                image.write_pixel(x, y, color);
            }
        }
        image
    }

    pub fn parallel_render(&self, world: Arc<RwLock<World>>, tracker: Arc<AtomicUsize>) -> Canvas {
        let world = world.read().unwrap();

        tracker.store(0, Ordering::Relaxed);

        println!("Beginning render...");

        const BAND_SIZE: usize = 10;
        let mut image = Canvas::new(self.hsize, self.vsize);

        println!("Rendering image: {} x {}", self.hsize, self.vsize);
        image
            .pixels()
            .par_chunks_mut(self.hsize * BAND_SIZE)
            .enumerate()
            .for_each(|(i, band)| {
                for row in 0..BAND_SIZE {
                    for col in 0..self.hsize {
                        let ray = self.ray_for_pixel(col, row + i * BAND_SIZE);
                        if (row * self.hsize) + col < band.len() {
                            band[(row * self.hsize) + col] =
                                world.color_at(ray, DEFAULT_RECURSION_DEPTH);
                            tracker.fetch_add(1, Ordering::SeqCst);
                        }
                    }
                    print!("\rRender progress: {:.1}%",
                           tracker.load(Ordering::Relaxed) as f64 * 100.0 /
                               ((self.hsize * self.vsize) as f64));
                }
            });
        image
    }

    pub fn preview_parallel_render(&self, world: RwLockReadGuard<World>) -> Canvas {
        const BAND_SIZE: usize = 10;
        let mut image = Canvas::new(self.hsize, self.vsize);
        image
            .pixels()
            .par_chunks_mut(self.hsize * BAND_SIZE)
            .enumerate()
            .for_each(|(i, band)| {
                for row in 0..BAND_SIZE {
                    for col in 0..self.hsize {
                        let ray = self.ray_for_pixel(col, row + i * BAND_SIZE);
                        if (row * self.hsize) + col < band.len() {
                            band[(row * self.hsize) + col] =
                                world.color_at(ray, 2);
                        }
                    }
                }
            });
        image
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pixel_size_horizontal_canvas() {
        let c = Camera::new(200, 125, FRAC_PI_2);
        assert!(c.pixel_size.approx_eq(0.01));
    }

    #[test]
    fn test_pixel_size_vertical_canvas() {
        let c = Camera::new(125, 200, FRAC_PI_2);
        assert!(c.pixel_size.approx_eq(0.01));
    }

    #[test]
    fn test_ray_for_pixel_center() {
        let mut c = Camera::new(201, 101, FRAC_PI_2);
        let r = c.ray_for_pixel(100, 50);

        assert_eq!(r.get_origin(), origin());
        assert_eq!(r.get_direction(), vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_ray_for_pixel_corner() {
        let c = Camera::new(201, 101, FRAC_PI_2);
        let r = c.ray_for_pixel(0, 0);

        assert_eq!(r.get_origin(), origin());
        assert_eq!(r.get_direction(), vector(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn test_ray_for_pixel_transformed() {
        let mut c = Camera::new(201, 101, FRAC_PI_2);
        c.set_transform(
            Matrix4::identity()
                .translate(0.0, -2.0, 5.0)
                .rotate_y(FRAC_PI_4)
        );

        let r = c.ray_for_pixel(100, 50);

        assert_eq!(r.get_origin(), point(0.0, 2.0, -5.0));
        assert_eq!(r.get_direction(), vector(FRAC_1_SQRT_2, 0.0, -FRAC_1_SQRT_2));
    }

    #[test]
    fn test_render_world() {
        let w = World::new_default();
        let mut c = Camera::new(11, 11, FRAC_PI_2);

        let from = point(0.0, 0.0, -5.0);
        let to = point(0.0, 0.0, 0.0);
        let up = vector(0.0, 1.0, 0.0);

        c.set_transform(view_transform(from, to, up));

        let image = c.render(&w);
        assert_eq!(image.pixel_at(5, 5), color(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn test_parallel_render_world() {
        let w = Arc::new(RwLock::new(World::new_default()));
        let mut c = Camera::new(11, 11, FRAC_PI_2);

        let from = point(0.0, 0.0, -5.0);
        let to = point(0.0, 0.0, 0.0);
        let up = vector(0.0, 1.0, 0.0);

        c.set_transform(view_transform(from, to, up));

        let image = c.parallel_render(w, Arc::new(AtomicUsize::new(0)));
        assert_eq!(image.pixel_at(5, 5), color(0.38066, 0.47583, 0.2855));
    }
}