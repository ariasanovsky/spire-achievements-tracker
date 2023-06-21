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

    fn play_reset_sound(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(path) = &self.settings.reset_sound {
            let (_stream, stream_handle) = OutputStream::try_default()?;
            let src = File::open(path)?;
            let source = Decoder::new(BufReader::new(src))?;
            stream_handle.play_raw(source.convert_samples())?;
            std::thread::sleep(std::time::Duration::from_secs(3));
        }
        Ok(())
    }

    fn play_achievement_sound(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(path) = &self.settings.achievement_sound {
            let (_stream, stream_handle) = OutputStream::try_default()?;
            let src = File::open(path)?;
            let source = Decoder::new(BufReader::new(src))?;
            stream_handle.play_raw(source.convert_samples())?;
            std::thread::sleep(std::time::Duration::from_secs(3));
        }
        Ok(())
    }
}

impl epi::App for App {
    fn update(&mut self, ctx: &egui::CtxRef, _: &mut epi::Frame<'_>) {
        if let Ok(achievements) = self.settings.achievements() {
            match achievements.partial_cmp(&self.achievements) {
                Some(std::cmp::Ordering::Greater) => {
                    let _ = self.play_achievement_sound();
                }
                Some(std::cmp::Ordering::Less) => {
                    let _ = self.play_reset_sound();
                }
                Some(std::cmp::Ordering::Equal) => {}
                None => {
                    let _ = self.play_reset_sound();
                }
            }
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

use rodio::{source::Source, Decoder, OutputStream};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let (_stream, stream_handle) = OutputStream::try_default()?;
    // let src = File::open("./.media/SOTE_SFX_Relic_Tingsha.ogg")?;
    // let source = Decoder::new(BufReader::new(src))?;
    // stream_handle.play_raw(source.convert_samples())?;
    // std::thread::sleep(std::time::Duration::from_secs(5));
    // let src = File::open("./.media/SOTE_SFX_Relic_Tingsha.ogg")?;
    // let source = Decoder::new(BufReader::new(src))?;
    // stream_handle.play_raw(source.convert_samples())?;

    let app = App::new()?;
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
