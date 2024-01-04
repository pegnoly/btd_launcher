import { useState } from "react";
import { HoverCard, Text } from "@mantine/core";

import { useMapModesContext } from "../contexts/map_mode";
import { invoke } from "@tauri-apps/api";

export enum MapMode {
    Blitz = "Blitz",
    Economic = "Economic",
    FinalBatte = "FinalBattle",
    Outcast = "Outcast",
    CaptureObject = "CaptureObject"
}

type MapModeProps = {
    name: string | undefined,
    //url: string | undefined,
    desc: string | undefined,
    configurable: boolean | undefined
}

type MapModeType = {
    mode: MapMode,
    disableable: boolean,
}

type MapModeElementProps = MapModeProps & MapModeType;

export function MapModeElement(props: MapModeElementProps) {
    const [selected, setSelected] = useState<boolean>(false);
    const mapModeContext = useMapModesContext();
    return (
        <>
        <HoverCard width={240} shadow="md" offset={1}>
            <HoverCard.Target>
                <button style={{
                    width: 100,
                    height: 50,
                    borderColor: (props.disableable == false) ? "yellow" : (selected ? "green" : "red"),
                    borderRadius: 0,
                    borderWidth: 3,
                    fontSize: 13,
                    fontFamily: "Pacifico",
                    wordWrap: "break-word",
                    padding: 0,
                    //backgroundImage: `url(${type.url})`,
                    backgroundSize: "contain",
                    backgroundColor: "transparent"
                }}
                onClick={() => {
                    if (props.disableable == true) {
                        let new_selected = !selected;
                        if (new_selected == true) {
                            mapModeContext?.setState([...mapModeContext?.state, props.mode]);
                            // if mode must not be configurable just enable it here
                            if (props.configurable == false) {
                                invoke("add_game_mode", {label: props.mode.toString(), mode: props.mode})
                            }
                        }
                        else {
                            mapModeContext?.setState(mapModeContext?.state.filter(m => m != props.mode));
                            // if mode must not be configurable just enable it here
                            if (props.configurable == false) {
                                invoke("remove_game_mode", {label: props.mode.toString()})
                            }
                        }
                        setSelected(new_selected);
                    }
                }}
                >{props.name}</button>
            </HoverCard.Target>
            <HoverCard.Dropdown>
                <Text size={10.5} align="center">{props.desc}</Text>
                <Text size={10} style={{color: "silver"}} align="center">{(props.disableable == false ? "[Встроенный режим для шаблона]" : "")}</Text>
            </HoverCard.Dropdown>
        </HoverCard>
        </>
    )
}

export const MapModeInfo = new Map<MapMode, MapModeProps>([
    [MapMode.Blitz, {
        //url: sd, 
        name: "Blitz-режим", 
        desc: "Активирует режим ускоренной постройки города и прироста армии",
        configurable: false
    }],
    [MapMode.Economic, {
        //url: sd, 
        name: "Экономическая победа", 
        desc: "Активирует условие победы при наборе определенного числа золота или редких ресурсов(используйте соотв. настройку)",
        configurable: true
    }],
    [MapMode.FinalBatte, {
        //url: sd, 
        name: "Финальная битва", 
        desc: "Позволяет установить дату принудительной финальной битвы(используйте соотв. настройку)",
        configurable: true
    }],
    [MapMode.Outcast, {
        //url: sd, 
        name: "Outcast-режим", 
        desc: "Активирует режим игры только одним героем",
        configurable: false
    }],
    [MapMode.CaptureObject, {
        //url: sd, 
        name: "Захват замка", 
        desc: "Активирует условие победы при захвате и удержании нейтрального замка(используйте соотв. настройку)",
        configurable: true
    }]
])