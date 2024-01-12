pub mod modifiers;

use std::{collections::HashMap, path::PathBuf};
use homm5_types::{player::Player, town::TownType};
use rand::seq::IteratorRandom;
use crate::map::MapTeamsCount;

use super::{PatchModifyable, PatchGroup, PatchCreatable};

/// Provides players info that can be used across different patches of PlayersPatchesGroup
pub struct PlayersInfoProvider {
    playable_heroes: HashMap<TownType, HashMap<String, String>>,
    already_selected_heroes: Vec<String>
}

impl PlayersInfoProvider {
    pub fn new(config: &PathBuf) -> Self {
        let heroes_de: HashMap<TownType, HashMap<String, String>> = serde_json::from_str(
            &std::fs::read_to_string(config.join("active_heroes.json")).unwrap()
        ).unwrap();
        PlayersInfoProvider { 
            playable_heroes: heroes_de,
            already_selected_heroes: vec![]
        }
    }
    /// Returns random tuple (hero_script_name, hero_xdb) of given race.
    pub fn get_random_hero_by_race(&mut self, race: &TownType) -> (&String, &String) {
        let possible_heroes: Vec<(&String, &String)> = self.playable_heroes.get(race).unwrap().iter()
            .filter(|p| self.already_selected_heroes.contains(p.0) == false)
            .collect();
        let mut rng = rand::thread_rng();
        let selected_hero = *possible_heroes.iter().choose(&mut rng).unwrap();
        self.already_selected_heroes.push(selected_hero.0.clone());
        selected_hero
    }
}

/// Provides players-related information that can be shared between other groups.
pub struct PlayersCrossPatchInfo {
    /// In outcast these heroes must be written into avaliableHeroes tag
    pub avaliable_heroes: Vec<String>
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
    patches: Vec<&'a mut dyn PatchModifyable<Modifyable = Player>>,
    //getters: Vec<&'a dyn PatchGetter<Patchable = Player, Additional = TownGameInfo>>
}

impl<'a> PlayerPatchesGroup<'a>  {
    pub fn new() -> Self {
        PlayerPatchesGroup { 
            patches: vec![]
        }
    }

    pub fn with_modifyable(mut self, patch: &'a mut dyn PatchModifyable<Modifyable = Player>) -> Self {
        self.patches.push(patch);
        self
    }
}

impl<'a> PatchGroup for PlayerPatchesGroup<'a> {
    fn run(&mut self, text: &String, writer: &mut quick_xml::Writer<&mut Vec<u8>>) {
        let players_de: Result<Vec<Player>, quick_xml::DeError> = quick_xml::de::from_str(text);
        match players_de {
            Ok(mut players) => {
                for mut player in players.iter_mut() {
                    for patch in self.patches.iter_mut() {
                        patch.try_modify(&mut player);
                    }
                }
                writer.create_element("players")
                    .write_inner_content(|w| {
                        for player in players.iter() {
                            w.write_serializable("Item", player).unwrap();
                        }
                        Ok(())
                    }).unwrap();
            },
            Err(e) => println!("Error deserializing players: {}", e.to_string())
        }
    }
}

/// TeamsGenerator is a modifyable patch strategy that maps teams to their players count in map-tag.xdb file.
pub struct TeamsGenerator {
    teams_info: Vec<usize>
}

impl TeamsGenerator {
    pub fn new(teams_info: Vec<usize>) -> Self {
        TeamsGenerator {
            teams_info: teams_info 
        }
    }
}

impl PatchCreatable for TeamsGenerator {
    fn try_create(&self, writer: &mut quick_xml::Writer<&mut Vec<u8>>) {
        let mut teams_count = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];
        self.teams_info.iter()
            .for_each(|t| {
                if *t != 0 {
                    teams_count[*t as usize] += 1;
                }
            });
        let actual_teams = teams_count.into_iter()
            .filter(|count| {
                *count > 0 
            })
            .collect();
        writer.write_serializable("teams", &MapTeamsCount { teams: actual_teams }).unwrap();
    }
}