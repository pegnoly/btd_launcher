use tauri::{Manager, State, AppHandle, api::dialog::FileDialogBuilder, App};
use patcher::{Patcher,
    map::{Unpacker, Map, template::{Template, TemplateTransferable, TemplatesInfoModel}}, 
    patch_strategy::{
        base::{BaseCreator, TemplateInfoGenerator}, 
        building::{BuildingModifyable, BuildingCreatable}, 
        treasure::TreasurePatcher, 
        player::{PlayersPatcher, TeamsGenerator}, light::LightPatcher, quest::QuestPatcher,
        town::TownPatcher, misc::{MoonCalendarWriter, OutcastFilesWriter, MapNameChanger, UndergroundTerrainCreator},
        win_condition::{MapWinCondition, FinalBattleTime, WinConditionWriter, WinConditionFinalBattleFileProcessor, ResourceWinInfo, EconomicWinConditionTextProcessor, CaptureObjectWinConditionTextProcessor}
    }, CodeGenerator, FileWriter, TextProcessor
};
use serde::{Serialize, Deserialize};
use tokio::{sync::Mutex, io::AsyncWriteExt};
use zip::write::FileOptions;
use std::{path::PathBuf, collections::HashMap, f64::consts::E, io::Read, cell::{RefCell, RefMut}};
use std::ops::Range;
use std::io::Write;

use crate::{file_management::PathManager, update_manager::SingleValuePayload};

// frontend communication structs.
#[derive(Serialize, Clone, Debug)]
pub struct MapDisplayableInfo {
    pub file_name: String,
    pub template: TemplateTransferable,
    pub players_count: u8,
}

pub struct ActivityInfo {
    pub active: bool
}

pub struct PatcherManager {
    pub activity: Mutex<ActivityInfo>,
    pub map: Mutex<Option<Map>>,
    pub templates_model: Mutex<TemplatesInfoModel>,
    pub config_path: PathBuf
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MapDisplayableName {
    pub name: String
}

#[derive(Debug, Serialize, Clone)]
pub struct PatcherVisibility {
    pub visible: bool
}

#[tauri::command]
pub async fn show_patcher(app: AppHandle, patcher_manager: State<'_, PatcherManager>) -> Result<(), ()> {
    let mut activity = patcher_manager.activity.lock().await;
    if activity.active == false {
        activity.active = true;
        app.app_handle().emit_to("main", "patcher_visibility_changed", SingleValuePayload {
            value: false
        }).unwrap();
    }
    else {
        activity.active = false;
        app.app_handle().emit_to("main", "patcher_visibility_changed", SingleValuePayload {
            value: true
        }).unwrap();
    }

    Ok(())
}

/// Invoked when user clicks on map_pick button of patcher. Creates an open file dialog and send picked map path to frontend.
/// Actually i think its a good idea to unpack map also here cause frontend interaction is unnessessary here, but can't solve troubles with moving States into closure now.
#[tauri::command]
pub async fn pick_map(
    app: AppHandle, 
    path_manager: State<'_, PathManager>
) -> Result<(), ()> {
    let file_dialog = FileDialogBuilder::new()
        .add_filter("hommV maps", &["h5m"])
        .set_directory(path_manager.maps());
    file_dialog.pick_file(move |file_path| {
        match file_path {
            Some(file) => {
                app.emit_to("main", "map_picked", SingleValuePayload {value: file.to_str().unwrap().to_string()});
            }
            None => {}
        }
    });
    Ok(())
}

/// Invoked when map_picked event is listened on frontend.
/// Unpacks map files into temp directory.
/// Assigns unpacked map to patcher manager.
/// Detects some base information to display it of frontend and this also useful for some patches.
/// Returns nessessary information about map to display on frontend.
#[tauri::command]
pub async fn unpack_map(
    app: AppHandle,
    patcher_manager: State<'_, PatcherManager>, 
    map_path: String
) -> Result<MapDisplayableInfo, ()> {
    let mut map = Unpacker::unpack_map(&PathBuf::from(&map_path));
    map.init_write_dirs();
    let mut map_holder = patcher_manager.map.lock().await;
    let mut templates_holder = patcher_manager.templates_model.lock().await;
    *map_holder = Some(map);
    let template = map_holder.as_mut().unwrap().detect_template(&templates_holder).unwrap();
    let tag_info = map_holder.as_ref().unwrap().detect_tag_info().unwrap();
    //println!("tag info: {:?}", &tag_info);
    for i in 1..&tag_info.players_count + 1 {
        map_holder.as_mut().unwrap().teams_info[i] = i;
    }
    map_holder.as_mut().unwrap().size = tag_info.size as usize;
    Ok((MapDisplayableInfo {
            file_name: map_path.split("\\").last().unwrap().to_string(),
            players_count: tag_info.players_count as u8,
            template: template
        }
    ))
}

/// Invoked when user selects new team for some player.
#[tauri::command]
pub async fn update_player_team_info(
    patcher_manager: State<'_, PatcherManager>,
    player: usize,
    team: usize
) -> Result<(), ()> {
    let mut map_holder = patcher_manager.map.lock().await;
    println!("Updating player's {} team info {} from frontend", &player, &team);
    map_holder.as_mut().unwrap().teams_info[player] = team;
    Ok(())
}

/// Invoked when user checks use_night_lights setting.
#[tauri::command]
pub async fn set_night_lights_setting(
    patcher_manager: State<'_, PatcherManager>,
    use_night_lights: bool
) -> Result<(), ()> {
    let mut map_holder = patcher_manager.map.lock().await;
    map_holder.as_mut().unwrap().settings.use_night_lights = use_night_lights;
    println!("Updating night_light setting from frontend {}", use_night_lights);
    Ok(())
}

/// Invoked when user checks set_weeks_only setting.
#[tauri::command]
pub async fn set_weeks_only_setting(
    patcher_manager: State<'_, PatcherManager>,
    weeks_only: bool
) -> Result<(), ()> {
    let mut map_holder = patcher_manager.map.lock().await;
    map_holder.as_mut().unwrap().settings.only_neutral_weeks = weeks_only;
    println!("Updating weeks_only setting from frontend {}", weeks_only);
    Ok(())
}

/// Invoked when used somehow modifies final_battle setting.
#[tauri::command]
pub async fn update_final_battle_setting(
    patcher_manager: State<'_, PatcherManager>,
    is_enabled: bool,
    final_battle_time: Option<FinalBattleTime>
) -> Result<(), ()> {
    let mut map_holder = patcher_manager.map.lock().await;
    if is_enabled == true {
        map_holder.as_mut().unwrap().set_win_condition("final", MapWinCondition::Final(final_battle_time.unwrap()));
        println!("New final battle timing is {:?}", &map_holder.as_ref().unwrap().conds.get("final"));
    }
    else {
        map_holder.as_mut().unwrap().remove_win_condition("final");
    }
    Ok(())
}

/// Invoked when used somehow modifies economic_victory setting.
#[tauri::command]
pub async fn update_economic_victory_setting(
    patcher_manager: State<'_, PatcherManager>,
    is_enabled: bool,
    resource_info: Option<ResourceWinInfo>
) -> Result<(), ()> {
    let mut map_holder = patcher_manager.map.lock().await;
    if is_enabled == true {
        map_holder.as_mut().unwrap().set_win_condition("economic", MapWinCondition::Economic(resource_info.unwrap()));
        println!("New economic win info is {:?}", &map_holder.as_ref().unwrap().conds.get("economic"));
    }
    else {
        map_holder.as_mut().unwrap().remove_win_condition("economic");
    }
    Ok(())
}

/// Invoked when user somehow modifies capture_object setting.
#[tauri::command]
pub async fn update_capture_object_setting(
    patcher_manager: State<'_, PatcherManager>,
    is_enabled: bool,
    delay: Option<u8>
) -> Result<(), ()> {
    let mut map_holder = patcher_manager.map.lock().await;
    if is_enabled == true {
        map_holder.as_mut().unwrap().set_win_condition("capture", MapWinCondition::Capture(delay.unwrap()));
        println!("New capture info is {:?}", &map_holder.as_ref().unwrap().conds.get("capture"));
    }
    else {
        map_holder.as_mut().unwrap().remove_win_condition("capture");
    }
    Ok(())
}

#[derive(serde::Deserialize)]
pub struct MapPackable {
    pub name: String,
    pub dir: PathBuf
}

/// Invoked when user activates patch process.
/// Creates all necessary patches and runs it.
/// Repacks map after it.
#[tauri::command]
pub async fn patch_map(
    app: AppHandle, 
    patcher_manager: State<'_, PatcherManager>, 
) -> Result<(), ()> {
    let mut map_holder = patcher_manager.map.lock().await;
    let base_creator = BaseCreator::new(
        map_holder.as_ref().unwrap().get_write_dir(String::from("main")),
        patcher_manager.config_path.join("patcher\\adds\\common\\")
    );
    let teams = map_holder.as_ref().unwrap().teams_info.clone();
    let mut players_patcher = PlayersPatcher::new(teams.clone());
    let mut building_modifyable = BuildingModifyable::new(
        patcher_manager.config_path.join("patcher\\banks_types.json"),
        patcher_manager.config_path.join("patcher\\new_buildings_types.json"),
        map_holder.as_ref().unwrap().template()
    );
    let light_patcher = LightPatcher::new(patcher_manager.config_path.join("patcher\\lights.json"), 
        map_holder.as_ref().unwrap().settings.use_night_lights);
    let mut treasure_patcher = TreasurePatcher::new();
    let mut town_patcher = TownPatcher::new(
        patcher_manager.config_path.join("patcher\\town_types.json"), 
        patcher_manager.config_path.join("patcher\\town_specs.json"),
        map_holder.as_ref().unwrap().template(),
        map_holder.as_ref().unwrap().has_win_condition("capture")
    );
    let underground_terrain_creator = UndergroundTerrainCreator::new(
        map_holder.as_ref().unwrap().has_win_condition("final"),
        patcher_manager.config_path.join("patcher\\adds\\terrains\\"),
        map_holder.as_ref().unwrap().get_write_dir(String::from("main")),
        map_holder.as_ref().unwrap().size
    );
    let mut win_condition_writer = WinConditionWriter {
        conditions: &map_holder.as_ref().unwrap().conds,
        quest_path: &patcher_manager.config_path.join("patcher\\win_condition_quests.xml"),
        write_dir: &map_holder.as_ref().unwrap().get_write_dir(String::from("main")),
        quest_info_path: &patcher_manager.config_path.join("patcher\\adds\\win_conditions\\"),
    };
    let p = Patcher::new()
        .with_root(map_holder.as_ref().unwrap().map_xdb()).unwrap()
        .with_creatable("AmbientLight", &light_patcher, false)
        .with_creatable("GroundAmbientLights", &light_patcher, false)
        .with_creatable("MapScript", &base_creator, true)
        .with_creatable("CustomTeams", &base_creator, true)
        .with_creatable("RMGmap", &base_creator, true)
        .with_creatable("objects", &BuildingCreatable::new(patcher_manager.config_path.join("patcher\\")), false)
        .with_creatable("HasUnderground", &underground_terrain_creator, true)
        .with_creatable("UndergroundTerrainFileName", &underground_terrain_creator, true)
        .with_modifyable("AdvMapTreasure", &mut treasure_patcher)
        .with_modifyable("AdvMapBuilding", &mut building_modifyable)
        .with_modifyable("AdvMapTown", &mut town_patcher)
        .with_modifyable("players", &mut players_patcher)
        .with_modifyable("Secondary", &mut QuestPatcher::new(patcher_manager.config_path.join("patcher\\test_quest.xml")))
        .with_modifyable("Primary", &mut win_condition_writer)
        .run();
    let g = CodeGenerator::new()
        .with(&building_modifyable)
        .with(&treasure_patcher)
        .with(&win_condition_writer)
        .with(&TemplateInfoGenerator{template: &map_holder.as_ref().unwrap().template._type})
        .run(&map_holder.as_ref().unwrap().map_xdb().parent().unwrap().to_path_buf());
    let f = FileWriter::new()
        .with(&MoonCalendarWriter::new(
            map_holder.as_ref().unwrap().settings.only_neutral_weeks,
            map_holder.as_ref().unwrap().get_write_dir(String::from("game_mechanics")),
            patcher_manager.config_path.join("patcher\\adds\\moon_calendar\\Default.xdb")
        ))
        .with(&OutcastFilesWriter::new(
            map_holder.as_ref().unwrap().template(),
            &map_holder.as_ref().unwrap().get_write_dir(String::from("game_mechanics")),
            &patcher_manager.config_path.join("patcher\\adds\\outcast\\Summon_Creatures.xdb")
        ))
        .with(&base_creator)
        .with(&underground_terrain_creator)
        .with(&win_condition_writer)
        .run();
    // map-tag patch
    let pp = Patcher::new()
        .with_root(map_holder.as_ref().unwrap().map_tag()).unwrap()
        .with_creatable("HasUnderground", &underground_terrain_creator, true)
        .with_modifyable("teams", &mut TeamsGenerator::new(teams.clone()))
        .run();
    let t = TextProcessor::new(map_holder.as_ref().unwrap().map_name())
        .with(&MapNameChanger{})
        .run();
    let m = MapPackable {
        name: map_holder.as_ref().unwrap().name.clone(),
        dir: map_holder.as_ref().unwrap().dir.clone()
    };
    // win condition quests processing
    let fbtp = TextProcessor::new(&map_holder.as_ref().unwrap().map_xdb().parent().unwrap().join("final_battle_desc.txt"))
        .with(&WinConditionFinalBattleFileProcessor {
            final_battle_time: map_holder.as_ref().unwrap().conds.get("final")
        })
        .run();
    let etp = TextProcessor::new(&map_holder.as_ref().unwrap().map_xdb().parent().unwrap().join("economic_desc.txt"))
        .with(&EconomicWinConditionTextProcessor {
            resource_info: map_holder.as_ref().unwrap().conds.get("economic")
        })
        .run();
    let cotp = TextProcessor::new(&map_holder.as_ref().unwrap().map_xdb().parent().unwrap().join("capture_object_desc.txt"))
        .with(&CaptureObjectWinConditionTextProcessor {
            delay_info: map_holder.as_ref().unwrap().conds.get("capture"),
            town_name: &town_patcher.neutral_town_name,
        })
        .run();
    zip_map(m);
    Ok(())
}

use walkdir::WalkDir;

/// Creates patched map file from temp directory.
#[tauri::command]
pub fn zip_map(map: MapPackable)  {
    let mut zip_file = std::fs::File::create(
        map.dir.parent().unwrap().join(&map.name)
    ).unwrap();
    let mut map_zipped = zip::ZipWriter::new(zip_file);
    for entry in WalkDir::new(&map.dir) {
        match entry {
            Ok(e) => {
                let path = e.path();
                println!("path: {:?}", path);
                if path.is_file() {
                    let file_name = path.strip_prefix(&map.dir).unwrap().to_str().unwrap();
                    let mut curr_file = std::fs::File::open(&path).unwrap();
                    let mut s = Vec::new();
                    curr_file.read_to_end(&mut s);
                    map_zipped.start_file(file_name, Default::default());
                    map_zipped.write_all(s.as_slice());
                }
            }
            Err(err) => {
                println!("Error while packing")
            }
        }
    }
    map_zipped.finish().unwrap();
    std::fs::remove_dir_all(&map.dir);
}