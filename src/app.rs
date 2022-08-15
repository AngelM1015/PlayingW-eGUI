/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct MyApp {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,
}

impl Default for MyApp {

    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Write text into here: ".to_owned(),
            value: 3.3
        }
    }
}

impl MyApp {

    /// Called once before the first frame.

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
         // `cc.egui_ctx.set_visuals`
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        //cc.egui_ctx.set_fonts();
        let mut fonts = egui::FontDefinitions::default();

        // Install my own font (maybe supporting non-latin characters):
        fonts.font_data.insert("my_font".to_owned(),
           egui::FontData::from_static(include_bytes!("/Users/iquit/Desktop/CODE/test/hello/src/assets/fonts/MesloLGS_NF_Regular.ttf"))); // .ttf and .otf supported

        // Put my font first (highest priority):
        fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap()
            .insert(0, "my_font".to_owned());

        // Put my font as last fallback for monospace:
        fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap()
            .push("my_font".to_owned());

        cc.egui_ctx.set_fonts(fonts);
        // Start with the default fonts (we will be adding to them rather than replacing them).
        // Install my own font (maybe supporting non-latin characters).
        // .ttf and .otf files supported.

        Default::default()
    }
}

impl eframe::App for MyApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        egui::Rgba::TRANSPARENT // Make sure we don't paint anything behind the rounded corners
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self { label, value } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        custom_window_frame(ctx, frame, "egui testing frame", |ui| {

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(label);
                println!(">>>{:?}", label);
            });

            let highest_val = 10.0;
            let response = ui.add(
                egui::Slider::new(
                    value, 0.0..= highest_val
                ).text("A a").clamp_to_range(true).show_value(false)
            );

            if ui.button("Increment").clicked() {
                *value += 1.0;
            }

            let resp = ui.add(egui::Slider::new(value, 0.0..= highest_val));
            resp.on_hover_text("Drag me!");

            // to mark the area during development. Will be removed later
            ui.painter().rect_stroke(response.rect, 0.0, (1.0, egui::Color32::WHITE));

            if response.changed(){
                println!(">>>{:?}", value);
                // the actions are taken here
            }

            ui.label(egui::RichText::new(format!("{}", label)).size(21.0));

            ui.label("This is just the contents of the window");
            ui.horizontal(|ui| {
                ui.label("egui theme:");
                egui::widgets::global_dark_light_mode_buttons(ui);
                egui::Window::new("Window").show(ctx, |ui| {
                    ui.label("Windows can be moved by dragging them.");
                });
            });
        });

        // egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        //     // The top panel is often a good place for a menu bar:
        //     egui::menu::bar(ui, |ui| {
        //         ui.menu_button("File", |ui| {
        //             if ui.button("Quit").clicked() {
        //                 frame.quit();
        //             }
        //         });
        //     });
        // });

        // egui::CentralPanel::default().show(ctx, |ui| {
        //     // The central panel the region left after adding TopPanel's and SidePanel's

        //     ui.heading("OS");
        //     ui.spacing_mut().item_spacing.x = 0.0;
        //     if ui.button("hello").clicked() {
        //         println!("hello");
        //     }
        //     egui::warn_if_debug_build(ui);
        // });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }
}

fn custom_window_frame(
    ctx: &egui::Context,
    frame: &mut eframe::Frame,
    title: &str,
    add_contents: impl FnOnce(&mut egui::Ui),
) {
    use egui::*;
    let text_color = ctx.style().visuals.text_color();

    // Height of the title bar
    let height = 28.0;

    CentralPanel::default()
        .frame(Frame::none())
        .show(ctx, |ui| {
            let rect = ui.max_rect();
            let painter = ui.painter();

            // Paint the frame:
            painter.rect(
                rect.shrink(1.0),
                10.0,
                ctx.style().visuals.window_fill(),
                Stroke::new(1.0, text_color),
            );

            // Paint the title:
            painter.text(
                rect.center_top() + vec2(0.0, height / 2.0),
                Align2::CENTER_CENTER,
                title,
                FontId::proportional(height - 2.0),
                text_color,
            );

            // Paint the line under the title:
            painter.line_segment(
                [
                    rect.left_top() + vec2(2.0, height),
                    rect.right_top() + vec2(-2.0, height),
                ],
                Stroke::new(1.0, text_color),
            );

            // Add the close button:
            let close_response = ui.put(
                Rect::from_min_size(rect.left_top(), Vec2::splat(height)),
                Button::new(RichText::new("❌").size(height - 4.0)).frame(false),
            );
            if close_response.clicked() {
                frame.quit();
                println!(">>>close");
            }

            // Interact with the title bar (drag to move window):
            let title_bar_rect = {
                let mut rect = rect;
                rect.max.y = rect.min.y + height;
                rect
            };
            let title_bar_response =
                ui.interact(title_bar_rect, Id::new("title_bar"), Sense::click());
            if title_bar_response.is_pointer_button_down_on() {
                frame.drag_window();
            }

            // Add the contents:
            let content_rect = {
                let mut rect = rect;
                rect.min.y = title_bar_rect.max.y;
                rect
            }
            .shrink(4.0);
            let mut content_ui = ui.child_ui(content_rect, *ui.layout());
            add_contents(&mut content_ui);
        });
}