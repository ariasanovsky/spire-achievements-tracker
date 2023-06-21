use std::path::PathBuf;

use serde::{Deserialize, Serialize};

// const EXAMPLE_JSON: &str = r"{
//     "SHRUG_IT_OFF": "true",
//     "GHOST_GUARDIAN": "true",
//     "CHAMP": "true",
//     "SLIME_BOSS": "true",
//     "GUARDIAN": "true",
//     "COLLECTOR": "true",
//     "CROW": "true",
//     "RUBY": "true",
//     "SHAPES": "true",
//     "EMERALD": "true",
//     "PLAGUE": "true",
//     "PERFECT": "true",
//     "IMPERVIOUS": "true",
//     "AUTOMATON": "true",
//     "THE_PACT": "true",
//     "TIME_EATER": "true",
//     "ADRENALINE": "true",
//     "POWERFUL": "true",
//     "COME_AT_ME": "true",
//     "PURITY": "true",
//     "NINJA": "true",
//     "INFINITY": "true",
//     "ASCEND_0": "true",
//     "CATALYST": "true",
//     "JAXXED": "true",
//     "SAPPHIRE": "true",
//     "ASCEND_10": "true",
//     "FOCUSED": "true",
//     "BARRICADED": "true",
//     "TRANSIENT": "true",
//     "YOU_ARE_NOTHING": "true",
//     "MINIMALIST": "true",
//     "SPEED_CLIMBER": "true",
//     "LUCKY_DAY": "true",
//     "ASCEND_20": "true",
//     "NEON": "true",
//     "EMERALD_PLUS": "true",
//     "RUBY_PLUS": "true",
//     "SAPPHIRE_PLUS": "true",
//     "THE_ENDING": "true",
//     "COMMON_SENSE": "true",
//     "ONE_RELIC": "true",
//     "DONUT": "true",
//     "ETERNAL_ONE": "true",
//     "AMETHYST": "true"
//   }"

#[derive(Debug, Deserialize, Serialize, Default)]
#[allow(non_snake_case)]
pub struct Achievements {
    SHRUG_IT_OFF: Option<String>,
    GHOST_GUARDIAN: Option<String>,
    CHAMP: Option<String>,
    SLIME_BOSS: Option<String>,
    GUARDIAN: Option<String>,
    COLLECTOR: Option<String>,
    CROW: Option<String>,
    RUBY: Option<String>,
    SHAPES: Option<String>,
    EMERALD: Option<String>,
    PLAGUE: Option<String>,
    PERFECT: Option<String>,
    IMPERVIOUS: Option<String>,
    AUTOMATON: Option<String>,
    THE_PACT: Option<String>,
    TIME_EATER: Option<String>,
    ADRENALINE: Option<String>,
    POWERFUL: Option<String>,
    COME_AT_ME: Option<String>,
    PURITY: Option<String>,
    NINJA: Option<String>,
    INFINITY: Option<String>,
    ASCEND_0: Option<String>,
    CATALYST: Option<String>,
    JAXXED: Option<String>,
    SAPPHIRE: Option<String>,
    ASCEND_10: Option<String>,
    FOCUSED: Option<String>,
    BARRICADED: Option<String>,
    TRANSIENT: Option<String>,
    YOU_ARE_NOTHING: Option<String>,
    MINIMALIST: Option<String>,
    SPEED_CLIMBER: Option<String>,
    LUCKY_DAY: Option<String>,
    ASCEND_20: Option<String>,
    NEON: Option<String>,
    EMERALD_PLUS: Option<String>,
    RUBY_PLUS: Option<String>,
    SAPPHIRE_PLUS: Option<String>,
    THE_ENDING: Option<String>,
    COMMON_SENSE: Option<String>,
    ONE_RELIC: Option<String>,
    DONUT: Option<String>,
    ETERNAL_ONE: Option<String>,
    AMETHYST: Option<String>,
}

impl Achievements {
    pub fn from_path(path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let achievements = serde_json::from_reader(reader)?;
        Ok(achievements)
    }
}

#[cfg(test)]
mod test_achievement_list {
    use std::{path::Path, io::Read};

    use super::*;
    #[test]
    fn test_serialize_default_list() {
        let default_list = Achievements::default();
        let serialized = serde_json::to_string(&default_list).unwrap();
        dbg!(serialized);
    }

    #[test]
    fn test_deserialize_current_achievements() {
        let achievements = Path::new("C:/Program Files (x86)/Steam/steamapps/common/SlayTheSpire/preferences/STSAchievements");
        let mut file = std::fs::File::open(achievements).unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();
        let deserialized: Achievements = serde_json::from_str(&buffer).unwrap();
        dbg!(deserialized);
    }
}