use std::collections::HashMap;

use egui::Align2;
use egui::Color32;
use egui_commonmark::*;

use std::time::Duration;

#[derive(serde::Deserialize, serde::Serialize)]
struct MdViewer {
    title: String,
    markdown: String,
    is_visible: bool,
}

impl MdViewer {
    fn new(title: &str, md: String) -> MdViewer {
        MdViewer {
            title: title.to_string(),
            markdown: md,
            is_visible: false,
        }
    }
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    md_views: HashMap<String, MdViewer>,

    show_settings: bool,
    show_photo: bool,
    show_calculator: bool,

    bg_color: [f32; 3],
    txt_color: [f32; 3],

    calculator_buff: String,
    calculator_formula: String,

    #[serde(skip)]
    cache: CommonMarkCache,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            cache: CommonMarkCache::default(),
            md_views: HashMap::new(),
            bg_color: [38.0 / 255.0, 38.0 / 255.0, 38.0 / 255.0],
            txt_color: [1.0, 1.0, 1.0],

            calculator_buff: "".to_string(),
            calculator_formula: "".to_string(),

            show_settings: false,
            show_photo: false,
            show_calculator: false,
        }
    }
}

impl TemplateApp {
    fn push_md_view(&mut self, title: &str, md: String) {
        println!("Pushing {}", title);
        self.md_views
            .insert(title.to_string(), MdViewer::new(title, md));
    }

    fn show_md_view(&mut self, title: &str) {
        if let Some(view) = self.md_views.get_mut(title) {
            println!("Showing {}", title);
            view.is_visible = true;
        } else {
            println!("No such view {}", title);
        }
    }

    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_pixels_per_point(2.0);

        cc.egui_ctx.style_mut(|style| {
            // Show the url of a hyperlink on hover
            style.url_in_tooltip = true;
            style.visuals.override_text_color = Some(Color32::from_rgb(255, 255, 255));
            style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(0, 0, 0);
            style.visuals.widgets.open.fg_stroke.color = Color32::from_rgb(255, 255, 255);
        });

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        //if let Some(storage) = cc.storage {
        //    return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        //}

        let mut ret: TemplateApp = Default::default();

        ret.push_md_view(
            "Some History",
            String::from(include_str!("../assets/md/history.md")),
        );
        ret.push_md_view(
            "Education",
            String::from(include_str!("../assets/md/posstud.md")),
        );
        ret.push_md_view(
            "Skills",
            String::from(include_str!("../assets/md/skills.md")),
        );
        ret.push_md_view(
            "Publications",
            String::from(include_str!("../assets/md/pub.md")),
        );
        ret.push_md_view(
            "Services",
            String::from(include_str!("../assets/md/services.md")),
        );
        ret.push_md_view(
            "ADMIRE",
            String::from(include_str!("../assets/md/admire.md")),
        );
        ret.push_md_view(
            "MPI / MPC",
            String::from(include_str!("../assets/md/mpc.md")),
        );
        ret.push_md_view(
            "Profiling / Debugging",
            String::from(include_str!("../assets/md/profdeb.md")),
        );
        ret.push_md_view(
            "Containers / Virtualization",
            String::from(include_str!("../assets/md/cont.md")),
        );

        ret.push_md_view(
            "Imeon Monitor",
            String::from(include_str!("../assets/md/proj/imeon.md")),
        );

        ret.push_md_view(
            "Talkspirit Proxy",
            String::from(include_str!("../assets/md/proj/talkspirit.md")),
        );

        ret.push_md_view(
            "Second Brain Helper",
            String::from(include_str!("../assets/md/proj/sb.md")),
        );

        ret.push_md_view(
            "Clippyrs",
            String::from(include_str!("../assets/md/proj/clippy.md")),
        );

        ret
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel")
            .frame(egui::Frame::none().fill(egui::Color32::from_hex("#afa8af").unwrap()))
            .show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("About", |ui| {
                        if ui.button("About Me").clicked() {
                            self.show_md_view("Some History");
                        }
                        if ui.button("How I Look").clicked() {
                            self.show_photo = true;
                        }
                        ui.hyperlink_to("Github..", "https://github.com/besnardjb");
                        ui.hyperlink_to("Linkedn..", "https://www.linkedin.com/in/jbbesnard");
                    });

                    ui.menu_button("Resume", |ui| {
                        if ui.button("Position/Education").clicked() {
                            self.show_md_view("Education");
                        }
                        if ui.button("Skills").clicked() {
                            self.show_md_view("Skills");
                        }
                        if ui.button("Publications").clicked() {
                            self.show_md_view("Publications");
                        }
                        if ui.button("Services").clicked() {
                            self.show_md_view("Services");
                        }
                        ui.hyperlink_to("Download CV..", "https://www.jbbesnard.pro/pdf/cv.pdf");
                    });

                    ui.menu_button("Software", |ui| {
                        ui.menu_button("Work Projects", |ui| {
                            if ui.button("ADMIRE").clicked() {
                                self.show_md_view("ADMIRE");
                            }
                            if ui.button("MPI / MPC").clicked() {
                                self.show_md_view("MPI / MPC");
                            }
                            if ui.button("Profiling / Debugging").clicked() {
                                self.show_md_view("Profiling");
                            }
                            if ui.button("Containers / Virtualization").clicked() {
                                self.show_md_view("Containers / Virtualization");
                            }
                        });
                        ui.menu_button("Fun Projects", |ui| {
                            if ui.button("Imeon Monitor").clicked() {
                                self.show_md_view("Imeon Monitor");
                            }
                            if ui.button("Talkspirit Proxy").clicked() {
                                self.show_md_view("Talkspirit Proxy");
                            }
                            if ui.button("Second Brain Helper").clicked() {
                                self.show_md_view("Second Brain Helper");
                            }
                            if ui.button("Clippyrs").clicked() {
                                self.show_md_view("Clippyrs");
                            }
                        });
                        if ui.button("Calculator").clicked() {
                            self.show_calculator = true;
                        }
                    });
                });
            });

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(egui::Color32::from_hex("#008").unwrap()))
            .show(ctx, |ui| {
                ui.style_mut().visuals.override_text_color = Some(Color32::from_rgb(255, 255, 255));
                ui.style_mut().visuals.hyperlink_color = Color32::from_rgb(0, 255, 255);

                egui_extras::install_image_loaders(ctx);

                egui::Image::new(egui::include_image!("../assets/bg.png"))
                    .paint_at(ui, ui.ctx().screen_rect());

                ui.heading("Welcome to Jean-Baptiste BESNARD's Website");

                let screen = ctx.screen_rect();

                egui::Window::new("Official Photo")
                    .open(&mut self.show_photo)
                    .default_pos(screen.min)
                    .auto_sized()
                    .max_height(screen.max.y / 2.0)
                    .pivot(Align2::CENTER_CENTER)
                    .default_pos(screen.max / 2.0)
                    .show(ui.ctx(), |ui| {
                        ui.image(egui::include_image!("../assets/pic.jpg"));
                    });

                egui::Window::new("Settings")
                    .open(&mut self.show_settings)
                    .default_pos(screen.min)
                    .min_width(screen.max.x / 2.0)
                    .min_height(screen.max.y / 2.0)
                    .pivot(Align2::CENTER_CENTER)
                    .default_pos(screen.max / 2.0)
                    .show(ui.ctx(), |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Background color: ");
                            if ui.color_edit_button_rgb(&mut self.bg_color).changed() {
                                ctx.style_mut(|st| {
                                    st.visuals.window_fill = Color32::from_rgb(
                                        (self.bg_color[0] * 255.0) as u8,
                                        (self.bg_color[1] * 255.0) as u8,
                                        (self.bg_color[2] * 255.0) as u8,
                                    )
                                })
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.label("Text color: ");
                            if ui.color_edit_button_rgb(&mut self.txt_color).changed() {
                                ctx.style_mut(|st| {
                                    st.visuals.override_text_color = Some(Color32::from_rgb(
                                        (self.txt_color[0] * 255.0) as u8,
                                        (self.txt_color[1] * 255.0) as u8,
                                        (self.txt_color[2] * 255.0) as u8,
                                    ))
                                })
                            }
                        });
                    });

                /* Calculator */
                egui::Window::new("Calculator")
                    .open(&mut self.show_calculator)
                    .default_pos(screen.min)
                    .min_width(screen.max.x / 2.0)
                    .min_height(screen.max.y / 2.0)
                    .pivot(Align2::CENTER_CENTER)
                    .default_pos(screen.max / 2.0)
                    .show(ui.ctx(), |ui| {
                        egui::ScrollArea::vertical()
                            .max_height(screen.max.y / 2.0)
                            .stick_to_bottom(true)
                            .show(ui, |ui| {
                                ui.text_edit_multiline(&mut self.calculator_buff);
                            });

                        ui.horizontal(|ui| {
                            let tb = ui.text_edit_singleline(&mut self.calculator_formula);

                            if ui.input(|ui| ui.key_pressed(egui::Key::Enter)) {
                                match meval::eval_str(&self.calculator_formula) {
                                    Ok(r) => {
                                        self.calculator_buff +=
                                            format!("{} = {}\n", self.calculator_formula, r)
                                                .as_str();
                                    }
                                    Err(e) => {
                                        self.calculator_buff += format!(
                                            "Error in {} : {}\n",
                                            self.calculator_formula, e
                                        )
                                        .as_str();
                                    }
                                }
                                tb.request_focus();
                            }
                        });
                    });

                for (title, win) in self.md_views.iter_mut() {
                    egui::Window::new(title.clone())
                        .open(&mut win.is_visible)
                        .default_pos(screen.min)
                        .min_width(screen.max.x / 2.0)
                        .min_height(screen.max.y / 2.0)
                        .max_height(screen.max.y * 0.9)
                        .pivot(Align2::CENTER_CENTER)
                        .default_pos(screen.max / 2.0)
                        .show(ui.ctx(), |ui| {
                            egui::ScrollArea::vertical().show(ui, |ui| {
                                CommonMarkViewer::new("viewer")
                                    .default_implicit_uri_scheme("https://www.jbbesnard.pro/")
                                    .show(ui, &mut self.cache, &win.markdown);
                            });
                        });
                }

                egui::TopBottomPanel::bottom("bottom_panel")
                    .frame(egui::Frame::none().fill(egui::Color32::from_hex("#afa8af").unwrap()))
                    .show(ctx, |ui| {
                        egui::menu::bar(ui, |ui| {
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    let date = chrono::offset::Local::now();
                                    let sdate = format!("{}", date.format("%Y-%m-%d %H:%M:%S"));
                                    ui.label(sdate);
                                    ctx.request_repaint_after(Duration::from_secs(1));

                                    if ui.button("Settings").clicked() {
                                        self.show_settings = true;
                                    }
                                },
                            );
                        });
                    });
            });
    }
}
