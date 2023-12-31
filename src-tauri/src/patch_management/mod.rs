use tauri::{Manager, State, AppHandle, api::dialog::FileDialogBuilder, App};
use patcher::{Patcher,
    map::{Unpacker, Map, template::{Template, TemplateTransferable, TemplatesInfoModel, TemplateModeType}}, 
    patch_strategy::{
        base::{BaseCreator, TemplateInfoGenerator}, 
        building::{BuildingModifyable, CommonBuildingCreator}, 
        treasure::TreasurePatcher, 
        player::{PlayersPatcher, TeamsGenerator}, light::LightPatcher, quest::QuestPatcher,
        town::TownPatcher, misc::{MoonCalendarWriter, MapNameChanger, UndergroundTerrainCreator, OutcastMechanicsWriter, OutcastTextWriter},
        win_condition::{MapWinCondition, WinConditionWriter, 
            final_battle::{FinalBattleTime, FinalBattleArenaCreator, WinConditionFinalBattleFileProcessor}, 
            economic::{ResourceWinInfo, EconomicWinConditionTextProcessor}, 
            capture::CaptureObjectWinConditionTextProcessor}, creature::CreatureModifier
    }, CodeGenerator, FileWriter, TextProcessor
};
use serde::{Serialize, Deserialize};
use tokio::{sync::Mutex, io::AsyncWriteExt};
use zip::write::FileOptions;
use std::{path::PathBuf, collections::HashMap, f64::consts::E, io::Read, cell::{RefCell, RefMut}};
use std::ops::Range;
use std::io::Write;

use crate::{file_management::PathManager, SingleValuePayload};

/// This module presents functions for all steps of patching process.
/// The common flow is:
/// Pick map -> Unpack map -> Configure settings for patches -> Run all patches -> Repack new map and save base into separate folder.


/// Contains patcher props used in all steps of patching process.
pub struct PatcherManager {
    /// Map that is currently patched
    pub map: Mutex<Option<Map>>,
    /// Information of possible templates
    pub templates_model: Mutex<TemplatesInfoModel>,
    /// Path of configuration files of pactcher
    pub config_path: PathBuf
}

impl PatcherManager {
    pub fn new(config_path: &PathBuf) -> Self {
        let patcher_config_path = config_path.join("patcher\\");
        let mut templates_file = std::fs::File::open(patcher_config_path.join("templates.json")).unwrap();
        let mut templates_string = String::new();
        templates_file.read_to_string(&mut templates_string).unwrap();
        let templates: TemplatesInfoModel = serde_json::from_str(&templates_string).unwrap();
        println!("Templates: {:?}", &templates);
        PatcherManager { 
            map: Mutex::new(None), 
            templates_model: Mutex::new(templates), 
            config_path: patcher_config_path 
        }
    }
}

/// Contains information about map that will be displayed in frontend.
#[derive(Serialize, Clone, Debug)]
pub struct MapDisplayableInfo {
    pub file_name: String,
    pub template: TemplateTransferable,
    pub players_count: u8,
}

/// Invoked when user clicks on map_pick button of patcher. Creates an open file dialog and send picked map path to frontend.
/// Actually i think its a good idea to unpack map also here cause frontend interaction is unnessessary here, but can't solve troubles with moving States into closure now.
#[tauri::command]
pub async fn pick_map(
    app: AppHandle, 
    path_manager: State<'_, PathManager>
) -> Result<(), ()> {
    let temp = path_manager.maps().join("temp\\");
    if temp.exists() {
        std::fs::remove_dir_all(&temp).unwrap();
    }
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

/// Invoked when user checks disable_neutral_towns_dwells setting
#[tauri::command]
pub async fn set_neutral_towns_dwells_setting(
    patcher_manager: State<'_, PatcherManager>,
    is_disabled: bool
) -> Result<(), ()> {
    let mut map_holder = patcher_manager.map.lock().await;
    map_holder.as_mut().unwrap().settings.disable_neutral_towns_dwells = is_disabled;
    println!("Updating disable_neutral_towns_dwells setting from frontend {}", is_disabled);
    Ok(())
}

/// Invoked when user checks disable_neutral_towns_dwells setting
#[tauri::command]
pub async fn set_enable_new_arts_setting(
    patcher_manager: State<'_, PatcherManager>,
    is_enabled: bool
) -> Result<(), ()> {
    let mut map_holder = patcher_manager.map.lock().await;
    map_holder.as_mut().unwrap().settings.enable_new_arts = is_enabled;
    println!("Updating enable_new_arts setting from frontend {}", is_enabled);
    Ok(())
}

#[tauri::command] 
pub async fn add_game_mode(
    patcher_manager: State<'_, PatcherManager>,
    label: String,
    mode: TemplateModeType
) -> Result<(), ()> {
    let mut map_holder = patcher_manager.map.lock().await;
    map_holder.as_mut().unwrap().add_mode(label, mode);
    println!("Curr modes: {:?}", &map_holder.as_ref().unwrap().modes);
    Ok(())
}

#[tauri::command]
pub async fn remove_game_mode(
    patcher_manager: State<'_, PatcherManager>,
    label: String
) -> Result<(), ()> {
    let mut map_holder = patcher_manager.map.lock().await;
    map_holder.as_mut().unwrap().remove_mode(label);
    println!("Curr modes: {:?}", &map_holder.as_ref().unwrap().modes);
    Ok(())
}

#[tauri::command]
pub async fn add_final_battle_mode(
    patcher_manager: State<'_, PatcherManager>,
    label: String,
    timing: FinalBattleTime
) -> Result<(), ()> {
    let mut map_holder = patcher_manager.map.lock().await;
    map_holder.as_mut().unwrap().add_mode(label, TemplateModeType::FinalBattle(timing));
    println!("Curr modes: {:?}", &map_holder.as_ref().unwrap().modes);
    Ok(())
}

#[tauri::command]
pub async fn add_capture_object_mode(
    patcher_manager: State<'_, PatcherManager>,
    label: String,
    delay: u8
) -> Result<(), ()> {
    let mut map_holder = patcher_manager.map.lock().await;
    map_holder.as_mut().unwrap().add_mode(label, TemplateModeType::CaptureObject(delay));
    println!("Curr modes: {:?}", &map_holder.as_ref().unwrap().modes);
    Ok(())
}

#[tauri::command]
pub async fn add_economic_mode(
    patcher_manager: State<'_, PatcherManager>,
    label: String,
    resource_info: ResourceWinInfo
) -> Result<(), ()> {
    let mut map_holder = patcher_manager.map.lock().await;
    map_holder.as_mut().unwrap().add_mode(label, TemplateModeType::Economic(resource_info));
    println!("Curr modes: {:?}", &map_holder.as_ref().unwrap().modes);
    Ok(())
}

/// Invoked when user activates patch process.
/// Creates all necessary patches and runs it.
/// Repacks map after it.
#[tauri::command]
pub async fn patch_map(
    app: AppHandle, 
    patcher_manager: State<'_, PatcherManager>,
    path_manager: State<'_, PathManager>
) -> Result<(), ()> {
    // let map_locked = patcher_manager.map.lock().await;
    // let mut map = map_locked.as_ref().unwrap();

    // let config = patcher_manager.config_path.clone();

    // // ------ PATCHES -------
    // // base
    // let base_creator_path = config.join("adds\\common\\");
    // let base_creator = BaseCreator::new(
    //     &map.main_dir,
    //     &base_creator_path
    // );
    // // players
    // let teams = map.teams_info.clone();
    // let mut players_patcher = PlayersPatcher::new(teams.clone());
    // let mut teams_generator = TeamsGenerator::new(teams.clone());
    // // buildings
    // let building_creatable = CommonBuildingCreator::new(&patcher_manager.config_path);
    // let mut building_modifyable = BuildingModifyable::new(
    //     patcher_manager.config_path.join("banks_types.json"),
    //     patcher_manager.config_path.join("new_buildings_types.json"),
    //     &map.main_mode
    // );
    // // lights
    // let light_patcher = LightPatcher::new(patcher_manager.config_path.join("lights.json"), 
    // map.settings.use_night_lights);
    // // treasures
    // let mut treasure_patcher = TreasurePatcher::new(&patcher_manager.config_path);
    // // monsters
    // let mut monsters_patcher = CreatureModifier::new();
    // // towns
    // let mut town_patcher = TownPatcher::new(
    //     &patcher_manager.config_path, 
    //     &map.template,
    //     map.has_win_condition("capture")
    // );
    // // quests
    // let mut secondary_quest_patcher = QuestPatcher::new(patcher_manager.config_path.join("test_quest.xml"));
    // // win condition specific
    // let terrain_creator_path = config.join("adds\\terrains\\");
    // let underground_terrain_creator = UndergroundTerrainCreator::new(
    //     map.has_win_condition("final"),
    //     &terrain_creator_path,
    //     &map.main_dir,
    //     map.size
    // );
    // let mut win_condition_writer = WinConditionWriter {
    //     conditions: &map.conds,
    //     quest_path: &patcher_manager.config_path.join("win_condition_quests.xml"),
    //     write_dir: &map.main_dir,
    //     quest_info_path: &patcher_manager.config_path.join("adds\\win_conditions\\"),
    // };
    // let final_arena_creator = FinalBattleArenaCreator::new(
    //     &patcher_manager.config_path, 
    //     map.has_win_condition("final")
    // );
    
    // let map_xdb_patcher = Patcher::new()
    //     .with_root(&map.map_xdb).unwrap()
    //     .with_creatable("AmbientLight", &light_patcher, false)
    //     .with_creatable("GroundAmbientLights", &light_patcher, false)
    //     .with_creatable("MapScript", &base_creator, true)
    //     .with_creatable("CustomTeams", &base_creator, true)
    //     .with_creatable("RMGmap", &base_creator, true)
    //     .with_creatable("objects", &building_creatable, false)
    //     //.with_creatable("objects", &final_arena_creator, false)
    //     .with_creatable("HasUnderground", &underground_terrain_creator, true)
    //     .with_creatable("UndergroundTerrainFileName", &underground_terrain_creator, true)
    //     .with_modifyable("AdvMapTreasure", &mut treasure_patcher)
    //     .with_modifyable("AdvMapBuilding", &mut building_modifyable)
    //     .with_modifyable("AdvMapTown", &mut town_patcher)
    //     .with_modifyable("players", &mut players_patcher)
    //     .with_modifyable("Secondary", &mut secondary_quest_patcher)
    //     .with_modifyable("Primary", &mut win_condition_writer)
    //     .with_modifyable("AdvMapMonster", &mut monsters_patcher)
    //     .run();

    // let map_tag_patcher = Patcher::new()
    //     .with_root(&map.map_tag).unwrap()
    //     .with_creatable("HasUnderground", &underground_terrain_creator, true)
    //     .with_modifyable("teams", &mut teams_generator)
    //     .run();

    // // ------ FILE WRITERS ------
    // let file_writer = FileWriter::new()
    //     .with(&MoonCalendarWriter::new(
    //         map.settings.only_neutral_weeks,
    //         &map.game_mechanics_dir,
    //         &patcher_manager.config_path.join("adds\\moon_calendar\\Default.xdb")
    //     ))
    //     .with(&OutcastMechanicsWriter::new(
    //         &map.main_mode,
    //         &map.game_mechanics_dir,
    //         vec![
    //             (&patcher_manager.config_path.join("adds\\outcast\\Summon_Creatures.xdb"), &PathBuf::from("Spell\\Adventure_Spells\\Summon_Creatures.xdb")),
    //             (&patcher_manager.config_path.join("adds\\outcast\\Summon_Boat.xdb"), &PathBuf::from("Spell\\Adventure_Spells\\Summon_Boat.xdb"))
    //         ]
    //     ))
    //     .with(&OutcastTextWriter::new(
    //         &map.main_mode,
    //         &map.text_dir,
    //         &patcher_manager.config_path.join("adds\\outcast\\Long_Description.txt")
    //     ))
    //     .with(&base_creator)
    //     .with(&underground_terrain_creator)
    //     .with(&win_condition_writer)
    //     .run();

    // // ------- CODE GENERATORS -------
    // let template_info_generator = TemplateInfoGenerator{template: &map.main_mode};
    // let code_generator = CodeGenerator::new()
    //     .with(&building_modifyable)
    //     .with(&treasure_patcher)
    //     .with(&win_condition_writer)
    //     .with(&template_info_generator)
    //     .with(&monsters_patcher)
    //     .with(&town_patcher)
    //     .run(&map.main_dir);

    // // ------ TEXT PROCESSORS ------
    // let base_text_processor = TextProcessor::new(&map.map_name)
    //     .with(&MapNameChanger{})
    //     .run();
    // // win condition quests processing
    // let fbtp = TextProcessor::new(&map.main_dir.join("final_battle_desc.txt"))
    //     .with(&WinConditionFinalBattleFileProcessor {
    //         final_battle_time: map.conds.get("final")
    //     })
    //     .run();
    // let etp = TextProcessor::new(&map.main_dir.join("economic_desc.txt"))
    //     .with(&EconomicWinConditionTextProcessor {
    //         resource_info: map.conds.get("economic")
    //     })
    //     .run();
    // let cotp = TextProcessor::new(&map.main_dir.join("capture_object_desc.txt"))
    //     .with(&CaptureObjectWinConditionTextProcessor {
    //         delay_info: map.conds.get("capture"),
    //         town_name: &town_patcher.neutral_town_name,
    //     })
    //     .run();
    // zip_map(&map.name, &map.dir).await;
    // // move base map
    // let base_map_move_path = path_manager.maps().join("base_maps\\");
    // if base_map_move_path.exists() == false {
    //     std::fs::create_dir(&base_map_move_path);
    // }
    // std::fs::copy(
    //     &map.base_name,
    //     base_map_move_path.join(&map.base_name.file_name().unwrap().to_str().unwrap())
    // ).unwrap();
    // std::fs::remove_file(&map.base_name);
    Ok(())
}

use walkdir::WalkDir;

/// Creates patched map file from temp directory.
pub async fn zip_map(name: &String, dir: &PathBuf)  {
    let mut zip_file = std::fs::File::create(
        dir.parent().unwrap().join(name)
    ).unwrap();
    let mut map_zipped = zip::ZipWriter::new(zip_file);
    for entry in WalkDir::new(dir) {
        match entry {
            Ok(e) => {
                let path = e.path();
                // println!("path: {:?}", path);
                if path.is_file() {
                    let file_name = path.strip_prefix(dir).unwrap().to_str().unwrap();
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
    std::fs::remove_dir_all(dir);
}