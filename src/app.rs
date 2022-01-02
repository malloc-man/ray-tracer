use std::any::Any;
use std::fmt::Debug;
use eframe::{egui, epi};
use image::RgbImage;
use crate::prelude::*;
use crate::Shape::Sphere;

pub struct RayTracer {
    world: World,
    camera: Camera,
    active_object: Option<usize>,
    last_preview: Option<(egui::Vec2, egui::TextureId)>,
    preview_up_to_date: bool,
}

impl Default for RayTracer {
    fn default() -> Self {
        Self {
            world: World::new_default(),
            camera: Camera::new(640, 480, FRAC_PI_2),
            active_object: None,
            last_preview: None,
            preview_up_to_date: false,
        }
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
        self.camera.set_transform(view_transform(
                point(0.0, 1.5, -5.0),
                point(0.0, 1.5, 0.0),
                vector(0.0, 1.0, 0.0)));
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        self.update_preview(frame);

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
                        if ui.button("Delete Active Object").clicked() {
                            self.delete_active_object();
                        }
                    })
                });
            });
        });

        egui::SidePanel::right("side_panel").show(ctx, |ui| {
            ui.group(|ui| {
                ui.set_enabled(self.active_object.is_some());
                let index = self.active_object.unwrap_or(usize::MAX);

                ui.horizontal(|ui| {
                    if index == usize::MAX {
                        ui.color_edit_button_rgb(&mut [1.0, 1.0, 1.0]);
                        ui.label("Base Color");
                    } else {
                        ui.set_enabled(self.active_object().get_pattern().get_pattern_type() == PatternType::Solid);
                        let orig = self.active_object().get_color();
                        let mut color = [
                            orig.get_red() as f32,
                            orig.get_green() as f32,
                            orig.get_blue() as f32
                        ];
                        if ui.color_edit_button_rgb(&mut color).changed() {
                            self.active_object().set_color(
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
                    let mut pattern = self.active_object().get_pattern().get_pattern_type();
                    let orig = pattern;
                    egui::ComboBox::from_label("Pattern")
                        .selected_text(format!("{}", self.active_object().get_pattern()))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut pattern, PatternType::Solid, "Solid Color");
                            ui.selectable_value(&mut pattern, PatternType::Stripe, "Stripe");
                            ui.selectable_value(&mut pattern, PatternType::Gradient, "Gradient");
                            ui.selectable_value(&mut pattern, PatternType::Ring, "Ring");
                            ui.selectable_value(&mut pattern, PatternType::Checker3d, "Checkers");
                        });
                    if pattern != orig {
                        let new_pattern = self.active_object().get_pattern().duplicate_different_type(pattern);
                        self.active_object().set_pattern(new_pattern);
                        self.prep_update();
                    }
                }

                ui.vertical(|ui| {
                    if index != usize::MAX {
                        let ptrn = self.active_object().get_pattern();
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
                                    self.active_object().set_pattern(
                                        ptrn.duplicate_change_color_1(new_color)
                                    );
                                    self.prep_update();
                                }
                                ui.label("Pattern Color 1");
                            });

                            ui.horizontal(|ui| {
                                if ui.color_edit_button_rgb(&mut color2).changed() {
                                    let new_color = color(
                                        color2[0] as f64,
                                        color2[1] as f64,
                                        color2[2] as f64,
                                    );
                                    self.active_object().set_pattern(
                                        ptrn.duplicate_change_color_2(new_color)
                                    );
                                    self.prep_update();
                                }
                                ui.label("Pattern Color 2");
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
                        let mut casts_shadow = self.active_object().casts_shadow();
                        let orig = casts_shadow;
                        ui.checkbox(&mut casts_shadow, "Casts shadow");
                        self.world.objects()[index].set_casts_shadow(casts_shadow);
                        if casts_shadow != orig {
                            self.active_object().set_casts_shadow(casts_shadow);
                            self.prep_update();
                        }
                    }
                });
            });

            ui.group(|ui| {
                ui.set_enabled(self.active_object.is_some());
                let index = self.active_object.unwrap_or(usize::MAX);

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

            if ui.button("Render").clicked() {
                let canvas = self.camera.parallel_render(&self.world);
                canvas.canvas_to_png("image.png");
            }
        });

        egui::SidePanel::right("object_list").show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                let mut idx = 0;
                for item in self.world.read_objects() {
                    ui.selectable_value(&mut self.active_object, Some(idx), (format!("{}", item.shape)));
                    idx += 1;
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some((size, texture)) = self.last_preview {
                ui.image(texture, size);
            }
            egui::warn_if_debug_build(ui);
        });
    }
}

impl RayTracer {
    fn delete_active_object(&mut self) {
        if let Some(index) = self.active_object {
            self.world.objects().remove(index);
            match index {
                0 => self.active_object = None,
                _ => self.active_object = Some(index-1),
            }
            self.prep_update();
        }
    }

    fn update_preview(&mut self, frame: &epi::Frame) {
        if !self.preview_up_to_date {
            let preview_camera = Camera::new_preview(&self.camera);
            let buffer = preview_camera.preview_parallel_render(&self.world).canvas_to_buffer();

            let size = [buffer.width() as usize, buffer.height() as usize];
            let pixels = buffer.into_vec();
            let image = epi::Image::from_rgba_unmultiplied(size, &pixels);

            let texture = frame.alloc_texture(image);
            let size = egui::Vec2::new(size[0] as f32, size[1] as f32);
            self.last_preview = Some((size, texture));
            self.preview_up_to_date = true;
        }
    }

    fn prep_update(&mut self) {
        self.preview_up_to_date = false;
    }

    fn active_object(&mut self) -> &mut Object {
        &mut self.world.objects()[self.active_object.unwrap()]
    }

    fn read_active_object(&self) -> &Object {
        &self.world.read_objects()[self.active_object.unwrap()]
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
        self.active_object = Some(self.world.objects().len()-1);
        self.prep_update();
    }

    fn material_attribute_slider(&mut self, index: u8, ui: &mut egui::Ui, enabled: bool) {
        if enabled {
            let (mut orig, name, rng) = self.get_scalar_attribute_from_index(index);
            let save = orig;
            ui.add(egui::Slider::new(&mut orig, rng).text(name));
            if orig != save {
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
            0 => (self.read_active_object().get_ambient(), name, rng),
            1 => (self.read_active_object().get_diffuse(), name, rng),
            2 => (self.read_active_object().get_specular(), name, rng),
            3 => (self.read_active_object().get_shininess(), name, rng),
            4 => (self.read_active_object().get_reflective(), name, rng),
            5 => (self.read_active_object().get_refractive_index(), name, rng),
            6 => (self.read_active_object().get_transparency(), name, rng),
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
            0 => self.active_object().set_ambient(value),
            1 => self.active_object().set_diffuse(value),
            2 => self.active_object().set_specular(value),
            3 => self.active_object().set_shininess(value),
            4 => self.active_object().set_reflective(value),
            5 => self.active_object().set_refractive_index(value),
            6 => self.active_object().set_transparency(value),
            _ => return
        };
    }

    fn transformation_drag_updater(&mut self, index: u8, ui: &mut egui::Ui, enabled: bool) {
        if enabled {
            let mut orig = self.get_transform_from_index(index);
            let save = orig;
            ui.add(egui::DragValue::new(&mut orig).speed(0.1));
            if orig != save {
                self.set_transform_from_index(index, orig);
                self.prep_update();
            }
        } else {
            ui.add(egui::DragValue::new(&mut 0.0).speed(0.1));
        }
    }

    fn get_transform_from_index(&self, index: u8) -> f64 {
        match index {
            0 => self.read_active_object().get_translate_x(),
            1 => self.read_active_object().get_translate_y(),
            2 => self.read_active_object().get_translate_z(),
            3 => self.read_active_object().get_scale_x(),
            4 => self.read_active_object().get_scale_y(),
            5 => self.read_active_object().get_scale_z(),
            6 => self.read_active_object().get_rotate_x(),
            7 => self.read_active_object().get_rotate_y(),
            8 => self.read_active_object().get_rotate_z(),
            9 => self.read_active_object().get_shear_xy(),
            10 => self.read_active_object().get_shear_xz(),
            11 => self.read_active_object().get_shear_yx(),
            12 => self.read_active_object().get_shear_yz(),
            13 => self.read_active_object().get_shear_zx(),
            14 => self.read_active_object().get_shear_zy(),
            _ => 0.0
        }
    }

    fn set_transform_from_index(&mut self, index: u8, value: f64) {
        match index {
            0 => self.active_object().translate_x(value),
            1 => self.active_object().translate_y(value),
            2 => self.active_object().translate_z(value),
            3 => self.active_object().scale_x(value),
            4 => self.active_object().scale_y(value),
            5 => self.active_object().scale_z(value),
            6 => self.active_object().rotate_x(value),
            7 => self.active_object().rotate_y(value),
            8 => self.active_object().rotate_z(value),
            9 => self.active_object().shear_xy(value),
            10 => self.active_object().shear_xz(value),
            11 => self.active_object().shear_yx(value),
            12 => self.active_object().shear_yz(value),
            13 => self.active_object().shear_zx(value),
            14 => self.active_object().shear_zy(value),
            _ => return,
        };
    }
}