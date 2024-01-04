import { useState } from "react";
import { HoverCard, Text } from "@mantine/core";

import { useMapModesContext } from "../contexts/map_mode";

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
    desc: string | undefined
}

type MapModeType = {
    mode: MapMode
}

type MapModeElementProps = MapModeProps & MapModeType;

export function MapModeElement(props: MapModeElementProps) {
    const [selected, setSelected] = useState<boolean>(false);
    const mapModeContext = useMapModesContext();
    return (
        <>
        <HoverCard width={200} shadow="md">
            <HoverCard.Target>
                <button style={{
                    width: 100,
                    height: 50,
                    borderColor: !selected ? "red" : "green",
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
                    let new_selected = !selected;
                    if (new_selected == true) {
                        mapModeContext?.setState([...mapModeContext?.state, props.mode]);
                    }
                    else {
                        mapModeContext?.setState(mapModeContext?.state.filter(m => m != props.mode));
                    }
                    setSelected(new_selected);
                }}
                >{props.name}</button>
            </HoverCard.Target>
            <HoverCard.Dropdown>
                <Text size={11} align="center">{props.desc}</Text>
            </HoverCard.Dropdown>
        </HoverCard>
        </>
    )
}

export const MapModeInfo = new Map<MapMode, MapModeProps>([
    [MapMode.Blitz, {
        //url: sd, 
        name: "Blitz-режим", 
        desc: "Активирует режим ускоренной постройки города и прироста армии"
    }],
    [MapMode.Economic, {
        //url: sd, 
        name: "Экономическая победа", 
        desc: "Активирует условие победы при наборе определенного числа золота или редких ресурсов(используйте соотв. настройку)"
    }],
    [MapMode.FinalBatte, {
        //url: sd, 
        name: "Финальная битва", 
        desc: "Позволяет установить дату принудительной финальной битвы(используйте соотв. настройку)"
    }],
    [MapMode.Outcast, {
        //url: sd, 
        name: "Outcast-режим", 
        desc: "Активирует режим игры только одним героем"
    }],
    [MapMode.CaptureObject, {
        //url: sd, 
        name: "Захват замка", 
        desc: "Активирует условие победы при захвате и удержании нейтрального замка(используйте соотв. настройку)"
    }]
])