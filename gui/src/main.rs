use eframe::{egui, epaint::Vec2};
use solutions::DAYS;

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(Vec2::new(1000.0, 800.0)),
        ..Default::default()
    };
    eframe::run_native("AOC 2022", options, Box::new(|cc| Box::new(App::new(cc))))
}

fn setup_style(ctx: &egui::Context) {
    use egui::{FontDefinitions, FontFamily};

    let mut fonts = FontDefinitions::default();
    let font_name = "Cousine";
    fonts.font_data.insert(
        font_name.to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/Cousine-Regular.ttf")),
    );
    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .insert(0, font_name.to_owned());
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, font_name.to_owned());
    ctx.set_fonts(fonts);
}

struct App {}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_style(&cc.egui_ctx);
        Self {}
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui_main(ui);
            });
        });
    }
}

fn ui_main(ui: &mut egui::Ui) {
    for (i, day) in DAYS.iter().enumerate() {
        let title = format!("Day {}: {}", i + 1, &day.name);
        if let Some(solution) = &day.solution {
            ui.collapsing(title, |ui| {
                ui_source_code(ui, &solution.source_code);
            });
        } else {
            ui.label(title);
        }
    }
}

fn ui_source_code(ui: &mut egui::Ui, code: &str) {
    ui.add(
        egui::TextEdit::multiline(&mut code.to_string())
            .font(egui::TextStyle::Monospace) // for cursor height
            .code_editor()
            .desired_rows(10)
            .lock_focus(true)
            .desired_width(f32::INFINITY),
    );
}
