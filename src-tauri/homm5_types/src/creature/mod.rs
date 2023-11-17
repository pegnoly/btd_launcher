use serde::{Serialize, Deserialize};
use crate::common::FileRef;

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Spell {
    pub Spell: String,
    pub Mastery: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct MagicElement {
    pub First: String,
    pub Second: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Resources {
    pub Wood: u16,
    pub Ore: u16,
    pub Mercury: u16,
    pub Crystal: u16,
    pub Sulfur: u16,
    pub Gem: u16,
    pub Gold: u32
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Abilities {
    #[serde(rename = "Item")]
    pub Abilities: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KnownSpells {
    #[serde(rename = "Item")]
    pub spells: Option<Vec<Spell>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct CreatureVisual {
    pub CreatureNameFileRef: Option<FileRef>,
    pub DescriptionFileRef: Option<FileRef>,
    pub Icon128: Option<FileRef>
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct AdvMapCreatureShared {
    pub AttackSkill: u16,
    pub DefenceSkill: u16,
    pub MinDamage: u16,
    pub MaxDamage: u16,
    pub Speed: u8,
    pub Initiative: u16,
    pub Flying: bool,
    pub Health: u32,
    pub KnownSpells: KnownSpells,
    pub SpellPoints: u16,
    pub Exp: u64,
    pub Power: u64,
    pub CreatureTier: u8,
    pub Upgrade: bool,
    pub PairCreature: String,
    pub CreatureTown: String,
    pub MagicElement: MagicElement,
    pub WeeklyGrowth: u16,
    pub Cost: Resources,
    pub SubjectOfRandomGeneration: bool,
    pub MonsterShared: Option<FileRef>,
    pub CombatSize: u8,
    pub Visual: Option<FileRef>,
    pub Range: i8,
    pub BaseCreature: Option<String>,
    #[serde(rename = "Item")]
    pub Upgrades: Option<Vec<String>>,
    pub Abilities: Abilities,
    pub VisualExplained: Option<CreatureVisual>
}