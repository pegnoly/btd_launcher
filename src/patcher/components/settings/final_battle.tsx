import { useState, useEffect, useRef } from "react";
import { Grid, Text } from "@mantine/core";
import { invoke } from "@tauri-apps/api";
import { PatchState, usePatchStateContext } from "../../contexts/patch_state";
import { useMapModesContext } from "../../contexts/map_mode";
import { MapMode } from "../map_mode";

export class FinalBattleTiming {
    month: number = 1;
    week: number = 1;
    day: number = 1;
}

const MONTH_COUNT: number = 12;

function generateMonths() {
    let options = [];
    for (let index = 1; index <= MONTH_COUNT; index++) {
        options.push(<option key={index}>{index.toString()}</option>)
    }    
    return options;
}

export function FinalBattleElement() {

    const patcherStateContext = usePatchStateContext();
    const mapModeContext = useMapModesContext();

    const [enabled, setEnabled] = useState<boolean>(false);
    const [timing, setTiming] = useState<FinalBattleTiming>(new FinalBattleTiming());

    useEffect(() => {
        if (patcherStateContext?.state == PatchState.MapPicked) {
            setEnabled(false)
            setTiming(new FinalBattleTiming());
        }
    }, [patcherStateContext?.state]);

    useEffect(() => {
        console.log("map mode context changed: ", mapModeContext?.state)
        if (enabled == false) {
            if (mapModeContext?.state.includes(MapMode.FinalBatte)) {
                setEnabled(true);
                invoke("add_final_battle_mode", {label: "FinalBattle", timing: timing});
            }
        }
        else {
            if (mapModeContext?.state.includes(MapMode.FinalBatte) == false) {
                setEnabled(false);
                invoke("remove_game_mode", {label: "FinalBattle"});
            }
        }
    }, [mapModeContext?.state]);

    useEffect(() => {
        if (enabled == true) {
            invoke("add_final_battle_mode", {label: "FinalBattle", timing: timing});
        }
    }, [timing]);

    return (
        <>
        <div hidden={!enabled}>
            <Text size="xs">Дата финальной битвы</Text>
            <Grid>
                <Grid.Col span={1} offset={1}>
                    <Text align = "center" size={10}>Месяц</Text>
                    <select style={{
                            width: 40,
                            height: 20, 
                            fontSize: 12, 
                            position: "relative",
                            left: -3
                        }}
                        value={timing.month}
                        onChange={(event) => {
                            setTiming({
                                ...timing,
                                month: parseInt(event.currentTarget.value),
                            })
                        }}>
                        {generateMonths()}
                    </select>
                </Grid.Col>
                <Grid.Col span={1} offset={2}>
                    <Text align = "center" size={10}>Неделя</Text>
                    <select style={{
                            width: 40,
                            height: 20, 
                            fontSize: 12
                        }}
                        value={timing.week}
                        onChange={(event) => {
                            setTiming({
                                ...timing,
                                week: parseInt(event.currentTarget.value),
                            })
                        }}>
                        <option>1</option>
                        <option>2</option>
                        <option>3</option>
                        <option>4</option>
                    </select>
                </Grid.Col>
                <Grid.Col span={1} offset={2}>
                    <Text size={10}>День</Text>
                    <select style={{
                        width: 40,
                        height: 20, 
                        fontSize: 12, 
                        position: "relative",
                        left: -6  
                    }}
                    value={timing.day}
                    onChange={(event) => {
                        setTiming({
                            ...timing,
                            day: parseInt(event.currentTarget.value),
                        })
                    }}>
                        <option>1</option>
                        <option>2</option>
                        <option>3</option>
                        <option>4</option>
                        <option>5</option>
                        <option>6</option>
                        <option>7</option>
                    </select>
                </Grid.Col>
            </Grid>
        </div>
    </>
    )
}