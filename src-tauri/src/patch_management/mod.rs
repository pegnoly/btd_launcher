use homm5_types::town;
use tauri::{Manager, State, AppHandle, api::dialog::FileDialogBuilder, App};
use patcher::{Patcher,
    map::{Unpacker, Map, template::{Template, TemplateTransferable, TemplatesInfoModel, TemplateModeType, TemplateModeName}}, 
    patch_strategy::{
        PatchGroup,
        base::{MapScriptCreator, CustomTeamsCreator, RMGmapRemover, MapNameChanger}, 
        building::{BuildingInfoProvider, BuildingPatchesGroup, modifiers::{BuildingNameApplier, OutcastTavernReplacer}, getters::BuildingTypeDetector}, 
        treasure::{TreasureInfoProvider, TreasurePatchesGroup, modifiers::TreasureNameApplier, getters::TreasurePropsDetector}, 
        player::{PlayersInfoProvider, PlayersCrossPatchInfo, PlayerPatchesGroup, modifiers::{PlayerTeamSelector, OutcastPlayerHeroSelector, InactivePlayersTavernFilterRemover}}, 
        light::{LightsInfoProvider, AmbientLightCreator, GroundAmbientLightsCreator}, 
        quest::{QuestInfoProvider, QuestPatchesGroup, modifiers::{MapInitQuestCreator, MapModesQuestCreator, QuestEmptyItemsFixer}},
        town::{TownInfoProvider, TownPatchesGroup, 
            modifiers::{TownNameApplier, DefaultTownSchemesApplier}, 
            getters::{TownActiveTilesDetector, PlayerRaceDetector, CapturableTownDetector}, 
            PlayerRaceCrossPatchInfo, NeutralTownCrossPatchInfo
        }, 
        modes::{
            final_battle::{FinalBattleTime, FinalBattleModeTextProcessor}, 
            economic::{ResourceWinInfo, EconomicModeTextProcessor}, 
            capture::CaptureObjectModeTextProcessor,
            outcast::{OutcastMechanicsWriter, OutcastTextWriter, AvailableHeroesWriter}, ModesInfoGenerator
        }, 
        creature::{CreaturePatchesGroup, modifiers::{CreatureNameApplier, AdditionalStackFixer}},
        terrain::{UndergroundTerrainCreator, UndergroundEnabler, UndergroundTerrainNameApplier},
        objects::CommonObjectsCreator, mechanics::{MoonCalendarWriter, NewArtifactsEnabler}
    }, CodeGenerator, FileWriter, TextProcessor
};
use serde::{Serialize, Deserialize};
use tokio::{sync::Mutex, io::AsyncWriteExt};
use zip::write::FileOptions;
use std::{path::PathBuf, collections::HashMap, f64::consts::E, io::Read, cell::{RefCell, RefMut}, sync::RwLock};
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
    label: TemplateModeName,
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
    label: TemplateModeName
) -> Result<(), ()> {
    let mut map_holder = patcher_manager.map.lock().await;
    map_holder.as_mut().unwrap().remove_mode(label);
    println!("Curr modes: {:?}", &map_holder.as_ref().unwrap().modes);
    Ok(())
}

#[tauri::command]
pub async fn add_final_battle_mode(
    patcher_manager: State<'_, PatcherManager>,
    label: TemplateModeName,
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
    label: TemplateModeName,
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
    label: TemplateModeName,
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
    let map_locked = patcher_manager.map.lock().await;
    let mut map = map_locked.as_ref().unwrap();

    let config = patcher_manager.config_path.clone();
    let config_common_dir = config.join("adds\\common\\");
    let map_modes:Vec<_> = map.modes.clone().into_keys().collect();
    // Town patches group
    let town_info_provider = TownInfoProvider::new(&config);
    let mut player_race_cross_patch_info = RwLock::new(PlayerRaceCrossPatchInfo::new());
    let mut neutral_town_cross_patch_info = NeutralTownCrossPatchInfo{neutral_town_name: None};
    let mut town_name_applier = TownNameApplier::new(map.settings.disable_neutral_towns_dwells);
    let mut default_town_scheme_applier = DefaultTownSchemesApplier::new(&town_info_provider, &map_modes);
    let mut town_active_tile_detector = TownActiveTilesDetector::new(&config, &town_info_provider);
    let mut player_race_detector = PlayerRaceDetector::new(&player_race_cross_patch_info, &town_info_provider);
    let mut capturable_town_detector = CapturableTownDetector::new(
        &town_info_provider, 
        &mut neutral_town_cross_patch_info, 
        map.modes.contains_key(&TemplateModeName::CaptureObject)
    );
    let mut town_patch_group = TownPatchesGroup::new()
        .with_modifyable(&mut town_name_applier)
        .with_modifyable(&mut default_town_scheme_applier)
        .with_getter(&mut town_active_tile_detector)
        .with_getter(&mut player_race_detector)
        .with_getter(&mut capturable_town_detector);
    // Player patches group
    let mut player_info_provider = PlayersInfoProvider::new(&config);
    let mut player_cross_patch_info = RwLock::new(PlayersCrossPatchInfo::new());
    let mut player_team_selector = PlayerTeamSelector::new(&map.teams_info);
    let mut outcast_player_hero_selector = OutcastPlayerHeroSelector::new(
        &mut player_info_provider, 
        &player_race_cross_patch_info, 
        &player_cross_patch_info, 
        map.modes.contains_key(&TemplateModeName::Outcast)
    );
    let mut inactive_player_tavern_filter_remover = InactivePlayersTavernFilterRemover{};
    let mut player_patch_group = PlayerPatchesGroup::new()
        .with_modifyable(&mut player_team_selector)
        .with_modifyable(&mut outcast_player_hero_selector)
        .with_modifyable(&mut inactive_player_tavern_filter_remover);
    // Treasure patches group
    let treasure_info_provider = TreasureInfoProvider::new(&config);
    let mut treasure_name_applier = TreasureNameApplier::new();
    let mut treasure_props_detector = TreasurePropsDetector::new(&treasure_info_provider);
    let mut treasure_patch_group = TreasurePatchesGroup::new()
        .with_modifyable(&mut treasure_name_applier)
        .with_getter(&mut treasure_props_detector);
    // Building patches group
    let building_info_provider = BuildingInfoProvider::new(&config);
    let mut building_name_applier = BuildingNameApplier::new();
    let mut outcast_tavern_replacer = OutcastTavernReplacer::new(map.modes.contains_key(&TemplateModeName::Outcast));
    let mut building_type_detector = BuildingTypeDetector::new(&building_info_provider);
    let mut building_patch_group = BuildingPatchesGroup::new()
        .with_modifyable(&mut building_name_applier)
        .with_modifyable(&mut outcast_tavern_replacer)
        .with_getter(&mut building_type_detector);
    // Creature patches group
    let mut creature_name_applier = CreatureNameApplier::new();
    let mut additional_stack_fixer = AdditionalStackFixer{};
    let mut creature_patch_group = CreaturePatchesGroup::new()
        .with_modifyable(&mut creature_name_applier)
        .with_modifyable(&mut additional_stack_fixer);
    // Quest patches group
    let quest_info_provider = QuestInfoProvider::new(&config);
    let mut map_init_quest_creator = MapInitQuestCreator::new(&quest_info_provider);
    let mut map_modes_quest_creator = MapModesQuestCreator::new(&quest_info_provider);
    let mut empty_items_fixer = QuestEmptyItemsFixer{};
    let mut quest_patch_group = QuestPatchesGroup::new()
        .with_modifyable(&mut map_init_quest_creator)
        .with_modifyable(&mut map_modes_quest_creator)
        .with_modifyable(&mut empty_items_fixer);
    // Lights patches
    let light_info_provider = LightsInfoProvider::new(&config, map.settings.use_night_lights);
    let ambient_light_creator = AmbientLightCreator::new(&light_info_provider);
    let ground_ambient_lights_creator = GroundAmbientLightsCreator::new(&light_info_provider);
    //
    let map_script_creator = MapScriptCreator::new(
        &config_common_dir,
        &map.main_dir
    );
    //
    let final_battle_active = map.modes.contains_key(&TemplateModeName::FinalBattle);
    let common_objects_creator = CommonObjectsCreator::new(&config, final_battle_active);
    let underground_enabler = UndergroundEnabler::new(final_battle_active);
    let underground_name_applier = UndergroundTerrainNameApplier::new(final_battle_active);
    let available_heroes_writer = AvailableHeroesWriter::new(
        map.modes.contains_key(&TemplateModeName::Outcast), 
        &player_cross_patch_info
    );
    let map_xdb_patcher = Patcher::new()
        .with_root(&map.map_xdb).unwrap()
        .with_modifyables("AdvMapTown", &mut town_patch_group)
        .with_modifyables("players", &mut player_patch_group)
        .with_modifyables("AdvMapTreasure", &mut treasure_patch_group)
        .with_modifyables("AdvMapBuilding", &mut building_patch_group)
        .with_modifyables("AdvMapMonster", &mut creature_patch_group)
        .with_modifyables("Objectives", &mut quest_patch_group)
        .with_creatable("AmbientLight", &ambient_light_creator, true)
        .with_creatable("GroundAmbientLights", &ground_ambient_lights_creator, true)
        .with_creatable("MapScript", &map_script_creator, true)
        .with_creatable("CustomTeams", &CustomTeamsCreator{}, true)
        .with_creatable("RMGmap", &RMGmapRemover{}, true)
        .with_creatable("objects", &common_objects_creator, false)
        .with_creatable("HasUnderground", &underground_enabler, true)
        .with_creatable("UndergroundTerrainFileName", &underground_name_applier, true)
        .with_creatable("AvailableHeroes", &available_heroes_writer, true)
        .run();

    // File writers.
    let terrain_creator_path = config.join("adds\\terrains\\");
    let underground_terrain_creator = UndergroundTerrainCreator::new(
        final_battle_active,
        &terrain_creator_path,
        &map.main_dir,
        map.size
    );
    let modes_path = config.join("adds\\win_conditions\\");
    let map_modes_info_generator = ModesInfoGenerator::new(
        &map.modes, 
        &modes_path, 
        &map.main_dir
    );
    let file_writer = FileWriter::new()
        .with(&MoonCalendarWriter::new(
            map.settings.only_neutral_weeks,
            &map.game_mechanics_dir,
            &patcher_manager.config_path.join("adds\\moon_calendar\\Default.xdb")
        ))
        .with(&NewArtifactsEnabler::new(
            map.settings.enable_new_arts, 
            &map.game_mechanics_dir, 
            &config_common_dir.join("Artifacts.xdb")))
        .with(&OutcastMechanicsWriter::new(
            map.modes.contains_key(&TemplateModeName::Outcast),
            &map.game_mechanics_dir,
            vec![
                (&patcher_manager.config_path.join("adds\\outcast\\Summon_Creatures.xdb"), &PathBuf::from("Spell\\Adventure_Spells\\Summon_Creatures.xdb")),
                (&patcher_manager.config_path.join("adds\\outcast\\Summon_Boat.xdb"), &PathBuf::from("Spell\\Adventure_Spells\\Summon_Boat.xdb"))
            ]
        ))
        .with(&OutcastTextWriter::new(
            map.modes.contains_key(&TemplateModeName::Outcast),
            &map.text_dir,
            &patcher_manager.config_path.join("adds\\outcast\\Long_Description.txt")
        ))
        .with(&underground_terrain_creator)
        .with(&map_modes_info_generator)
        .run();
    // // ------- CODE GENERATORS -------
    let code_generator = CodeGenerator::new()
        .with(&building_patch_group)
        .with(&treasure_patch_group)
        .with(&map_modes_info_generator)
        .with(&creature_patch_group)
        .with(&town_patch_group)
        .run(&map.main_dir);

    // // ------ TEXT PROCESSORS ------
    let base_text_processor = TextProcessor::new(&map.map_name)
        .with(&MapNameChanger{})
        .run();
    // win condition quests processing
    let fbtp = TextProcessor::new(&map.main_dir.join("final_battle_desc.txt"))
        .with(&FinalBattleModeTextProcessor {
            final_battle_time: map.get_mode(&TemplateModeName::FinalBattle)
        })
        .run();
    let etp = TextProcessor::new(&map.main_dir.join("economic_desc.txt"))
        .with(&EconomicModeTextProcessor {
            resource_info: map.get_mode(&TemplateModeName::Economic)
        })
        .run();
    let cotp = TextProcessor::new(&map.main_dir.join("capture_object_desc.txt"))
        .with(&CaptureObjectModeTextProcessor::new(
            map.get_mode(&TemplateModeName::CaptureObject),
            &neutral_town_cross_patch_info
        ))
        .run();
    zip_map(&map.name, &map.dir);
    // move base map
    let base_map_move_path = path_manager.maps().join("base_maps\\");
    if base_map_move_path.exists() == false {
        std::fs::create_dir(&base_map_move_path);
    }
    std::fs::copy(
        &map.base_name,
        base_map_move_path.join(&map.base_name.file_name().unwrap().to_str().unwrap())
    ).unwrap();
    std::fs::remove_file(&map.base_name);
    Ok(())
}

use walkdir::WalkDir;

/// Creates patched map file from temp directory.
pub fn zip_map(name: &String, dir: &PathBuf)  {
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