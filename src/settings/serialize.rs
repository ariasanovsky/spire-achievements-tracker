use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::settings::DEFAULT_PAIRS;

use super::Settings;

impl<'de> Deserialize<'de> for Settings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Inner {
            preferences: String,
            save_slot: usize,
            width: usize,
            font_size: usize,
            padding: usize,
            reset_sound: Option<PathBuf>,
            achievement_sound: Option<PathBuf>,
            raw_achievements: Vec<Vec<String>>,
        }

        let Inner {
            preferences,
            save_slot,
            width,
            font_size,
            padding: text_padding,
            reset_sound,
            achievement_sound,
            mut raw_achievements,
        } = Inner::deserialize(deserializer)?;

        let pairs: Vec<(String, String)> = raw_achievements
            .drain(..)
            .filter_map(|mut raw_pair| {
                let mut pair = raw_pair.drain(..2);
                Some((pair.next()?, pair.next()?))
            })
            .collect();

        let achievements: [(String, String); 45] = pairs
            .try_into()
            .unwrap_or(DEFAULT_PAIRS.map(|(a, b)| (a.into(), b.into())));

        Ok(Self {
            preferences: PathBuf::from(preferences),
            save_slot,
            row_width: width,
            font_size,
            text_padding,
            reset_sound,
            achievement_sound,
            achievements,
        })
    }
}

impl Serialize for Settings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(Serialize)]
        struct Inner<'a> {
            preferences: &'a str,
            save_slot: usize,
            width: usize,
            font_size: usize,
            padding: usize,
            reset_sound: Option<&'a PathBuf>,
            achievement_sound: Option<&'a PathBuf>,
            raw_achievements: Vec<Vec<&'a str>>,
        }

        let Inner {
            preferences,
            save_slot,
            width,
            font_size,
            padding,
            reset_sound,
            achievement_sound,
            raw_achievements,
        } = Inner {
            preferences: self.preferences.to_str().unwrap(),
            save_slot: self.save_slot,
            width: self.row_width,
            font_size: self.font_size,
            padding: self.text_padding,
            reset_sound: self.reset_sound.as_ref(),
            achievement_sound: self.achievement_sound.as_ref(),
            raw_achievements: self
                .achievements
                .iter()
                .map(|(a, b)| vec![a.as_str(), b.as_str()])
                .collect(),
        };

        Inner::serialize(
            &Inner {
                preferences,
                save_slot,
                width,
                font_size,
                padding,
                reset_sound,
                achievement_sound,
                raw_achievements,
            },
            serializer,
        )
    }
}

#[cfg(test)]
mod test_serialize_settings {
    use crate::settings::*;

    #[test]
    fn default_serialize() {
        let settings = Settings::default();
        let serialized = serde_json::to_string(&settings).unwrap();
        dbg!(serialized);
    }
}
