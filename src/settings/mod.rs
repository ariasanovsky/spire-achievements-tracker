use std::path::PathBuf;

use crate::achievement_list::Achievements;

mod serialize;

const DEFAULT_PAIRS: [(&'static str, &'static str); 45] = [
    ("ADRENALINE", "Adrenaline"),
    ("AMETHYST", "Amethyst"),
    ("AMETHYST_PLUS", "Amethyst+"),
    ("ASCEND_0", "Ascend 0"),
    ("ASCEND_10", "Ascend 10"),
    ("ASCEND_20", "Ascend 20"),
    ("AUTOMATON", "The Automaton"),
    ("BARRICADED", "Barricaded"),
    ("CATALYST", "Catalyst"),
    ("CHAMP", "The Champion"),
    ("COLLECTOR", "The Collector"),
    ("COME_AT_ME", "Come At Me"),
    ("COMMON_SENSE", "Common Sense"),
    ("CROW", "The Crow"),
    ("DONUT", "Ooh Donut!"),
    ("EMERALD", "Emerald"),
    ("EMERALD_PLUS", "Emerald+"),
    ("ETERNAL_ONE", "Eternal One"),
    ("FOCUSED", "Focused"),
    ("GHOST_GUARDIAN", "The Ghost"),
    ("GUARDIAN", "The Guardian"),
    ("IMPERVIOUS", "Impervious"),
    ("INFINITY", "Infinity"),
    ("JAXXED", "Jaxxed"),
    ("LUCKY_DAY", "My Lucky Day"),
    ("MINIMALIST", "Minimalist"),
    ("NEON", "Neon"),
    ("NINJA", "Ninja"),
    ("ONE_RELIC", "Who Needs Relics?"),
    ("PERFECT", "Perfect"),
    ("PLAGUE", "Plague"),
    ("POWERFUL", "Powerful"),
    ("PURITY", "Purity"),
    ("RUBY", "Ruby"),
    ("RUBY_PLUS", "Ruby+"),
    ("SAPPHIRE", "Sapphire"),
    ("SAPPHIRE_PLUS", "Sapphire+"),
    ("SHAPES", "The Shapes"),
    ("SHRUG_IT_OFF", "Shrug It Off"),
    ("SPEED_CLIMBER", "Speed Climber"),
    ("THE_ENDING", "The End?"),
    ("THE_PACT", "The Pact"),
    ("THE_TRANSIENT", "The Transient"),
    ("TIME_EATER", "The Time Eater"),
    ("YOU_ARE_NOTHING", "You Are Nothing")
];


#[derive(Debug)]
pub struct Settings {
    pub preferences: PathBuf,
    pub save_slot: usize,
    pub width: usize,
    pub achievements: [(String, String); 45],
}

impl Settings {
    fn achievements_path(&self) -> PathBuf {
        self.preferences
        .join(match self.save_slot {
            0 => "STSAchievements".to_string(),
            i => format!("{i}_STSAchievements"),
        })
    }

    pub fn achievements(&self) -> Result<Achievements, Box<dyn std::error::Error>> {
        let path = self.achievements_path();
        Achievements::from_path(&path)
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            preferences: PathBuf::from("C:/Program Files (x86)/Steam/steamapps/common/SlayTheSpire/preferences"),
            save_slot: Default::default(),
            width: 10,
            achievements: DEFAULT_PAIRS
            .map(|(a, b)| (a.to_string(), b.to_string())),
        }
    }
}
