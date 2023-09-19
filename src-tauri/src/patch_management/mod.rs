use tauri::{Manager, State, AppHandle, api::dialog::FileDialogBuilder, App};
use patcher::{Patcher, TemplateTransferable, TemplatesInfoModel,
    map::{Unpacker, Map, Template}, 
    patch_strategy::{
        base::BaseCreator, 
        building::{BuildingModifyable, BuildingCreatable}, 
        treasure::TreasurePatcher, 
        player::{PlayersPatcher, TeamsGenerator}, light::LightPatcher, quest::QuestPatcher,
        town::TownPatcher, misc::{MoonCalendarWriter, OutcastFilesWriter, MapNameChanger}
    }, CodeGenerator, FileWriter, TextProcessor
};
use serde::{Serialize, Deserialize};
use tokio::{sync::Mutex, io::AsyncWriteExt};
use zip::write::FileOptions;
use std::{path::PathBuf, collections::HashMap, f64::consts::E, io::Read, cell::{RefCell, RefMut}};
use std::ops::Range;
use std::io::Write;

use crate::file_management::PathManager;

// frontend communication structs.
#[derive(Serialize, Clone)]
pub struct MapDisplayableInfo {
    //pub file_name: String,
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
        app.app_handle().emit_to("main", "patcher_visibility_changed", PatcherVisibility {visible: false}).unwrap();
    }
    Ok(())
}

#[tauri::command]
pub fn pick_map(app: AppHandle, path_manager: State<PathManager>) {
    let file_dialog = FileDialogBuilder::new()
        .add_filter("hommV maps", &["h5m"])
        .set_directory(path_manager.maps());
    file_dialog.pick_file(move |file_path| {
        match file_path {
            Some(file) => {
                app.app_handle().emit_to("main", "map_picked", MapDisplayableName {name : file.to_str().unwrap().to_string()});
            }
            None => {}
        }
    });
}

#[tauri::command]
pub async fn unpack_map(
    app: AppHandle, 
    patcher_manager: State<'_, PatcherManager>, 
    map_path: String
) -> Result<(), ()> {
    let mut map = Unpacker::unpack_map(&PathBuf::from(map_path));
    map.init_write_dirs();
    let mut map_holder = patcher_manager.map.lock().await;
    let mut templates_holder = patcher_manager.templates_model.lock().await;
    *map_holder = Some(map);
    let template = map_holder.as_mut().unwrap().detect_template(&templates_holder).unwrap();
    let players_count = map_holder.as_ref().unwrap().detect_teams_count();
    for i in 1..players_count.unwrap() + 1 {
        map_holder.as_mut().unwrap().teams_info[i] = i;
    }
    app.app_handle().emit_to("main", "map_unpacked",
        MapDisplayableInfo {
            players_count: players_count.unwrap() as u8,
            template: template
        });
    Ok(())
}

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

#[derive(serde::Deserialize)]
pub struct MapPackable {
    pub name: String,
    pub dir: PathBuf
}

#[tauri::command]
pub async fn patch_map(
    app: AppHandle, 
    patcher_manager: State<'_, PatcherManager>, 
) -> Result<(), ()> {
    let mut map_holder = patcher_manager.map.lock().await;
    let base_creator = BaseCreator::new(
        map_holder.as_ref().unwrap().get_write_dir(String::from("main")),
        patcher_manager.config_path.join("patcher\\adds\\")
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
        map_holder.as_ref().unwrap().template()
    );
    let p = Patcher::new()
        .with_root(map_holder.as_ref().unwrap().map_xdb()).unwrap()
        .with_creatable("AmbientLight", &light_patcher, false)
        .with_creatable("GroundAmbientLights", &light_patcher, false)
        .with_creatable("MapScript", &base_creator, true)
        .with_creatable("CustomTeams", &base_creator, true)
        .with_creatable("RMGmap", &base_creator, true)
        .with_creatable("objects", &BuildingCreatable::new(patcher_manager.config_path.join("patcher\\")), false)
        .with_modifyable("AdvMapTreasure", &mut treasure_patcher)
        .with_modifyable("AdvMapBuilding", &mut building_modifyable)
        .with_modifyable("AdvMapTown", &mut town_patcher)
        .with_modifyable("players", &mut players_patcher)
        .with_modifyable("Secondary", &mut QuestPatcher::new(patcher_manager.config_path.join("patcher\\test_quest.xml")))
        .run();
    let g = CodeGenerator::new()
        .with(&building_modifyable)
        .with(&treasure_patcher)
        .run(&map_holder.as_ref().unwrap().map_xdb().parent().unwrap().to_path_buf());
    let f = FileWriter::new()
        .with(&MoonCalendarWriter::new(
            map_holder.as_ref().unwrap().settings.only_neutral_weeks,
            map_holder.as_ref().unwrap().get_write_dir(String::from("game_mechanics")),
            patcher_manager.config_path.join("patcher\\adds\\Default.xdb")
        ))
        .with(&OutcastFilesWriter::new(
            map_holder.as_ref().unwrap().template(),
            &map_holder.as_ref().unwrap().get_write_dir(String::from("game_mechanics")),
            &patcher_manager.config_path.join("patcher\\adds\\Summon_Creatures.xdb")
        ))
        .with(&base_creator)
        .run();
    let pp = Patcher::new()
        .with_root(map_holder.as_ref().unwrap().map_tag()).unwrap()
        .with_modifyable("teams", &mut TeamsGenerator::new(teams.clone()))
        .run();
    let t = TextProcessor::new(map_holder.as_ref().unwrap().map_name())
        .with(&MapNameChanger{})
        .run();
    let m = MapPackable {
        name: map_holder.as_ref().unwrap().name.clone(),
        dir: map_holder.as_ref().unwrap().dir.clone()
    };
    zip_map(m);
    Ok(())
}

use walkdir::WalkDir;

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