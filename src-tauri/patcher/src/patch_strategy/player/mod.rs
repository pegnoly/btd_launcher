use quick_xml::Writer;
use serde::{Serialize, Deserialize};
use homm5_types::player::Player;
use super::PatchModifyable;
use crate::map::MapTeamsCount;

///PlayersPatcher is a modifyable patch strategy that sets teams info for active map players.
pub struct PlayersPatcher {
    players_count: usize,
    teams_info: Vec<usize>
}

impl PlayersPatcher {
    pub fn new(teams_info: Vec<usize>) -> Self {
        PlayersPatcher {
            players_count: 0,
            teams_info: teams_info
        }
    }
}

impl PatchModifyable for PlayersPatcher {
    fn try_modify(&mut self, text: &String, writer: &mut Writer<&mut Vec<u8>>) {
        let players_info: Result<Vec<Player>, quick_xml::DeError> = quick_xml::de::from_str(&text);
        match players_info {
            Ok(mut players) => {
                let patched_players: Vec<Player> = players.iter_mut()
                    .map(|p| {
                        if p.active_player == true {
                            self.players_count += 1;
                            p.team = self.teams_info[self.players_count];
                        }
                        p.to_owned()
                    })
                    .collect();
                    writer.write_event(quick_xml::events::Event::Start(quick_xml::events::BytesStart::new("players"))).unwrap();
                    for player in patched_players {
                        writer.write_serializable("Item", &player).unwrap();
                    }
            }
            Err(_e) => {
                println!("Error catched when patching players teams");
            }
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

impl PatchModifyable for TeamsGenerator {
    /// From map's teams info create vec [team] = players_count and write to file only elements with count > 0
    fn try_modify(&mut self, _text: &String, writer: &mut quick_xml::Writer<&mut Vec<u8>>) {
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