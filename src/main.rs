use core::panic;
use std::{path::PathBuf, fs::File, io::{BufReader, BufWriter}, collections::HashMap};

use spire_achievements_tracker::{settings::Settings, achievement_list::Achievements};

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

use egui::{Align2, CentralPanel, CtxRef, Label, TextEdit, Ui, Window, Grid};
use eframe::{egui::CtxRef as EguiCtxRef, epi};

#[derive(Default)]
struct App {
    settings: Settings,
    achievements: Achievements,
    text: Vec<String>,
}

impl epi::App for App {
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        self.text = self.settings.achievements
        .iter()
        .map(|(_, achievement)| {
            format!("{achievement:?}")
        }).collect();

        let mut fonts = egui::FontDefinitions::default();
        fonts.family_and_size.insert(
            egui::TextStyle::Body,
            (egui::FontFamily::Proportional, 30.0)
        );

        ctx.set_fonts(fonts);

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("my_grid")
            .show(ui, |ui| {
                for (i, (achievement, _)) in self.settings.achievements
                .iter().enumerate() {
                    ui.colored_label(
                        egui::Color32::from_rgb(255, 0, 0),
                        format!("{achievement}    ")
                    );
                    if (i + 1) % self.settings.width == 0 {
                        ui.end_row();
                    }
                }
            })
            
            
            
            // ui.horizontal_wrapped(|ui| {
            //     ui.style_mut().spacing.item_spacing.x = 100.0; // Set horizontal spacing
            //     ui.style_mut().spacing.item_spacing.y = 10.0; // Set vertical spacing
                
            //     let mut grid = egui::Grid::new("my_grid");
            //     for 
            //     //ui.
            //     for text in &mut self.text {
            //         ui.add(
            //             TextEdit::multiline(text)
            //             .desired_rows(1)
            //             .desired_width(80.0)
            //             .text_color(egui::Color32::from_rgb(255, 0, 0))
            //         );
            //     }
            // }
            // );
        });
    }

    fn name(&self) -> &str {
        "Achievement Tracker"
    }
}

fn main() {
    let settings = settings().unwrap();
    dbg!(&settings);

    let achievements = settings.achievements().unwrap_or_default();
    dbg!(&achievements);
    
    let mut map = HashMap::new();
    for (a, b) in &settings.achievements {
        map.insert(a.clone(), b.clone());
    }
    
    let app = App {
        settings,
        achievements,
        text: vec![],
    };

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}

