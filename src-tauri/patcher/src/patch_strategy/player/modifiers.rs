use homm5_types::{player::{Player, PlayerID, BannedHeroesRaces, AllowedHeroes}, town::TownType};
use strum::IntoEnumIterator;
use crate::patch_strategy::{PatchModifyable, town::TownCrossPatchInfo};
use super::{PlayersInfoProvider, PlayersCrossPatchInfo};

/// Applies teams to active players.
pub struct PlayerTeamSelector<'a> {
    teams_info: &'a Vec<usize>,
    active_players_count: u8  
}

impl<'a> PlayerTeamSelector<'a>  {
    pub fn new(teams: &Vec<usize>) -> Self {
        PlayerTeamSelector { 
            teams_info: (), 
            active_players_count: () 
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
    player_info_provider: &'a PlayersInfoProvider<'a>,
    player_race_provider: &'a TownCrossPatchInfo,
    player_cross_patch_provider: &'a PlayersCrossPatchInfo,
    active_players_count: u8
}

impl<'a> OutcastPlayerHeroSelector<'a> {
    pub fn new(pip: &PlayersInfoProvider, prp: &TownCrossPatchInfo, pcpp: &PlayersCrossPatchInfo, enabled: bool) -> Self  {
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
        if object.active_player == true {
            self.active_players_count += 1;
            // detect player's race
            let race = self.player_race_provider.players_race_info.iter().find(|p| {
                PlayerID::iter().enumerate().filter(|e| e.0 == self.active_players_count)
            });
            match race {
                Some(actual_race) => {
                    // select random hero of this race
                    let hero = self.player_info_provider.get_random_hero_by_race(actual_race.1);
                    self.player_cross_patch_provider.avaliable_heroes.push(*hero.1);
                    let banned_races: Vec<TownType> = TownType::iter().collect();
                    // ban all races
                    object.tavern_filter.banned_heroes_races = Some(BannedHeroesRaces {
                        items: Some(banned_races)
                    });
                    // add selected hero as only allowed
                    object.tavern_filter.allowed_heroes = Some(AllowedHeroes { 
                        items: Some(vec![*hero.0])
                    });
                }
                None => println!("Impossible to detect race of player {}", self.active_players_count)
            }
        }
    }
}