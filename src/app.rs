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
                    if index == usize::MAX {
                        ui.add(egui::Slider::new(&mut 0.0, 0.0..=1.0).text("Ambient"));
                    } else {
                        let mut ambient = self.active_object().get_ambient();
                        let orig = ambient;
                        ui.add(egui::Slider::new(&mut ambient, 0.0..=1.0).text("Ambient"));
                        if ambient != orig {
                            self.active_object().set_ambient(ambient);
                            self.prep_update();
                        };
                    };
                });

                ui.horizontal(|ui| {
                    if index == usize::MAX {
                        ui.add(egui::Slider::new(&mut 0.0, 0.0..=1.0).text("Diffuse"));
                    } else {
                        let mut diffuse = self.active_object().get_diffuse();
                        let orig = diffuse;
                        ui.add(egui::Slider::new(&mut diffuse, 0.0..=1.0).text("Diffuse"));
                        if diffuse != orig {
                            self.active_object().set_diffuse(diffuse);
                            self.prep_update();
                        }
                    };
                });

                ui.horizontal(|ui| {
                    if index == usize::MAX {
                        ui.add(egui::Slider::new(&mut 0.0, 0.0..=1.0).text("Specular"));
                    } else {
                        let mut specular = self.world.objects()[index].get_specular();
                        let orig = specular;
                        ui.add(egui::Slider::new(&mut specular, 0.0..=1.0).text("Specular"));
                        if specular != orig {
                            self.active_object().set_specular(specular);
                            self.prep_update();
                        }
                    }
                });

                ui.horizontal(|ui| {
                    if index == usize::MAX {
                        ui.add(egui::Slider::new(&mut 0.0, 0.0..=400.0).text("Shininess"));
                    } else {
                        let mut shininess = self.active_object().get_shininess();
                        let orig = shininess;
                        ui.add(egui::Slider::new(&mut shininess, 0.0..=400.0).text("Shininess"));
                        self.world.objects()[index].set_shininess(shininess);
                        if shininess != orig {
                            self.active_object().set_shininess(shininess);
                            self.prep_update();
                        }
                    }
                });

                ui.horizontal(|ui| {
                    if index == usize::MAX {
                        ui.add(egui::Slider::new(&mut 0.0, 0.0..=1.0).text("Reflective"));
                    } else {
                        let mut reflective = self.active_object().get_reflective();
                        let orig = reflective;
                        ui.add(egui::Slider::new(&mut reflective, 0.0..=1.0).text("Reflective"));
                        if reflective != orig {
                            self.active_object().set_reflective(reflective);
                            self.prep_update();
                        }
                    }
                });

                ui.horizontal(|ui| {
                    if index == usize::MAX {
                        ui.add(egui::Slider::new(&mut 0.0, 0.0..=5.0).text("Refractive Index"));
                    } else {
                        let mut refractive_index = self.active_object().get_refractive_index();
                        let orig = refractive_index;
                        ui.add(egui::Slider::new(&mut refractive_index, 0.0..=5.0).text("Refractive Index"));
                        if refractive_index != orig {
                            self.active_object().set_refractive_index(refractive_index);
                            self.prep_update();
                        }
                    }
                });

                ui.horizontal(|ui| {
                    if index == usize::MAX {
                        ui.add(egui::Slider::new(&mut 0.0, 0.0..=1.0).text("Transparency"));
                    } else {
                        let mut transparency = self.active_object().get_transparency();
                        let orig = transparency;
                        ui.add(egui::Slider::new(&mut transparency, 0.0..=1.0).text("Transparency"));
                        if transparency != orig {
                            self.active_object().set_transparency(transparency);
                            self.prep_update();
                        }
                    }
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
            if ui.button("Render").clicked() {
                let canvas = self.camera.parallel_render(&self.world);
                canvas.canvas_to_png("image.png");
            }
        });

        egui::SidePanel::right("object_list").show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                let mut idx = 0;
                for item in self.world.objects() {
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
}