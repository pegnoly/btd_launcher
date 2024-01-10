use std::{cell::RefCell, vec};

use homm5_types::player::{Player, PlayerID, BannedHeroesRaces, AllowedHeroes};
use strum::IntoEnumIterator;
use crate::patch_strategy::{PatchModifyable, town::PlayerRaceCrossPatchInfo};
use super::{PlayersInfoProvider, PlayersCrossPatchInfo};

/// Applies teams to active players.
pub struct PlayerTeamSelector<'a> {
    teams_info: &'a Vec<usize>,
    active_players_count: usize  
}

impl<'a> PlayerTeamSelector<'a>  {
    pub fn new(teams: &'a Vec<usize>) -> Self {
        PlayerTeamSelector { 
            teams_info: teams, 
            active_players_count: 0 
        }
    }
}

impl<'a> PatchModifyable for PlayerTeamSelector<'a>  {
    type Modifyable = Player;

    fn try_modify(&mut self, object: &mut Self::Modifyable) {
        if object.active_player == true {
            self.active_players_count += 1;
            object.team = self.teams_info[self.active_players_count];
        }
    }
}

/// In Outcast mode bans all races for player and sets only one avaliable hero.
pub struct OutcastPlayerHeroSelector<'a> {
    is_enabled: bool,
    player_info_provider: &'a mut PlayersInfoProvider,
    player_race_provider: &'a RefCell<PlayerRaceCrossPatchInfo>,
    player_cross_patch_provider: &'a mut PlayersCrossPatchInfo,
    active_players_count: usize
}

impl<'a> OutcastPlayerHeroSelector<'a> {
    pub fn new(pip: &'a mut PlayersInfoProvider, prp: &'a RefCell<PlayerRaceCrossPatchInfo>, pcpp: &'a mut PlayersCrossPatchInfo, enabled: bool) -> Self  {
        OutcastPlayerHeroSelector { 
            is_enabled: enabled, 
            player_info_provider: pip,
            player_race_provider: prp,
            player_cross_patch_provider: pcpp,
            active_players_count: 0
        }
    } 
}

impl<'a> PatchModifyable for OutcastPlayerHeroSelector<'a> {
    type Modifyable = Player;

    fn try_modify(&mut self, object: &mut Self::Modifyable) {
        if object.active_player == true && self.is_enabled == true {
            self.active_players_count += 1;
            // detect player's race
            let player_id = PlayerID::iter().enumerate().find(|p| p.0 == self.active_players_count).unwrap().1;
            let provider_borrowed = self.player_race_provider.borrow();
            let race = provider_borrowed.players_race_info.get(&player_id);
            match race {
                Some(actual_race) => {
                    // select random hero of this race
                    let hero = self.player_info_provider.get_random_hero_by_race(actual_race);
                    self.player_cross_patch_provider.avaliable_heroes.push(hero.1.clone());
                    // let mut banned_races = vec![];
                    // TownType::iter().for_each(|t| {
                    //     if t != TownType::TownNoType {
                    //         banned_races.push(t.to_string());
                    //     }
                    // });
                    // ban all races
                    object.tavern_filter.banned_heroes_races = Some(BannedHeroesRaces {
                        items: Some(vec![
                            "TOWN_HEAVEN".to_string(), "TOWN_PRESERVE".to_string(), "TOWN_ACADEMY".to_string(), "TOWN_INFERNO".to_string(),
                            "TOWN_NECROMANCY".to_string(), "TOWN_DUNGEON".to_string(), "TOWN_FORTRESS".to_string(), "TOWN_STRONGHOLD".to_string(),
                        ])
                    });
                    // add selected hero as only allowed
                    object.tavern_filter.allowed_heroes = Some(AllowedHeroes { 
                        items: Some(vec![hero.0.clone()])
                    });
                }
                None => println!("Impossible to detect race of player {}", self.active_players_count)
            }
        }
    }
}

pub struct InactivePlayersTavernFilterRemover {}

impl PatchModifyable for InactivePlayersTavernFilterRemover {
    type Modifyable = Player;

    fn try_modify(&mut self, object: &mut Self::Modifyable) {
        if object.active_player == false {
            object.tavern_filter.banned_heroes_races = None;
            object.tavern_filter.allowed_heroes = None;
        }
    }
}