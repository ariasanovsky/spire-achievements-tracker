use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, BufWriter},
    path::PathBuf,
};

use spire_achievements_tracker::{achievement_list::Achievements, settings::Settings};

fn settings() -> Result<Settings, Box<dyn std::error::Error>> {
    let settings_path = PathBuf::from("settings.json");
    if settings_path.is_file() {
        let settings_file = File::open(&settings_path)?;
        let reader = BufReader::new(settings_file);
        let settings: Settings = serde_json::from_reader(reader)?;
        Ok(settings)
    } else {
        let settings = Settings::default();
        let writer = BufWriter::new(File::create(&settings_path)?);
        serde_json::to_writer_pretty(writer, &settings)?;
        Ok(settings)
    }
}

use eframe::epi;

struct App {
    settings: Settings,
    rename_map: HashMap<String, String>,
    achievements: Achievements,
}

impl App {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let settings = settings()?;
        let rename_map = settings.name_map();
        let achievements = settings.achievements().unwrap_or_default();

        Ok(Self {
            settings,
            rename_map,
            achievements,
        })
    }
}

impl epi::App for App {
    fn update(&mut self, ctx: &egui::CtxRef, _: &mut epi::Frame<'_>) {
        if let Ok(achievements) = self.settings.achievements() {
            self.achievements = achievements;
        } else {
            return;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("").show(ui, |ui| {
                for (i, (achievement, _)) in self.settings.achievements.iter().enumerate() {
                    let color = if self.achievements.values.contains(achievement) {
                        egui::Color32::from_rgb(0, 255, 0)
                    } else {
                        egui::Color32::from_rgb(255, 0, 0)
                    };
                    let text = format!(
                        "{}{}",
                        self.rename_map.get(achievement).unwrap_or(achievement),
                        " ".repeat(self.settings.text_padding)
                    );
                    ui.colored_label(color, text);
                    if (i + 1) % self.settings.row_width == 0 {
                        ui.end_row();
                    }
                }
            })
        });
    }

    fn name(&self) -> &str {
        "Achievement Tracker"
    }

    fn setup(
        &mut self,
        ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
        let mut fonts = egui::FontDefinitions::default();
        fonts.family_and_size.insert(
            egui::TextStyle::Body,
            (
                egui::FontFamily::Proportional,
                self.settings.font_size as f32,
            ),
        );

        ctx.set_fonts(fonts);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::new()?;
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
