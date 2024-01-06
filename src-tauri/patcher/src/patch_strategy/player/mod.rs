pub mod modifiers;

use std::{collections::HashMap, path::PathBuf};
use homm5_types::{player::Player, town::TownType};
use rand::seq::IteratorRandom;
use strum::IntoEnumIterator;
use super::{PatchModifyable, PatchGroup};

/// Provides players info that can be used across different patches of PlayersPatchesGroup
pub struct PlayersInfoProvider<'a> {
    playable_heroes: &'a HashMap<TownType, HashMap<String, String>>
}

impl<'a> PlayersInfoProvider<'a> {
    pub fn new(config: &PathBuf) -> Self {
        let heroes_de: HashMap<TownType, HashMap<String, String>> = serde_json::from_str(
            &std::fs::read_to_string(config.join("active_heroes.json")).unwrap()
        ).unwrap();
        PlayersInfoProvider { 
            playable_heroes: &heroes_de 
        }
    }
    /// Returns random tuple (hero_script_name, hero_xdb) of given race.
    pub fn get_random_hero_by_race(&self, race: &TownType) -> (&String, &String) {
        let possible_heroes = self.playable_heroes.get(race).unwrap();
        let mut rng = rand::thread_rng();
        possible_heroes.iter().choose(&mut rng)
    }
}

/// Provides players-related information that can be shared between other groups.
pub struct PlayersCrossPatchInfo {
    /// In outcast these heroes must be written into avaliableHeroes tag
    avaliable_heroes: Vec<String>
}

impl PlayersCrossPatchInfo {
    pub fn new() -> Self {
        PlayersCrossPatchInfo {
            avaliable_heroes: vec![]
        }
    }
}

/// This group contains all <player> tag related patches.
pub struct PlayerPatchesGroup<'a> {
    patches: Vec<&'a dyn PatchModifyable<Modifyable = Player>>,
    //getters: Vec<&'a dyn PatchGetter<Patchable = Player, Additional = TownGameInfo>>
}

impl<'a> PlayerPatchesGroup<'a>  {
    pub fn new() -> Self {
        PlayerPatchesGroup { 
            patches: vec![]
        }
    }
}

impl<'a> PatchGroup for PlayerPatchesGroup<'a> {
    fn run(&mut self, text: &String) {
        let players_de: Result<Vec<Player>, quick_xml::DeError> = quick_xml::de::from_str(&text);
        match players_de {
            Ok(mut players) => {
                for mut player in players {
                    for patch in self.patches {
                        patch.try_modify(&mut player);
                    }
                }
            },
            Err(e) => println!("Error deserializing players: {}", e.to_string())
        }
    }

    fn with_modifyable(&mut self, patch: &dyn PatchModifyable<Modifyable = Player>) -> &mut Self {
        self.patches.push(patch)
    }
}

// TeamsGenerator is a modifyable patch strategy that maps teams to their players count in map-tag.xdb file.
// pub struct TeamsGenerator {
//     teams_info: Vec<usize>
// }

// impl TeamsGenerator {
//     pub fn new(teams_info: Vec<usize>) -> Self {
//         TeamsGenerator {
//             teams_info: teams_info 
//         }
//     }
// }

// impl PatchModifyable for TeamsGenerator {
//     /// From map's teams info create vec [team] = players_count and write to file only elements with count > 0
//     fn try_modify(&mut self, _text: &String, writer: &mut quick_xml::Writer<&mut Vec<u8>>) {
//         let mut teams_count = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];
//         self.teams_info.iter()
//             .for_each(|t| {
//                 if *t != 0 {
//                     teams_count[*t as usize] += 1;
//                 }
//             });
//         let actual_teams = teams_count.into_iter()
//             .filter(|count| {
//                 *count > 0 
//             })
//             .collect();
//         writer.write_serializable("teams", &MapTeamsCount { teams: actual_teams }).unwrap();
//     }
// }