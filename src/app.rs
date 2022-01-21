use eframe::{egui, epi};
use crate::prelude::*;
use std::sync::RwLockWriteGuard;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use crate::egui::Align;

pub struct RayTracer {
    world: World,
    camera: Camera,
    active_object: Option<usize>,
    last_preview: Option<(egui::Vec2, egui::TextureId)>,
    preview_up_to_date: Arc<AtomicBool>,
    thread_just_finished_preview: Arc<AtomicBool>,
    rendering: Arc<AtomicBool>,
    rendering_progress: Arc<AtomicUsize>,
    preview_image: Arc<RwLock<Option<epi::Image>>>,
    preview_camera: Arc<RwLock<Camera>>,
    world_sender: std::sync::mpsc::Sender<World>,
}

impl Default for RayTracer {
    fn default() -> Self {
        let mut new = Self {
            world: World::new_default(),
            camera: Camera::new(1000, 800, FRAC_PI_2),
            active_object: None,
            last_preview: None,
            preview_up_to_date: Arc::new(
                AtomicBool::new(
                    false
                )
            ),
            thread_just_finished_preview: Arc::new(
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
            world_sender: std::sync::mpsc::channel().0,
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
        let up_to_date = self.preview_up_to_date.clone();
        let img_arc = self.preview_image.clone();
        let cam_arc = self.preview_camera.clone();
        let just_finished = self.thread_just_finished_preview.clone();
        let (tx, rx) = std::sync::mpsc::channel();
        self.world_sender = tx;

        // Thread that handles rendering the preview image.
        std::thread::spawn(move || {
            loop {
                if !up_to_date.load(Ordering::Relaxed) {
                    if let Ok(wd) = rx.try_recv() {
                        let mut img_lock = img_arc.write().unwrap();
                        let cam = *cam_arc.read().unwrap();
                        let buffer = cam.preview_parallel_render(wd).canvas_to_buffer();
                        let size = [cam.get_hsize() as usize, cam.get_vsize() as usize];
                        let image = epi::Image::from_rgba_unmultiplied(size, &buffer.into_vec());
                        img_lock.insert(image);
                        up_to_date.store(true, Ordering::Relaxed);
                        just_finished.store(true, Ordering::Relaxed);
                    }
                };
            }
        });

        self.world_sender.send(world);
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        self.show_menu_bar(ctx, frame);
        self.show_render_bar(ctx);

        egui::SidePanel::right("side_panel").show(ctx, |ui| {
            self.show_camera_pane(ui);
            ui.add(egui::Separator::default());

            self.show_materials_pane(ui);

            // Interface for shape-specific attributes (min, max, end caps)
            if let Some(arc_curr_obj) = self.get_active_object() {
                let curr_obj = arc_curr_obj.write().unwrap();
                match *curr_obj {
                    ObjectHolder::Object(object) => {
                        match object.shape {
                            Shape::Cone {min: _, max: _, closed: _} => &self.shape_specific_interface(ui, curr_obj),
                            Shape::Cylinder {min: _, max: _, closed: _} => &self.shape_specific_interface(ui, curr_obj),
                            _ => &(),
                        };
                    },
                    ObjectHolder::Group(_) => ()
                };
            };

            ui.add(egui::Separator::default());

            // Transformations
            self.show_transformation_panel(ui);
        });

        egui::SidePanel::right("object_list").show(ctx, |ui| {
            self.show_object_list(ui);
        });

        let arc_img = self.preview_image.clone();
        self.update_preview(frame, arc_img.read().unwrap());

        egui::CentralPanel::default().show(ctx, |ui| {
            self.show_preview_image(ui);
            egui::warn_if_debug_build(ui);
        });
    }
}

impl RayTracer {
    fn get_active_object(&self) -> Option<Arc<RwLock<ObjectHolder>>> {
        match self.active_object {
            Some(value) => Some(self.world.objects()[value].clone()),
            None => None,
        }
    }

    fn delete_active_object(&mut self) {
        if let Some(index) = self.active_object {
            self.world.mut_objects().remove(index);
            if index == 0 {
                self.active_object = None;
            } else {
                self.active_object = Some(index - 1);
            }
            self.prep_update();
        }
    }

    fn update_preview(&mut self, frame: &epi::Frame, curr_preview: RwLockReadGuard<Option<epi::Image>>) {
        if let Some(image) = curr_preview.as_ref() {
            if self.thread_just_finished_preview.load(Ordering::Relaxed) {
                if let Some((img, tex)) = self.last_preview {
                    frame.free_texture(tex);
                }
                let x = image.size[0] as f32;
                let y = image.size[1] as f32;
                let texture = frame.alloc_texture(image.clone());
                let size = egui::Vec2::new(x, y);
                self.last_preview = Some((size, texture));
                self.thread_just_finished_preview.store(false, Ordering::Relaxed);
            };
        }
    }

    fn prep_update(&mut self) {
        self.preview_up_to_date.store(false, Ordering::Relaxed);
        let wd = self.world.clone();
        self.world_sender.send(wd);
    }

    fn add_new_shape(&mut self, shape: Shape) {
        let new = match shape {
            Shape::Cone {min, max, closed} => cones::new(min, max, closed),
            Shape::Cube => cubes::new(),
            Shape::Cylinder {min, max, closed} => cylinders::new(min, max, closed),
            Shape::Plane => planes::new(),
            Shape::Sphere => spheres::new(),
        };
        self.world.add_object(new);
        self.prep_update();
    }

    fn add_new_group(&mut self) {
        let grp = Group::new_empty();
        self.world.add_group(grp);
    }

    fn material_attribute_slider(&mut self, index: u8, ui: &mut egui::Ui, enabled: bool, object: &mut Object) {
        if enabled {
            let (mut orig, name, rng) = self.get_scalar_attribute_from_index(index, object);
            if ui.add(egui::Slider::new(&mut orig, rng).text(name)).changed() {
                self.set_scalar_attribute_from_index(index, orig, object);
                self.prep_update();
            }
        } else {
            let (name, rng) = self.scalar_attribute_spec(index);
            ui.add(egui::Slider::new(&mut 0.0, rng).text(name));
        }
    }

    fn get_scalar_attribute_from_index(&self, index: u8, object: &mut Object) -> (f64, &str, std::ops::RangeInclusive<f64>) {
        let (name, rng) = self.scalar_attribute_spec(index);
        match index {
            0 => (object.get_ambient(), name, rng),
            1 => (object.get_diffuse(), name, rng),
            2 => (object.get_specular(), name, rng),
            3 => (object.get_shininess(), name, rng),
            4 => (object.get_reflective(), name, rng),
            5 => (object.get_refractive_index(), name, rng),
            6 => (object.get_transparency(), name, rng),
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

    fn set_scalar_attribute_from_index(&mut self, index: u8, value: f64, object: &mut Object) {
        match index {
            0 => object.set_ambient(value),
            1 => object.set_diffuse(value),
            2 => object.set_specular(value),
            3 => object.set_shininess(value),
            4 => object.set_reflective(value),
            5 => object.set_refractive_index(value),
            6 => object.set_transparency(value),
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
        if let Some(curr_obj) = self.get_active_object() {
            let object = curr_obj.read().unwrap();
            match index {
                0 => object.get_translate_x(),
                1 => object.get_translate_y(),
                2 => object.get_translate_z(),
                3 => object.get_scale_x(),
                4 => object.get_scale_y(),
                5 => object.get_scale_z(),
                6 => object.get_rotate_x().to_degrees(),
                7 => object.get_rotate_y().to_degrees(),
                8 => object.get_rotate_z().to_degrees(),
                9 => object.get_shear_xy(),
                10 => object.get_shear_xz(),
                11 => object.get_shear_yx(),
                12 => object.get_shear_yz(),
                13 => object.get_shear_zx(),
                14 => object.get_shear_zy(),
                _ => 0.0
            }
        } else {
            0.0
        }
    }

    fn set_transform_from_index(&mut self, index: u8, value: f64) {
        if let Some(curr_obj) = self.get_active_object() {
            let mut object = curr_obj.write().unwrap();
            match index {
                0 => object.translate_x(value),
                1 => object.translate_y(value),
                2 => object.translate_z(value),
                3 => object.scale_x(value),
                4 => object.scale_y(value),
                5 => object.scale_z(value),
                6 => object.rotate_x(value.to_radians()),
                7 => object.rotate_y(value.to_radians()),
                8 => object.rotate_z(value.to_radians()),
                9 => object.shear_xy(value),
                10 => object.shear_xz(value),
                11 => object.shear_yx(value),
                12 => object.shear_yz(value),
                13 => object.shear_zx(value),
                14 => object.shear_zy(value),
                _ => return,
            };
        }
    }

    fn shape_specific_interface(&mut self, ui: &mut egui::Ui, mut curr_obj: RwLockWriteGuard<ObjectHolder>) {
        if let Some(arc_object) = self.get_active_object() {
            if let ObjectHolder::Object(ref mut obj) = *curr_obj {
                match obj.shape {
                    Shape::Cone {ref mut min, ref mut max, ref mut closed} => {
                        ui.add(egui::Separator::default());
                        ui.with_layout(egui::Layout::top_down_justified(Align::Center), |ui| {
                            ui.label("Shape-specific attributes");
                            ui.horizontal(|ui| {
                                ui.label("Min");
                                if ui.add(egui::DragValue::new(min).speed(0.1)).changed() {
                                    self.prep_update();
                                };
                                ui.label("Max");
                                if ui.add(egui::DragValue::new(max).speed(0.1)).changed() {
                                    self.prep_update();
                                };
                                if ui.add(egui::Checkbox::new(closed, "Closed")).changed() {
                                    self.prep_update();
                                };
                            });
                        });
                    }
                    Shape::Cylinder {ref mut min, ref mut max, ref mut closed} => {
                        ui.add(egui::Separator::default());
                        ui.with_layout(egui::Layout::top_down_justified(Align::Center), |ui| {
                            ui.label("Shape-specific attributes");
                            ui.horizontal(|ui| {
                                ui.label("Min");
                                if ui.add(egui::DragValue::new(min).speed(0.1)).changed() {
                                    self.prep_update();
                                };
                                ui.label("Max>");
                                if ui.add(egui::DragValue::new(max).speed(0.1)).changed() {
                                    self.prep_update();
                                };
                                if ui.add(egui::Checkbox::new(closed, "Closed")).changed() {
                                    self.prep_update();
                                };
                            });
                        });
                    }
                    _ => ()
                }
            }
        }
    }

    fn show_menu_bar(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
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
                    if ui.button("New Group").clicked() {
                        self.add_new_group();
                    }

                    ui.group(|ui| {
                        ui.set_enabled(self.active_object.is_some() && self.get_active_object().unwrap().read().unwrap().is_group());
                        ui.menu_button("Add objects to group", |ui| {
                            let vec_objects = self.world.mut_objects();
                            let mut remove: usize = usize::MAX;
                            for (i, item) in vec_objects.iter().enumerate() {
                                let obj = item.read();
                                if obj.is_err() || Some(i) == self.active_object {
                                    continue;
                                }
                                let object = obj.unwrap();
                                if ui.button(format!("{}. {}", i+1, object)).clicked() {
                                    let group_index = self.active_object.unwrap();
                                    let group = vec_objects[group_index].clone();
                                    let mut group_lock = group.write().unwrap();
                                    group_lock.add_object_holder(object.clone());
                                    remove = i;
                                }
                            }
                            if remove != usize::MAX {
                                vec_objects.remove(remove);
                                self.active_object = Some(self.active_object.unwrap() - 1);
                            }
                        });
                        ui.separator();
                        ui.menu_button("Remove objects from group", |ui| {
                            let mut world_objects = self.world.mut_objects();
                            let mut active_group = world_objects[self.active_object.unwrap()].write().unwrap();
                            let mut remove = usize::MAX;
                            for (i, item) in active_group.get_group_members().unwrap().iter().enumerate() {
                                if ui.button(format!("{}. {}", i+1, item)).clicked() {
                                    remove = i;
                                }
                            };
                            let mut removed_item = None;
                            if remove != usize::MAX {
                                removed_item = Some(active_group.get_group_members().unwrap()[remove].clone());
                                active_group.remove_from_group(remove);
                            };
                            std::mem::drop(active_group);
                            if removed_item.is_some() {
                                world_objects.push(Arc::new(RwLock::new(removed_item.unwrap())));
                            }
                        });
                    });

                    ui.group(|ui| {
                        ui.set_enabled(self.get_active_object().is_some());
                        if ui.button("Delete active object").clicked() {
                            self.delete_active_object();
                        };
                    });
                });
            });
        });
    }

    fn show_render_bar(&mut self, ctx: &egui::CtxRef) {
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
    }

    fn show_camera_pane(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::top_down_justified(Align::Center),|ui| {
            ui.label("Camera");
            let arc_preview_camera = self.preview_camera.clone();
            ui.horizontal(|ui| {
                let mut hsize = self.camera.get_hsize();
                if ui.add(egui::DragValue::new(&mut hsize).speed(1)).changed() {
                    self.camera.set_hsize(hsize);
                    let mut prev = arc_preview_camera.write().unwrap();
                    *prev = Camera::new_preview(&self.camera);
                    self.prep_update();
                }
                ui.label("x");

                let mut vsize = self.camera.get_vsize();
                if ui.add(egui::DragValue::new(&mut vsize).speed(1)).changed() {
                    self.camera.set_vsize(vsize);
                    let mut prev = arc_preview_camera.write().unwrap();
                    *prev = Camera::new_preview(&self.camera);
                    self.prep_update();
                }
                ui.label("y");

                let mut fov = self.camera.get_fov().to_degrees();
                if ui.add(egui::DragValue::new(&mut fov).speed(0.5)).changed() {
                    self.camera.set_fov(fov.to_radians());
                    let mut prev = arc_preview_camera.write().unwrap();
                    *prev = Camera::new_preview(&self.camera);
                    self.prep_update();
                }
                ui.label("FOV");
            });

            ui.add_space(4.0);

            ui.horizontal(|ui| {
                let mut from = self.camera.get_from();

                if ui.add(egui::DragValue::new(&mut from.x).speed(0.1)).changed() {
                    self.camera.set_from(from);
                    let mut prev = arc_preview_camera.write().unwrap();
                    *prev = Camera::new_preview(&self.camera);
                    self.prep_update();
                }
                ui.label("x");
                if ui.add(egui::DragValue::new(&mut from.y).speed(0.1)).changed() {
                    self.camera.set_from(from);
                    let mut prev = arc_preview_camera.write().unwrap();
                    *prev = Camera::new_preview(&self.camera);
                    self.prep_update();
                }
                ui.label("y");
                if ui.add(egui::DragValue::new(&mut from.z).speed(0.1)).changed() {
                    self.camera.set_from(from);
                    let mut prev = arc_preview_camera.write().unwrap();
                    *prev = Camera::new_preview(&self.camera);
                    self.prep_update();
                }
                ui.label("z");
                ui.label("|   From");
            });

            ui.horizontal(|ui| {
                let mut to = self.camera.get_to();
                if ui.add(egui::DragValue::new(&mut to.x).speed(0.1)).changed() {
                    self.camera.set_to(to);
                    let mut prev = arc_preview_camera.write().unwrap();
                    *prev = Camera::new_preview(&self.camera);
                    self.prep_update();
                }
                ui.label("x");
                if ui.add(egui::DragValue::new(&mut to.y).speed(0.1)).changed() {
                    self.camera.set_to(to);
                    let mut prev = arc_preview_camera.write().unwrap();
                    *prev = Camera::new_preview(&self.camera);
                    self.prep_update();
                }
                ui.label("y");
                if ui.add(egui::DragValue::new(&mut to.z).speed(0.1)).changed() {
                    self.camera.set_to(to);
                    let mut prev = arc_preview_camera.write().unwrap();
                    *prev = Camera::new_preview(&self.camera);
                    self.prep_update();
                }
                ui.label("z");
                ui.label("|   To");
            });

            ui.horizontal(|ui| {
                let mut up = self.camera.get_up();
                if ui.add(egui::DragValue::new(&mut up.x).speed(0.1)).changed() {
                    self.camera.set_up(up);
                    let mut prev = arc_preview_camera.write().unwrap();
                    *prev = Camera::new_preview(&self.camera);
                    self.prep_update();
                }
                ui.label("x");
                if ui.add(egui::DragValue::new(&mut up.y).speed(0.1)).changed() {
                    self.camera.set_up(up);
                    let mut prev = arc_preview_camera.write().unwrap();
                    *prev = Camera::new_preview(&self.camera);
                    self.prep_update();
                }
                ui.label("y");
                if ui.add(egui::DragValue::new(&mut up.z).speed(0.1)).changed() {
                    self.camera.set_up(up);
                    let mut prev = arc_preview_camera.write().unwrap();
                    *prev = Camera::new_preview(&self.camera);
                    self.prep_update();
                }
                ui.label("z");
                ui.label("|   Up");
            });
        });
    }

    fn show_materials_pane(&mut self, ui: &mut egui::Ui) {
        if let Some(arc_obj) = self.get_active_object() {
            let mut active_object = arc_obj.write().unwrap();
            match *active_object {
                ObjectHolder::Object(ref mut object) => {
                    ui.with_layout(egui::Layout::top_down_justified(Align::Center), |ui| {
                        ui.label("Material attributes");
                        ui.horizontal(|ui| {
                            ui.set_enabled(object.get_pattern().get_pattern_type() == PatternType::Solid);
                            let orig = object.get_color();
                            let mut color = [
                                orig.get_red() as f32,
                                orig.get_green() as f32,
                                orig.get_blue() as f32
                            ];
                            if ui.color_edit_button_rgb(&mut color).changed() {
                                object.set_color(
                                    surfaces::colors::color(
                                        color[0] as f64,
                                        color[1] as f64,
                                        color[2] as f64
                                    )
                                );
                                self.prep_update();
                            };
                            ui.label("Base Color");
                        });

                        let mut pattern = object.get_pattern().get_pattern_type();
                        let orig = pattern;
                        egui::ComboBox::from_label("Pattern")
                            .selected_text(format!("{}", object.get_pattern()))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut pattern, PatternType::Solid, "Solid Color");
                                ui.selectable_value(&mut pattern, PatternType::Stripe, "Stripe");
                                ui.selectable_value(&mut pattern, PatternType::Gradient, "Gradient");
                                ui.selectable_value(&mut pattern, PatternType::Ring, "Ring");
                                ui.selectable_value(&mut pattern, PatternType::Checker3d, "Checkers");
                            });
                        if pattern != orig {
                            let new_pattern = object.get_pattern().duplicate_different_type(pattern);
                            object.set_pattern(new_pattern);
                            self.prep_update();
                        };

                        ui.horizontal(|ui| {
                            let ptrn = object.get_pattern();
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
                                        object.set_pattern(
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
                                        object.set_pattern(
                                            ptrn.duplicate_change_color_2(new_color)
                                        );
                                        self.prep_update();
                                    }
                                    ui.label("Color 2");
                                });
                            };
                        });

                        ui.horizontal(|ui| {
                            self.material_attribute_slider(0, ui, true, object);
                        });

                        ui.horizontal(|ui| {
                            self.material_attribute_slider(1, ui, true, object);
                        });

                        ui.horizontal(|ui| {
                            self.material_attribute_slider(2, ui, true, object);
                        });

                        ui.horizontal(|ui| {
                            self.material_attribute_slider(3, ui, true, object);
                        });

                        ui.horizontal(|ui| {
                            self.material_attribute_slider(4, ui, true, object);
                        });

                        ui.horizontal(|ui| {
                            self.material_attribute_slider(5, ui, true, object);
                        });

                        ui.horizontal(|ui| {
                            self.material_attribute_slider(6, ui, true, object);
                        });

                        ui.horizontal(|ui| {
                            let mut casts_shadow = object.casts_shadow();
                            if ui.checkbox(&mut casts_shadow, "Casts shadow").changed() {
                                object.set_casts_shadow(casts_shadow);
                                self.prep_update();
                            };
                        });
                    });
                },
                _ => ()
            }
        }
    }

    fn show_transformation_panel(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::top_down_justified(Align::Center), |ui| {
            let curr_obj = &mut self.get_active_object();
            ui.set_enabled(curr_obj.is_some());
            ui.label("Object transformations");
            ui.horizontal(|ui| {
                ui.group(|ui| {
                    self.transformation_drag_updater(0, ui, true);
                    self.transformation_drag_updater(1, ui, true);
                    self.transformation_drag_updater(2, ui, true);
                });
                ui.label("Translation");
            });

            ui.horizontal(|ui| {
                ui.group(|ui| {
                    self.transformation_drag_updater(3, ui, true);
                    self.transformation_drag_updater(4, ui, true);
                    self.transformation_drag_updater(5, ui, true);
                });
                ui.label("Scaling");
            });

            ui.horizontal(|ui| {
                ui.group(|ui| {
                    self.transformation_drag_updater(6, ui, true);
                    self.transformation_drag_updater(7, ui, true);
                    self.transformation_drag_updater(8, ui, true);
                });
                ui.label("Rotation");
            });

            ui.horizontal(|ui| {
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            self.transformation_drag_updater(9, ui, true);
                            self.transformation_drag_updater(10, ui, true);
                            self.transformation_drag_updater(11, ui, true);
                        });

                        ui.horizontal(|ui| {
                            self.transformation_drag_updater(12, ui, true);
                            self.transformation_drag_updater(13, ui, true);
                            self.transformation_drag_updater(14, ui, true);
                        });
                    });
                });
                ui.label("Shearing");
            });
        });
    }

    fn show_object_list(&mut self, ui: &mut egui::Ui) {
        let objects = self.world.objects();
        let active_index = &mut self.active_object;
        egui::ScrollArea::vertical().show(ui, |ui| {
            for (i, item) in objects.iter().enumerate() {
                let object = item.read().unwrap();
                ui.selectable_value(
                    active_index,
                    Some(i),
                    if object.is_group() {
                        let mut string = format!("{}. Group", i+1);
                        for obj in object.get_group_members().unwrap() {
                            string.push_str(&*format!("\n  - {}", obj))
                        }
                        string
                    } else {
                        format!("{}. {}", i+1, object)
                    }
                );
            };
        });
    }

    fn show_preview_image(&self, ui: &mut egui::Ui) {
        egui::ScrollArea::new([true, true]).show(ui, |ui| {
            if let Some((size, texture)) = self.last_preview {
                ui.image(texture, size);
            }
        });
    }
}