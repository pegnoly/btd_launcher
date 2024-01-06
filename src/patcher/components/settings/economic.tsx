import { useState, useEffect, useRef } from "react";
import { Checkbox, Grid, Text } from "@mantine/core";
import { invoke } from "@tauri-apps/api";
import { PatchState, usePatchStateContext } from "../../contexts/patch_state";
import { useMapModesContext } from "../../contexts/map_mode";
import { MapMode } from "../map_mode";

export enum ResourceType {
    Gold = "Gold",
    RareResource = "RareResource"
}

export class EconomicVictoryProps {
    resType: ResourceType = ResourceType.Gold;
    goldCount: number = 200000;
    resCount: number = 50;
}

export function EconomicVictoryElement() {

    const patcherStateContext = usePatchStateContext();
    const mapModeContext = useMapModesContext();

    const [enabled, setEnabled] = useState<boolean>(false);
    const [economicProps, setEconomicProps] = useState<EconomicVictoryProps>(new EconomicVictoryProps());

    useEffect(() => {
        if (patcherStateContext?.state == PatchState.Inactive) {
            setEnabled(false);
            setEconomicProps(new EconomicVictoryProps());
        }
    }, [patcherStateContext?.state])

    useEffect(() => {
        if (enabled == false) {
            if (mapModeContext?.state.includes(MapMode.Economic)) {
                setEnabled(true);
                invoke("add_economic_mode", {label: "Economic", resourceInfo: {
                    type: economicProps.resType,
                    count: economicProps.resType == ResourceType.Gold ? economicProps.goldCount : economicProps.resCount
                }});
            }
        }
        else {
            if (mapModeContext?.state.includes(MapMode.Economic) == false) {
                setEnabled(false);
                invoke("remove_game_mode", {label: "Economic"});
            }
        }
    }, [mapModeContext?.state]);

    useEffect(() => {
        if (enabled == true) {
            invoke("add_economic_mode", {label: "Economic", resourceInfo: {
                type: economicProps.resType,
                count: economicProps.resType == ResourceType.Gold ? economicProps.goldCount : economicProps.resCount
            }});
        }
    }, [economicProps]);

    const resourcesInfo = {
        [ResourceType.Gold] : {values: [200000, 300000, 500000], update: ((count: number) => {
            setEconomicProps({
                ...economicProps,
                goldCount: count,
            });
        })},
        [ResourceType.RareResource] : {values: [50, 75, 100], update: ((count: number) => {
            setEconomicProps({
                ...economicProps,
                resCount: count,
            });
        })}
    }

    return (
        <div>
            <div hidden={!enabled}>
                <Text size="xs">Тип и число ресурсов для победы</Text>
                <Grid>
                    <Grid.Col span={5}>
                        <Text align="center" size={10} style={{position: "relative", left: 15}}>Тип ресурсов</Text>
                        <select style={{
                                width: 120, 
                                height: 20, 
                                fontSize: 12, 
                                position: "relative", 
                                left: 9
                            }} 
                            defaultValue={economicProps.resType}
                            onChange={
                                (e) => {
                                    let resType: ResourceType = ResourceType[e.target.value as keyof typeof ResourceType];
                                    let resAmount = resType == ResourceType.Gold ? economicProps.goldCount : economicProps.resCount
                                    setEconomicProps(prev => ({
                                        ...prev,
                                        resType: resType,
                                        goldCount: resType == ResourceType.Gold ? resAmount : prev.goldCount,
                                        resCount: resType == ResourceType.RareResource ? resAmount : prev.resCount
                                    }))
                                }
                            }>
                            <option value={ResourceType.Gold}>Золото</option>
                            <option value={ResourceType.RareResource}>Редкие ресурсы</option>
                        </select>
                    </Grid.Col>
                    <Grid.Col span={5}>
                        <Text align="center" size={10} style={{position: "relative", left: 15}}>Число ресурсов</Text>
                        <select 
                            style={{
                                width: 55, 
                                height: 20, 
                                fontSize: 12, 
                                position: "relative", 
                                left: 40
                            }}
                            value={economicProps.resType == ResourceType.Gold ? economicProps.goldCount : economicProps.resCount} 
                            onChange={
                                (e) => {
                                    let newAmount = parseInt(e.target.value);
                                    setEconomicProps(prev => ({
                                        ...prev,
                                        goldCount: prev.resType == ResourceType.Gold ? newAmount : prev.goldCount,
                                        resCount: prev.resType == ResourceType.RareResource ? newAmount : prev.resCount
                                    }))
                                }
                            }>
                            {resourcesInfo[economicProps.resType].values.map((value, index) => (
                                <option key={index} value={value}>{value.toString()}</option>
                            ))}
                        </select>
                    </Grid.Col>
                </Grid>
            </div>
        </div>
    )
}