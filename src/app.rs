use eframe::{egui, epi};
use crate::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use crate::egui::Align;

pub struct RayTracer {
    world: Arc<RwLock<World>>,
    camera: Camera,
    active_object: Option<usize>,
    last_preview: Option<(egui::Vec2, egui::TextureId)>,
    preview_up_to_date: Arc<AtomicBool>,
    rendering: Arc<AtomicBool>,
    rendering_progress: Arc<AtomicUsize>,
    preview_image: Arc<RwLock<Option<epi::Image>>>,
    preview_camera: Arc<RwLock<Camera>>,
}

impl Default for RayTracer {
    fn default() -> Self {
        let mut new = Self {
            world: Arc::new(
                RwLock::new(
                    World::new_default()
                )
            ),
            camera: Camera::new(1000, 800, FRAC_PI_2),
            active_object: None,
            last_preview: None,
            preview_up_to_date: Arc::new(
                AtomicBool::new(
                    false
                )
            ),
            rendering: Arc::new(
                AtomicBool::new(
                    false
                )
            ),
            rendering_progress: Arc::new(
                AtomicUsize::new(0
                )
            ),
            preview_image: Arc::new(
                RwLock::new(
                    None
                )
            ),
            preview_camera: Arc::new(
                RwLock::new(
                    Camera::new_preview(
                        &Camera::new(1000, 800, FRAC_PI_2)
                    )
                )
            ),
        };

        new.camera.set_from(point(0.0, 1.5, -5.0));
        new.camera.set_to(point(0.0, 1.5, 0.0));
        new.camera.set_up(vector(0.0, 1.0, 0.0));

        let mut preview = new.preview_camera.clone();
        let mut prev = preview.write().unwrap();
        prev.set_from(point(0.0, 1.5, -5.0));
        prev.set_to(point(0.0, 1.5, 0.0));
        prev.set_up(vector(0.0, 1.0, 0.0));

        new
    }
}

impl epi::App for RayTracer {
    fn name(&self) -> &str {
        "Ray Tracer"
    }

    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        let world = self.world.clone();
        let stop = self.preview_up_to_date.clone();
        let img_arc = self.preview_image.clone();
        let cam_arc = self.preview_camera.clone();

        std::thread::spawn(move || {
            loop {
                if !stop.load(Ordering::Relaxed) {
                    let world_lock = world.try_read();
                    if let Ok(world) = world_lock {
                        let cam = Camera::new_preview(&cam_arc.read().unwrap());
                        let buffer = cam.preview_parallel_render(world).canvas_to_buffer();
                        let size = [buffer.width() as usize, buffer.height() as usize];
                        let pixels = buffer.into_vec();
                        let image = epi::Image::from_rgba_unmultiplied(size, &pixels);
                        img_arc.write().unwrap().insert(image);
                        stop.store(true, Ordering::Relaxed);
                    }
                }
            }
        });
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        self.update_preview(ctx, frame);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
                ui.menu_button("Edit", |ui| {

                });
                ui.menu_button("View", |ui| {

                });
                ui.menu_button("World", |ui| {
                    ui.menu_button("New Object...", |ui| {
                        if ui.button("Cone").clicked() {
                            self.add_new_shape(Shape::Cone {min: 0.0, max: 1.0, closed: true});
                        }
                        if ui.button("Cube").clicked() {
                            self.add_new_shape(Shape::Cube);
                        }
                        if ui.button("Cylinder").clicked() {
                            self.add_new_shape(Shape::Cylinder {min: 0.0, max: 1.0, closed: true});
                        }
                        if ui.button("Plane").clicked() {
                            self.add_new_shape(Shape::Plane);
                        }
                        if ui.button("Sphere").clicked() {
                            self.add_new_shape(Shape::Sphere);
                        }
                    });
                    ui.group(|ui| {
                        ui.set_enabled(self.active_object.is_some());
                        if ui.button("Delete active object").clicked() {
                            self.delete_active_object();
                        };
                    })
                });
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.add_enabled(
                    !self.rendering.load(Ordering::Relaxed),
                    egui::Button::new("Render")
                ).clicked() {
                    let wd = self.world.clone();
                    let cmra = self.camera;
                    let arc_rendering_state = self.rendering.clone();
                    let arc_progress = self.rendering_progress.clone();
                    std::thread::spawn(move || {
                        arc_rendering_state.store(true, Ordering::SeqCst);
                        let canvas = cmra.parallel_render(wd, arc_progress);
                        canvas.canvas_to_png("image.png");
                        arc_rendering_state.store(false, Ordering::SeqCst);
                    });
                };

                if self.rendering.load(Ordering::SeqCst) {
                    let progress = self.rendering_progress.load(Ordering::SeqCst) as f32 /
                        (self.camera.get_vsize() * self.camera.get_vsize()) as f32;
                    ui.horizontal(|ui| {
                        ui.add(
                            egui::ProgressBar::new(progress)
                                .show_percentage()
                                .animate(true)
                        );
                    });
                };
            });
        });

        egui::SidePanel::right("side_panel").show(ctx, |ui| {

            // Camera pane
            ui.with_layout(egui::Layout::top_down_justified(Align::Center),|ui| {
                ui.label("Camera");

                ui.horizontal(|ui| {
                    let mut hsize = self.camera.get_hsize();
                    if ui.add(egui::DragValue::new(&mut hsize).speed(1)).changed() {
                        self.prep_update();
                        self.camera.set_hsize(hsize);
                        let mut prev = self.preview_camera.write().unwrap();
                        *prev = Camera::new_preview(&self.camera);
                    }
                    ui.label("x");

                    let mut vsize = self.camera.get_vsize();
                    if ui.add(egui::DragValue::new(&mut vsize).speed(1)).changed() {
                        self.prep_update();
                        self.camera.set_vsize(vsize);
                        let mut prev = self.preview_camera.write().unwrap();
                        *prev = Camera::new_preview(&self.camera);
                    }
                    ui.label("y");

                    let mut fov = self.camera.get_fov().to_degrees();
                    if ui.add(egui::DragValue::new(&mut fov).speed(0.5)).changed() {
                        self.prep_update();
                        self.camera.set_fov(fov.to_radians());
                        let mut prev = self.preview_camera.write().unwrap();
                        *prev = Camera::new_preview(&self.camera);
                    }
                    ui.label("FOV");
                });

                ui.add_space(4.0);

                ui.horizontal(|ui| {
                    let mut from = self.camera.get_from();

                    if ui.add(egui::DragValue::new(&mut from.x).speed(0.1)).changed() {
                        self.prep_update();
                        self.camera.set_from(from);
                        let mut prev = self.preview_camera.write().unwrap();
                        *prev = Camera::new_preview(&self.camera);
                    }
                    ui.label("x");
                    if ui.add(egui::DragValue::new(&mut from.y).speed(0.1)).changed() {
                        self.prep_update();
                        self.camera.set_from(from);
                        let mut prev = self.preview_camera.write().unwrap();
                        *prev = Camera::new_preview(&self.camera);
                    }
                    ui.label("y");
                    if ui.add(egui::DragValue::new(&mut from.z).speed(0.1)).changed() {
                        self.prep_update();
                        self.camera.set_from(from);
                        let mut prev = self.preview_camera.write().unwrap();
                        *prev = Camera::new_preview(&self.camera);
                    }
                    ui.label("z");
                    ui.label("|   From");
                });

                ui.horizontal(|ui| {
                    let mut to = self.camera.get_to();
                    if ui.add(egui::DragValue::new(&mut to.x).speed(0.1)).changed() {
                        self.prep_update();
                        self.camera.set_to(to);
                        let mut prev = self.preview_camera.write().unwrap();
                        *prev = Camera::new_preview(&self.camera);
                    }
                    ui.label("x");
                    if ui.add(egui::DragValue::new(&mut to.y).speed(0.1)).changed() {
                        self.prep_update();
                        self.camera.set_to(to);
                        let mut prev = self.preview_camera.write().unwrap();
                        *prev = Camera::new_preview(&self.camera);
                    }
                    ui.label("y");
                    if ui.add(egui::DragValue::new(&mut to.z).speed(0.1)).changed() {
                        self.prep_update();
                        self.camera.set_to(to);
                        let mut prev = self.preview_camera.write().unwrap();
                        *prev = Camera::new_preview(&self.camera);
                    }
                    ui.label("z");
                    ui.label("|   To");
                });

                ui.horizontal(|ui| {
                    let mut up = self.camera.get_up();
                    if ui.add(egui::DragValue::new(&mut up.x).speed(0.1)).changed() {
                        self.prep_update();
                        self.camera.set_up(up);
                        let mut prev = self.preview_camera.write().unwrap();
                        *prev = Camera::new_preview(&self.camera);
                    }
                    ui.label("x");
                    if ui.add(egui::DragValue::new(&mut up.y).speed(0.1)).changed() {
                        self.prep_update();
                        self.camera.set_up(up);
                        let mut prev = self.preview_camera.write().unwrap();
                        *prev = Camera::new_preview(&self.camera);
                    }
                    ui.label("y");
                    if ui.add(egui::DragValue::new(&mut up.z).speed(0.1)).changed() {
                        self.prep_update();
                        self.camera.set_up(up);
                        let mut prev = self.preview_camera.write().unwrap();
                        *prev = Camera::new_preview(&self.camera);
                    }
                    ui.label("z");
                    ui.label("|   Up");
                });

            });

            ui.add(egui::Separator::default());

            // Materials pane
            ui.with_layout(egui::Layout::top_down_justified(Align::Center), |ui| {
                ui.set_enabled(self.active_object.is_some());
                let index = self.active_object.unwrap_or(usize::MAX);
                ui.label("Material attributes");
                ui.horizontal(|ui| {
                    if index == usize::MAX {
                        ui.color_edit_button_rgb(&mut [1.0, 1.0, 1.0]);
                        ui.label("Base Color");
                    } else {
                        ui.set_enabled(self.world.read().unwrap().read_objects()[self.active_object.unwrap()].get_pattern().get_pattern_type() == PatternType::Solid);
                        let orig = self.world.read().unwrap().read_objects()[self.active_object.unwrap()].get_color();
                        let mut color = [
                            orig.get_red() as f32,
                            orig.get_green() as f32,
                            orig.get_blue() as f32
                        ];
                        if ui.color_edit_button_rgb(&mut color).changed() {
                            self.world.write().unwrap().objects()[self.active_object.unwrap()].set_color(
                                surfaces::colors::color(
                                    color[0] as f64,
                                    color[1] as f64,
                                    color[2] as f64
                                )
                            );
                            self.prep_update();
                        };
                        ui.label("Base Color");
                    };
                });

                if index == usize::MAX {
                    egui::ComboBox::from_label("Pattern")
                        .selected_text("Pattern")
                        .show_ui(ui, |ui| {});
                } else {
                    let mut pattern = self.world.read().unwrap().read_objects()[self.active_object.unwrap()].get_pattern().get_pattern_type();
                    let orig = pattern;
                    egui::ComboBox::from_label("Pattern")
                        .selected_text(format!("{}", self.world.read().unwrap().read_objects()[self.active_object.unwrap()].get_pattern()))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut pattern, PatternType::Solid, "Solid Color");
                            ui.selectable_value(&mut pattern, PatternType::Stripe, "Stripe");
                            ui.selectable_value(&mut pattern, PatternType::Gradient, "Gradient");
                            ui.selectable_value(&mut pattern, PatternType::Ring, "Ring");
                            ui.selectable_value(&mut pattern, PatternType::Checker3d, "Checkers");
                        });
                    if pattern != orig {
                        let new_pattern = self.world.read().unwrap().read_objects()[self.active_object.unwrap()].get_pattern().duplicate_different_type(pattern);
                        self.world.write().unwrap().objects()[self.active_object.unwrap()].set_pattern(new_pattern);
                        self.prep_update();
                    }
                }

                ui.horizontal(|ui| {
                    if index != usize::MAX {
                        let ptrn = self.world.read().unwrap().read_objects()[self.active_object.unwrap()].get_pattern();
                        if ptrn.get_pattern_type() != PatternType::Solid {
                            let clrs = ptrn.colors();
                            let mut color1 = [
                                clrs[0].get_red() as f32,
                                clrs[0].get_green() as f32,
                                clrs[0].get_blue() as f32,
                            ];

                            let mut color2 = [
                                clrs[1].get_red() as f32,
                                clrs[1].get_green() as f32,
                                clrs[1].get_blue() as f32,
                            ];
                            ui.horizontal(|ui| {
                                if ui.color_edit_button_rgb(&mut color1).changed() {
                                    let new_color = color(
                                        color1[0] as f64,
                                        color1[1] as f64,
                                        color1[2] as f64,
                                    );
                                    self.world.write().unwrap().objects()[self.active_object.unwrap()].set_pattern(
                                        ptrn.duplicate_change_color_1(new_color)
                                    );
                                    self.prep_update();
                                }
                                ui.label("Color 1");
                            });

                            ui.horizontal(|ui| {
                                if ui.color_edit_button_rgb(&mut color2).changed() {
                                    let new_color = color(
                                        color2[0] as f64,
                                        color2[1] as f64,
                                        color2[2] as f64,
                                    );
                                    self.world.write().unwrap().objects()[self.active_object.unwrap()].set_pattern(
                                        ptrn.duplicate_change_color_2(new_color)
                                    );
                                    self.prep_update();
                                }
                                ui.label("Color 2");
                            });
                        }
                    }
                });

                ui.horizontal(|ui| {
                    self.material_attribute_slider(0, ui, index != usize::MAX);
                });

                ui.horizontal(|ui| {
                    self.material_attribute_slider(1, ui, index != usize::MAX);
                });

                ui.horizontal(|ui| {
                    self.material_attribute_slider(2, ui, index != usize::MAX);
                });

                ui.horizontal(|ui| {
                    self.material_attribute_slider(3, ui, index != usize::MAX);
                });

                ui.horizontal(|ui| {
                    self.material_attribute_slider(4, ui, index != usize::MAX);
                });

                ui.horizontal(|ui| {
                    self.material_attribute_slider(5, ui, index != usize::MAX);
                });

                ui.horizontal(|ui| {
                    self.material_attribute_slider(6, ui, index != usize::MAX);
                });

                ui.horizontal(|ui| {
                    if index == usize::MAX {
                        ui.checkbox(&mut false, "Casts shadow");
                    } else {
                        let mut casts_shadow = self.world.read().unwrap().read_objects()[self.active_object.unwrap()].casts_shadow();
                        if ui.checkbox(&mut casts_shadow, "Casts shadow").changed() {
                            self.world.write().unwrap().objects()[self.active_object.unwrap()].set_casts_shadow(casts_shadow);
                            self.prep_update();
                        }
                    }
                });
            });

            // Interface for shape-specific attributes (min, max, end caps)
            if self.active_object.is_some() {
                let curr_shape = self.world.read().unwrap().read_objects()[self.active_object.unwrap()].shape;
                match curr_shape {
                    Shape::Cone {min, max, closed} => self.shape_specific_interface(min, max, closed, ui),
                    Shape::Cylinder {min, max, closed} => self.shape_specific_interface(min, max, closed, ui),
                    _ => ()
                };
            }

            ui.add(egui::Separator::default());

            // Transformations
            ui.with_layout(egui::Layout::top_down_justified(Align::Center), |ui| {
                ui.set_enabled(self.active_object.is_some());
                let index = self.active_object.unwrap_or(usize::MAX);
                ui.label("Object transformations");
                ui.horizontal(|ui| {
                    ui.group(|ui| {
                        self.transformation_drag_updater(0, ui, index != usize::MAX);
                        self.transformation_drag_updater(1, ui, index != usize::MAX);
                        self.transformation_drag_updater(2, ui, index != usize::MAX);
                    });
                    ui.label("Translation");
                });

                ui.horizontal(|ui| {
                    ui.group(|ui| {
                        self.transformation_drag_updater(3, ui, index != usize::MAX);
                        self.transformation_drag_updater(4, ui, index != usize::MAX);
                        self.transformation_drag_updater(5, ui, index != usize::MAX);
                    });
                    ui.label("Scaling");
                });

                ui.horizontal(|ui| {
                    ui.group(|ui| {
                        self.transformation_drag_updater(6, ui, index != usize::MAX);
                        self.transformation_drag_updater(7, ui, index != usize::MAX);
                        self.transformation_drag_updater(8, ui, index != usize::MAX);
                    });
                    ui.label("Rotation");
                });

                ui.horizontal(|ui| {
                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                self.transformation_drag_updater(9, ui, index != usize::MAX);
                                self.transformation_drag_updater(10, ui, index != usize::MAX);
                                self.transformation_drag_updater(11, ui, index != usize::MAX);
                            });

                            ui.horizontal(|ui| {
                                self.transformation_drag_updater(12, ui, index != usize::MAX);
                                self.transformation_drag_updater(13, ui, index != usize::MAX);
                                self.transformation_drag_updater(14, ui, index != usize::MAX);
                            });
                        });
                    });
                    ui.label("Shearing");
                });
            });
        });

        egui::SidePanel::right("object_list").show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                let mut idx = 0;
                for item in self.world.read().unwrap().read_objects() {
                    ui.selectable_value(
                        &mut self.active_object,
                        Some(idx),
                        format!("{}", item.shape)
                    );
                    idx += 1;
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::new([true, true]).show(ui, |ui| {
                if let Some((size, texture)) = self.last_preview {
                    ui.image(texture, size);
                }
            });
            egui::warn_if_debug_build(ui);
        });
    }
}

impl RayTracer {
    fn delete_active_object(&mut self) {
        if let Some(index) = self.active_object {
            self.world.write().unwrap().objects().remove(index);
            match index {
                0 => self.active_object = None,
                _ => self.active_object = Some(index-1),
            }
            self.prep_update();
        }
    }

    fn update_preview(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        let curr_preview = self.preview_image.try_read();
        if let Ok(image_lock) = curr_preview {
            if image_lock.is_some() {
                let image = image_lock.clone().unwrap();
                let x = image.size[0] as f32;
                let y = image.size[1] as f32;
                let texture = frame.alloc_texture(image);
                let size = egui::Vec2::new(x, y);
                self.last_preview = Some((size, texture));
                ctx.request_repaint();
                frame.free_texture(texture);
            }
        }
    }

    fn prep_update(&mut self) {
        self.preview_up_to_date.store(false, Ordering::Relaxed);
    }

    fn add_new_shape(&mut self, shape: Shape) {
        let new = match shape {
            Shape::Cone {min, max, closed} => cones::new(min, max, closed),
            Shape::Cube => cubes::new(),
            Shape::Cylinder {min, max, closed} => cylinders::new(min, max, closed),
            Shape::Plane => planes::new(),
            Shape::Sphere => spheres::new(),
        };
        self.world.write().unwrap().add_object(new);
        self.active_object = Some(self.world.read().unwrap().read_objects().len()-1);
        self.prep_update();
    }

    fn material_attribute_slider(&mut self, index: u8, ui: &mut egui::Ui, enabled: bool) {
        if enabled {
            let (mut orig, name, rng) = self.get_scalar_attribute_from_index(index);
            if ui.add(egui::Slider::new(&mut orig, rng).text(name)).changed() {
                self.set_scalar_attribute_from_index(index, orig);
                self.prep_update();
            }
        } else {
            let (name, rng) = self.scalar_attribute_spec(index);
            ui.add(egui::Slider::new(&mut 0.0, rng).text(name));
        }
    }

    fn get_scalar_attribute_from_index(&self, index: u8) -> (f64, &str, std::ops::RangeInclusive<f64>) {
        let (name, rng) = self.scalar_attribute_spec(index);
        match index {
            0 => (self.world.read().unwrap().read_objects()[self.active_object.unwrap()].get_ambient(), name, rng),
            1 => (self.world.read().unwrap().read_objects()[self.active_object.unwrap()].get_diffuse(), name, rng),
            2 => (self.world.read().unwrap().read_objects()[self.active_object.unwrap()].get_specular(), name, rng),
            3 => (self.world.read().unwrap().read_objects()[self.active_object.unwrap()].get_shininess(), name, rng),
            4 => (self.world.read().unwrap().read_objects()[self.active_object.unwrap()].get_reflective(), name, rng),
            5 => (self.world.read().unwrap().read_objects()[self.active_object.unwrap()].get_refractive_index(), name, rng),
            6 => (self.world.read().unwrap().read_objects()[self.active_object.unwrap()].get_transparency(), name, rng),
            _ => (0.0, "", 0.0..=0.0)
        }
    }

    fn scalar_attribute_spec(&self, index: u8) -> (&str, std::ops::RangeInclusive<f64>) {
        match index {
            0 => ("Ambient", 0.0..=1.0),
            1 => ("Diffuse", 0.0..=1.0),
            2 => ("Specular", 0.0..=1.0),
            3 => ("Shininess", 0.0..=400.0),
            4 => ("Reflective", 0.0..=1.0),
            5 => ("Refractive Index", 0.0..=5.0),
            6 => ("Transparency", 0.0..=1.0),
            _ => ("", 0.0..=0.0)
        }
    }

    fn set_scalar_attribute_from_index(&mut self, index: u8, value: f64) {
        match index {
            0 => self.world.write().unwrap().objects()[self.active_object.unwrap()].set_ambient(value),
            1 => self.world.write().unwrap().objects()[self.active_object.unwrap()].set_diffuse(value),
            2 => self.world.write().unwrap().objects()[self.active_object.unwrap()].set_specular(value),
            3 => self.world.write().unwrap().objects()[self.active_object.unwrap()].set_shininess(value),
            4 => self.world.write().unwrap().objects()[self.active_object.unwrap()].set_reflective(value),
            5 => self.world.write().unwrap().objects()[self.active_object.unwrap()].set_refractive_index(value),
            6 => self.world.write().unwrap().objects()[self.active_object.unwrap()].set_transparency(value),
            _ => return
        };
    }

    fn transformation_drag_updater(&mut self, index: u8, ui: &mut egui::Ui, enabled: bool) {
        if enabled {
            let mut orig = self.get_transform_from_index(index);
            if ui.add(egui::DragValue::new(&mut orig).speed(0.1)).changed() {
                self.set_transform_from_index(index, orig);
                self.prep_update();
            }
        } else {
            ui.add(egui::DragValue::new(&mut 0.0).speed(0.1));
        }
    }

    fn get_transform_from_index(&self, index: u8) -> f64 {
        match index {
            0 => self.world.read().unwrap().read_objects()[self.active_object.unwrap()].get_translate_x(),
            1 => self.world.read().unwrap().read_objects()[self.active_object.unwrap()].get_translate_y(),
            2 => self.world.read().unwrap().read_objects()[self.active_object.unwrap()].get_translate_z(),
            3 => self.world.read().unwrap().read_objects()[self.active_object.unwrap()].get_scale_x(),
            4 => self.world.read().unwrap().read_objects()[self.active_object.unwrap()].get_scale_y(),
            5 => self.world.read().unwrap().read_objects()[self.active_object.unwrap()].get_scale_z(),
            6 => self.world.read().unwrap().read_objects()[self.active_object.unwrap()].get_rotate_x().to_degrees(),
            7 => self.world.read().unwrap().read_objects()[self.active_object.unwrap()].get_rotate_y().to_degrees(),
            8 => self.world.read().unwrap().read_objects()[self.active_object.unwrap()].get_rotate_z().to_degrees(),
            9 => self.world.read().unwrap().read_objects()[self.active_object.unwrap()].get_shear_xy(),
            10 => self.world.read().unwrap().read_objects()[self.active_object.unwrap()].get_shear_xz(),
            11 => self.world.read().unwrap().read_objects()[self.active_object.unwrap()].get_shear_yx(),
            12 => self.world.read().unwrap().read_objects()[self.active_object.unwrap()].get_shear_yz(),
            13 => self.world.read().unwrap().read_objects()[self.active_object.unwrap()].get_shear_zx(),
            14 => self.world.read().unwrap().read_objects()[self.active_object.unwrap()].get_shear_zy(),
            _ => 0.0
        }
    }

    fn set_transform_from_index(&mut self, index: u8, value: f64) {
        match index {
            0 => self.world.write().unwrap().objects()[self.active_object.unwrap()].translate_x(value),
            1 => self.world.write().unwrap().objects()[self.active_object.unwrap()].translate_y(value),
            2 => self.world.write().unwrap().objects()[self.active_object.unwrap()].translate_z(value),
            3 => self.world.write().unwrap().objects()[self.active_object.unwrap()].scale_x(value),
            4 => self.world.write().unwrap().objects()[self.active_object.unwrap()].scale_y(value),
            5 => self.world.write().unwrap().objects()[self.active_object.unwrap()].scale_z(value),
            6 => self.world.write().unwrap().objects()[self.active_object.unwrap()].rotate_x(value.to_radians()),
            7 => self.world.write().unwrap().objects()[self.active_object.unwrap()].rotate_y(value.to_radians()),
            8 => self.world.write().unwrap().objects()[self.active_object.unwrap()].rotate_z(value.to_radians()),
            9 => self.world.write().unwrap().objects()[self.active_object.unwrap()].shear_xy(value),
            10 => self.world.write().unwrap().objects()[self.active_object.unwrap()].shear_xz(value),
            11 => self.world.write().unwrap().objects()[self.active_object.unwrap()].shear_yx(value),
            12 => self.world.write().unwrap().objects()[self.active_object.unwrap()].shear_yz(value),
            13 => self.world.write().unwrap().objects()[self.active_object.unwrap()].shear_zx(value),
            14 => self.world.write().unwrap().objects()[self.active_object.unwrap()].shear_zy(value),
            _ => return,
        };
    }

    fn shape_specific_interface(&mut self, min: f64, max: f64, closed: bool, ui: &mut egui::Ui) {
        let mut new_min = min;
        let mut new_max = max;
        let mut new_closed = closed;

        ui.add(egui::Separator::default());

        ui.with_layout(egui::Layout::top_down_justified(Align::Center), |ui| {
            ui.label("Shape-specific attributes");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut new_min).speed(0.1));
                ui.label("Min");
                ui.add(egui::DragValue::new(&mut new_max).speed(0.1));
                ui.label("Max");
                ui.add(egui::Checkbox::new(&mut new_closed, "Closed"));
            });
        });

        let shape = self.world.read().unwrap().read_objects()[self.active_object.unwrap()].shape;

        if new_min != min {
            let new_shape = match shape {
                Shape::Cone {min: _, max, closed} => Shape::Cone {min: new_min, max, closed},
                Shape::Cylinder {min: _, max, closed} => Shape::Cylinder {min: new_min, max, closed},
                _ => self.world.read().unwrap().read_objects()[self.active_object.unwrap()].shape,
            };
            self.world.write().unwrap().objects()[self.active_object.unwrap()].shape = new_shape;
            self.prep_update();
        };

        if new_max != max {
            let new_shape = match shape {
                Shape::Cone {min, max: _, closed} => Shape::Cone {min, max: new_max, closed},
                Shape::Cylinder {min, max: _, closed} => Shape::Cylinder {min, max: new_max, closed},
                _ => self.world.read().unwrap().read_objects()[self.active_object.unwrap()].shape,
            };
            self.world.write().unwrap().objects()[self.active_object.unwrap()].shape = new_shape;
            self.prep_update();
        }

        if new_closed != closed {
            let new_shape = match shape {
                Shape::Cone {min, max, closed: _} => Shape::Cone {min, max, closed: new_closed},
                Shape::Cylinder {min, max, closed: _} => Shape::Cylinder {min, max, closed: new_closed},
                _ => self.world.read().unwrap().read_objects()[self.active_object.unwrap()].shape,
            };
            self.world.write().unwrap().objects()[self.active_object.unwrap()].shape = new_shape;
            self.prep_update();
        }
    }
}