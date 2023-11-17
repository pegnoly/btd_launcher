import { createStyles } from "@mantine/core";
import { Box, Button, Text, Grid } from "@mantine/core";
import { event, invoke } from "@tauri-apps/api";
import { useState } from "react";
import {useDisclosure} from "@mantine/hooks";

import patcherBack from "../assets/patcher_back.png"
import patcherButtonActive from "../assets/patcher_button_active.png"
import patcherButtonPushed from "../assets/patcher_button_pushed.png"
import patcherButtonDisabled from "../assets/patcher_button_disabled.png"
import PatcherSettings from "./settings/main";
import TeamSelector from "./team_selector/main";
import { listen } from "@tauri-apps/api/event";
import { SingleValuePayload } from "../../App";

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
      //backgroundRepeat: "no-repeat",
      //fontFamily: "Pacifico, cursive",
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
        //fontFamily: 'Balsamiq Sans, cursive'
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

export enum PatchState {
    Inactive,
    MapPicked,
    Patching
}

type PatcherMainProps = {
    visible: boolean;
}

type Template = {
    name: string;
    desc: string;
    settings_desc: string;
}

type MapProps = {
    players_count: number;
    template: Template;
}

export default function PatcherMain(props: PatcherMainProps) {
    const {classes} = patcherStyles();

    const [currentState, setCurrentState] = useState<PatchState>(PatchState.Inactive);
    const [currentMapName, setMapName] = useState<string>("");
    const [currentTemplate, setTemplate] = useState<string>("");
    const [currentPlayersCount, setPlayersCount] = useState<number>(0);
    const [templateDesc, setTemplateDesc] = useState<string>("");
    const [templateSettingsDesc, setTemplateSettingsDesc] = useState<string>("");

    async function mapPickButtonClicked(event: React.MouseEvent<HTMLButtonElement, MouseEvent>) {
        await invoke("pick_map")
            .catch((error) => console.log("error occured while picking map: ", error));
    }

    const mapPickedListener = listen("map_picked", (event) => {
        let name = event.payload as SingleValuePayload<string>;
        startMapUnpack(name.value);
    });

    async function startMapUnpack(path: string) { 
        setMapName(path);
        await invoke("unpack_map", {mapPath: path})
            .then((mapInfo) => {
                setCurrentState(PatchState.MapPicked);
                setUnpackedMapProps(mapInfo as MapProps);
            })
            .catch((error) => console.log("error occured while unpacking map: ", error));
    }

    function setUnpackedMapProps(mapInfo: MapProps) {
        setTemplate(mapInfo.template.name);
        setPlayersCount(mapInfo.players_count);
        setTemplateDesc(mapInfo.template.desc);
        setTemplateSettingsDesc(mapInfo.template.settings_desc);
    }

    async function patchButtonClick() {
        invoke("patch_map").then(); // msg that everything ok here and state changes here
    }

    return (
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
                <Text className={classes.button_text} size={12} align="center" color="yellow">{templateDesc}</Text>
                <Text className={classes.button_text} style = {{position: "relative", top: 10}} size={12} align="center" color="yellow">{templateSettingsDesc}</Text>
                <Button 
                    name="patcherStartup"
                    disabled={!(currentState == PatchState.MapPicked)}
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
                        <PatcherSettings state={currentState} template={currentTemplate}/>
                    </div>
                </Grid.Col>
                <Grid.Col offset={2} span={5}>
                    <TeamSelector playersCount={currentPlayersCount} patchState={currentState}/>
                </Grid.Col>
            </Grid>
            </div>
        </Box>
    )
}