use std::{path::PathBuf, io::BufRead};

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

#[derive(Debug, Default)]
#[allow(non_snake_case)]
pub struct Achievements {
    values: Vec<String>
}

impl Achievements {
    pub fn from_path(path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let values = reader
        .lines()
        .filter_map(Result::ok)
        .filter_map(|line| {
            line.split_once(": ")
            .map(|(key, _)|
                key
                .trim()
                .trim_matches('"')
                .into()
        )})
        .collect();
        Ok(Self { values })
    }
}

#[cfg(test)]
mod test_achievement_list {
    use std::path::PathBuf;

    use crate::achievement_list::Achievements;

    #[test]
    fn test_deserialize_current_achievements() {
        let achievements_path = PathBuf::from("C:/Program Files (x86)/Steam/steamapps/common/SlayTheSpire/preferences/STSAchievements");
        let achievements = Achievements::from_path(&achievements_path).unwrap();
        dbg!(achievements);
    }
}