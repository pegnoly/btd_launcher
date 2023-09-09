import {Box, MantineProvider, Button, Grid, Text, Radio, Group, Stack, Select, ScrollArea, Collapse, Title, Checkbox} from "@mantine/core";
import { event, invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { WebviewWindow } from "@tauri-apps/api/window";
import { ReactElement, useContext, useState } from "react";
import { createStyles } from "@mantine/core";
import {useDisclosure} from "@mantine/hooks";

import patcherBack from "./assets/patcher_back.png"
import patcherButtonActive from "./assets/patcher_button_active.png"
import patcherButtonPushed from "./assets/patcher_button_pushed.png"
import patcherButtonDisabled from "./assets/patcher_button_disabled.png"
import settingsBack from "./assets/settingsBack.png"
import actionsBack from "./assets/actions_panel_back.png"
import checkBoxBase from "./assets/check_box.png"
import checkBoxChecked from "./assets/check_box_checked.png"

import { PatcherVisibility } from "../App";
import { createContext } from "react";

const patcherStyles = createStyles((theme) => ({
  back: {
      width: 500,
      height: 410,
      backgroundImage: `url(${patcherBack})`,
      backgroundSize: 'hover',
      backgroundRepeat: 'no-repeat',
      backgroundColor: "transparent",
      overflow: "hidden"
  },
  map_info_div: {
    position: "absolute",
    top: 85,
    left: 45,
    fontWeight: "bolder",
    fontSize: 9,
    font: "icon",
    width: 400,
  },
  overlay: {
    position: "absolute",
    top: 100,
    left: 100,
    width: 400,
    height: 200
  },
  button : {
    width: 136,
    height: 49,
    backgroundImage: `url(${patcherButtonActive})`,
    backgroundSize: 'hover',
    backgroundColor: "transparent",
    border: 'none',
    //backgroundRepeat: "no-repeat",
    fontFamily: "Pacifico, cursive",
    fontSize: 13.5,
    color: "ActiveCaption",
    ":hover": {
      backgroundImage: `url(${patcherButtonPushed})`,
      backgroundSize: 'hover',
      backgroundColor: 'transparent',
      color: "ActiveBorder",
      border: "none"
    },
    ":active": {
      backgroundImage: `url(${patcherButtonPushed})`,
      backgroundSize: 'hover',
      backgroundColor: 'transparent',
      color: "blue",
      border: "none"
    },
    ":disabled": {
      backgroundImage: `url(${patcherButtonDisabled})`,
      backgroundSize: 'hover',
      backgroundColor: 'transparent',
      border: "none"
    }
  },
  select: {
    backgroundColor: "brown",
    borderRadius: 4,
    ":focus": {
      backgroundColor: "green"
    }
  },
  check_box : {
    backgroundImage: `url(${checkBoxBase})`,
    ":checked": {
      backgroundImage: `url(${checkBoxChecked})`
    }
  }
}));

async function tryClose(x: React.MouseEvent<HTMLButtonElement, MouseEvent>) {
  const currWindow = WebviewWindow.getByLabel("patcher");
  console.log("curr window: ", currWindow);
  currWindow?.hide()
}

enum TemplateType {
  Common,
  Outcast
}

interface Template {
  name: string,
  desc: string
}

interface MapInfo {
  players_count: number
  template: Template
}

interface MapName {
  name: string
}

function MapPickButtonClick(x: React.MouseEvent<HTMLButtonElement, MouseEvent>) {
  invoke("pick_map");
}

type PlayersInfo = {
  count: number
}

const playersInfoContext = createContext<[{}]>([{}]);

export default function Patcher(isVisible: PatcherVisibility) {

  const [mapPicked, setMapPicked] = useState<boolean>(false);

  // current map displayable variables
  const [currentMapName, setMapName] = useState<string>("test");
  const [currentTemplate, setTemplate] = useState<string>("");
  const [currentPlayersCount, setPlayersCount] = useState<number>(0);
  const [templateDesc, setTemplateDesc] = useState<string>("");

  // team selector props
  const [playersInfo, setPlayersInfo] = useState<string []>([]);
  const [teamsVariants, setTeamsVariants] = useState<string []>([]);

  // map settings check boxes
  const [nightLightsChecked, setNightLightsChecked] = useState<boolean>(false);
  const [weeksOnlyChecked, setWeeksOnlyChecked] = useState<boolean>(false);

  let mapPickedListener = listen("map_picked", (event) => {
    let name = (event.payload as MapName).name;
    setMapName(name);
    setMapPicked(true);
    unpackMap(name);
  })

  let mapUnpackedListener = listen("map_unpacked", (event) => {
    let map = event.payload as MapInfo;
    setTemplate(map.template.name);
    setPlayersCount(map.players_count);
    setTemplateDesc(map.template.desc);
    let playersData: string [] = [];
    let teamsData: string [] = [];
    for(let i = 1; i <= map.players_count; i++) {
      playersData[i] = i.toString();
      teamsData.push(i.toString());
    }
    setPlayersInfo(playersData);
    setTeamsVariants(teamsData);
  });
  
  async function patchButtonClick(x: React.MouseEvent<HTMLButtonElement, MouseEvent>) {
    await invoke("patch_map");
  }

  async function unpackMap(name: string) {
    await invoke("unpack_map", {mapPath: name});
  }

  const {classes} = patcherStyles();

  const [opened, handlers] = useDisclosure(false);
  const [settingsOpened, settingsHandlers] = useDisclosure(false);

  return (
    <MantineProvider theme={{fontFamily: "Geologica, sans-serif"}} withGlobalStyles withNormalizeCSS>
      <Box hidden={isVisible.visible} className={classes.back}>
          <Button 
            className={classes.button}
            name="mapPicker"
            style={{
              position: "absolute",
              top: 33,
              left: 175,
            }} 
            onClick={MapPickButtonClick}>Выберите карту</Button>
          <div style={{position: "relative", top: 175, left: 95, width: 300}}>
            <Text style={{fontFamily: 'Balsamiq Sans, cursive'}} align="center">Шаблон</Text>
            <Text style={{fontFamily: 'Balsamiq Sans, cursive'}} align="center" color="green">{currentTemplate}</Text>
            <Text style={{fontFamily: 'Balsamiq Sans, cursive'}} size={12} align="center" color="yellow">{templateDesc}</Text>
            <Button 
              name="patcherStartup"
              disabled={!mapPicked}
              className={classes.button}
              style={{
                position: "absolute",
                top: 140,
                left: 80
              }}
              onClick={patchButtonClick}>Обработать
            </Button>
          </div>
          <div className={classes.map_info_div}>
            <Grid>
              <Grid.Col span={5}>
                <div>
                  <Text style={{fontFamily: 'Balsamiq Sans, cursive'}} align="center">Имя карты</Text>
                  <Text style={{fontFamily: 'Balsamiq Sans, cursive'}} align="center" size={13} color="green">{currentMapName}</Text>
                  <Button 
                    name="settingsChecker"
                    disabled={!mapPicked}
                    className={classes.button}
                    style={{
                      position: "relative",
                      left: 15,
                    }}
                    onClick={settingsHandlers.toggle}>Настройки...
                  </Button>
                  <Collapse transitionDuration={1000} in={settingsOpened}>
                    <div
                      style={{
                        backgroundImage: `url(${settingsBack})`,
                        backgroundRepeat: "no-repeat",
                        backgroundSize: "contain",
                        width: 350,
                        height: 300,
                        position: "absolute",
                        left: 135,
                        top: -30,
                        zIndex: 99
                      }}>
                        <div style={{position: "relative", left: 35, top: 40}}>
                          <Checkbox size="xs" labelPosition="left" label="Использовать ночное освещение карты"
                            checked={nightLightsChecked}
                            onChange={(event) => {
                              setNightLightsChecked(event.currentTarget.checked);
                              invoke("set_night_lights_setting", {useNightLights: event.currentTarget.checked});
                            }}/>
                          <Checkbox size="xs" labelPosition="left" label="Отключить эффекты недель"
                            checked={weeksOnlyChecked}
                            onChange={(event) => {
                              setWeeksOnlyChecked(event.currentTarget.checked);
                              invoke("set_weeks_only_setting", {weeksOnly: event.currentTarget.checked});
                            }}/>
                        </div>
                    </div>
                  </Collapse>
                </div>
              </Grid.Col>
              <Grid.Col offset={2} span={5}>
                <div>
                  <Text style={{fontFamily: 'Balsamiq Sans, cursive'}} align="center">Число игроков</Text>
                  <Text style={{fontFamily: 'Balsamiq Sans, cursive'}} color="green" align="center" size={14}>{currentPlayersCount}</Text>
                  <Button 
                    name="teamSelector"
                    disabled={!mapPicked}
                    className={classes.button}
                    style={{
                      position: "relative",
                      left: 15,
                    }}
                    onClick={handlers.toggle}>Команды...</Button>
                  <Collapse transitionDuration={1000} in={opened}>
                    <div style={
                      { 
                        backgroundImage: `url(${actionsBack})`,
                        backgroundRepeat: "no-repeat",
                        backgroundSize: "contain",
                        position: "absolute",
                        top: -50,
                        left: 10,
                        height: 300,
                        width: 300,
                      }}> 
                    <Text style={{
                      position: "relative", top: 25, left: -15,
                      fontSize: 12
                    }} align="center">Назначьте команды игрокам</Text>
                    <ScrollArea style={{position: "relative", top: 30, left: 30}} w={200} h={200} type="always">
                          {playersInfo.map((team, player) => (
                            <div 
                              key={player}>
                              <Text style={{fontSize: 12, fontFamily: 'Balsamiq Sans, cursive'}} size="xs" align="center">Игрок {player}</Text>
                              <select 
                                className={classes.select}
                                style={{
                                  position: "relative", 
                                  left: 50, 
                                  width: 100, 
                                  height: 20
                                }} 
                                defaultValue={team}
                                onChange={e => {
                                  playersInfo[player] = e.target.value;
                                  invoke("update_player_team_info", {player: player, team: parseInt(e.target.value)})
                                }}>
                                {teamsVariants.map((value, _) => (
                                  <option>{value}</option>
                                ))}
                              </select>
                            </div>
                          ))}
                      </ScrollArea>
                    </div>
                  </Collapse>
                </div>
              </Grid.Col>
            </Grid>
          </div>
      </Box>
    </MantineProvider>
  );
}