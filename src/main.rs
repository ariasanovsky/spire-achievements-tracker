use std::{path::PathBuf, fs::File, io::{BufReader, BufWriter}};

use spire_achievements_tracker::settings::Settings;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings_path = PathBuf::from("settings.json");
    let settings: Settings = if settings_path.is_file() {
        let settings_file = File::open(&settings_path)?;
        let reader = BufReader::new(settings_file);
        serde_json::from_reader(reader)?
    } else {
        let settings = Settings::default();
        let writer = BufWriter::new(File::create(&settings_path)?);
        serde_json::to_writer_pretty(writer, &settings)?;
        settings
    };

    dbg!(&settings);

    let achievements = settings.achievements()?;
    dbg!(&achievements);
    
    Ok(())
}