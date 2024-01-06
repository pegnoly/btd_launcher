import { Flex, createStyles } from "@mantine/core";
import { Box, Button, Text, Grid } from "@mantine/core";
import { invoke } from "@tauri-apps/api";
import { useEffect, useState } from "react";

import patcherBack from "../assets/patcher_back.png"
import patcherButtonActive from "../assets/patcher_button_active.png"
import patcherButtonPushed from "../assets/patcher_button_pushed.png"
import patcherButtonDisabled from "../assets/patcher_button_disabled.png"

import PatcherSettings from "./settings/main";
import TeamSelector from "./team_selector/main";
import { listen } from "@tauri-apps/api/event";
import { SingleValuePayload } from "../../App";
import { AppState, useAppStateContext } from "../../contexts/AppState";
import { PatchState, usePatchStateContext } from "../contexts/patch_state";
import MapModesProvider from "../contexts/map_mode";

import { MapMode, MapModeElement, MapModeInfo } from "./map_mode";

export const patcherStyles = createStyles((theme) => ({
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
      fontFamily: "Pacifico",
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
    button_text: {
        fontFamily: 'Balsamiq Sans, cursive'
    },
    select: {
      backgroundColor: "brown",
      borderRadius: 4,
      ":focus": {
        backgroundColor: "green"
      }
    },
    // check_box : {
    //   backgroundImage: `url(${checkBoxBase})`,
    //   ":checked": {
    //     backgroundImage: `url(${checkBoxChecked})`
    //   }
    // }
}));

type PatcherMainProps = {
    visible: boolean;
}

type Template = {
    name: string;
    desc: string;
    settings_desc: string;
    main_mode: MapMode | null,
    possible_modes: MapMode[]
}

type MapProps = {
    file_name: string;
    players_count: number;
    template: Template;
}

export default function PatcherMain(props: PatcherMainProps) {
    const {classes} = patcherStyles();

    const patchStateContext = usePatchStateContext();
    const appStateContext = useAppStateContext();

    const [currentMapName, setMapName] = useState<string>("");
    const [currentTemplate, setTemplate] = useState<string>("");
    const [currentPlayersCount, setPlayersCount] = useState<number>(0);
    const [possibleMapModes, setPossibleMapModes] = useState<MapMode[]>([]);
    const [mainMapMode, setMainMapMode] = useState<MapMode|null>(null);

    async function mapPickButtonClicked(event: React.MouseEvent<HTMLButtonElement, MouseEvent>) {
        //patchStateContext?.setState(PatchState.Inactive);
        await invoke("pick_map")
            .catch((error) => console.log("error occured while picking map: ", error));
    }

    const mapPickedListener = listen("map_picked", (event) => {
        let name = event.payload as SingleValuePayload<string>;
        patchStateContext?.setState(PatchState.MapPicked);
        startMapUnpack(name.value);
    });

    async function startMapUnpack(path: string) { 
        await invoke("unpack_map", {mapPath: path})
            .then((mapInfo) => {
                patchStateContext?.setState(PatchState.Active);
                setUnpackedMapProps(mapInfo as MapProps);
            })
            .catch((error) => console.log("error occured while unpacking map: ", error));
    }

    function setUnpackedMapProps(mapInfo: MapProps) {
        console.log("map props: ", mapInfo);
        setMapName(mapInfo.file_name);
        setTemplate(mapInfo.template.name);
        setPlayersCount(mapInfo.players_count);
        setPossibleMapModes(mapInfo.template.possible_modes);
        setMainMapMode(mapInfo.template.main_mode);
    }

    async function patchButtonClick() {
        patchStateContext?.setState(PatchState.Patching);
        appStateContext?.setState(AppState.Busy);
        invoke("patch_map").then(() => {
            patchStateContext?.setState(PatchState.Inactive);
            appStateContext?.setState(AppState.Patching);
        }).catch((error) => {
            patchStateContext?.setState(PatchState.Inactive);
            appStateContext?.setState(AppState.Patching);
        });
    }

    useEffect(() => {
        if (patchStateContext?.state == PatchState.Inactive) {
            setMapName("");
            setPlayersCount(0);
            setTemplate("");
            setPossibleMapModes([])
        }
    }, [patchStateContext?.state])

    return (
        <MapModesProvider>
            <Box 
                hidden={!props.visible} 
                style={{
                    width: 500,
                    height: 410,
                    backgroundImage: `url(${patcherBack})`,
                    backgroundSize: 'hover',
                    backgroundRepeat: 'no-repeat',
                    backgroundColor: "transparent",
                    overflow: "hidden",
                    position: "absolute",
                    right: 275,
                    bottom: -50
                }}>
                <Button 
                    disabled={(patchStateContext?.state == PatchState.Configuring || patchStateContext?.state == PatchState.Patching)}
                    className={classes.button}
                    name="mapPicker"
                    style={{
                        position: "absolute",
                        top: 33,
                        left: 175,
                    }} 
                    onClick={mapPickButtonClicked}>Выберите карту
                </Button>
                <div 
                    style={{
                        position: "relative", 
                        top: 175, 
                        left: 50, 
                        width: 400
                    }}>
                    <Text className={classes.button_text} align="center">Шаблон</Text>
                    <Text className={classes.button_text} align="center" color="green">{currentTemplate}</Text>
                    <Flex style={{position: "relative", top: 5}} justify="center" gap={10}>
                        {possibleMapModes.map((mode, index) => (
                            <MapModeElement 
                                key={index} 
                                name={MapModeInfo.get(mode)?.name}
                                desc={MapModeInfo.get(mode)?.desc}
                                mode={mode}
                                disableable={mainMapMode != null && mainMapMode != mode}
                                configurable={MapModeInfo.get(mode)?.configurable}
                            />
                        ))}
                    </Flex>
                    <Button 
                        name="patcherStartup"
                        disabled={!(patchStateContext?.state == PatchState.Active)}
                        className={classes.button}
                        style={{
                            position: "absolute",
                            top: 165,
                            left: 125
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
                            <PatcherSettings template={currentTemplate}/>
                        </div>
                    </Grid.Col>
                    <Grid.Col offset={2} span={5}>
                        <TeamSelector playersCount={currentPlayersCount}/>
                    </Grid.Col>
                </Grid>
                </div>
            </Box>
        </MapModesProvider>
    )
}