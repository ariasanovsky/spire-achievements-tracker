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
            raw_achievements: Vec<Vec<String>>,
        }

        let Inner {
            preferences,
            save_slot,
            width,
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
        .unwrap_or(
            DEFAULT_PAIRS
            .map(|(a, b)| (a.into(), b.into()))
        );

        Ok(Self {
            preferences: PathBuf::from(preferences),
            save_slot,
            width,
            achievements,
        })
    }
}

impl Serialize for Settings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        #[derive(Serialize)]
        struct Inner<'a> {
            preferences: &'a str,
            save_slot: usize,
            width: usize,
            raw_achievements: Vec<Vec<&'a str>>,
        }

        let Inner {
            preferences,
            save_slot,
            width,
            raw_achievements,
        } = Inner {
            preferences: self.preferences.to_str().unwrap(),
            save_slot: self.save_slot,
            width: self.width,
            raw_achievements: self.achievements.iter().map(|(a, b)| vec![a.as_str(), b.as_str()]).collect(),
        };

        Inner::serialize(&Inner {
            preferences,
            save_slot,
            width,
            raw_achievements,
        }, serializer)
    }
}

#[cfg(test)]
mod test_serialize_settings {
    use super::*;
    use crate::settings::*;

    #[test]
    fn default_serialize() {
        let settings = Settings::default();
        let serialized = serde_json::to_string(&settings).unwrap();
        dbg!(serialized);
    }
}