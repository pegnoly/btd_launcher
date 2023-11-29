import { useState, useEffect, useRef } from "react";
import { Checkbox, Grid, Text } from "@mantine/core";
import { invoke } from "@tauri-apps/api";
import { PatchState, usePatchStateContext } from "../../contexts/patch_state";

enum ResourceType {
    Gold = "Gold",
    RareResource = "RareResource"
}

class EconomicVictoryProps {
    resType: ResourceType = ResourceType.Gold;
    goldCount: number = 200000;
    resCount: number = 50;
}

export function EconomicVictoryElement() {

    const patcherStateContext = usePatchStateContext();

    const [checked, setChecked] = useState<boolean>(false);
    const [economicProps, setEconomicProps] = useState<EconomicVictoryProps>(new EconomicVictoryProps());

    useEffect(() => {
        if (patcherStateContext?.state == PatchState.Inactive) {
            setChecked(false);
            setEconomicProps(new EconomicVictoryProps());
        }
    }, [patcherStateContext?.state])

    function sendResInfoToBackend(type: ResourceType, count: number) {
        invoke("update_economic_victory_setting", {isEnabled: true, resourceInfo: {_type: type, count: count}});
    }

    const resourcesInfo = {
        [ResourceType.Gold] : {values: [200000, 300000, 500000], update: ((count: number) => {
            setEconomicProps({
                ...economicProps,
                goldCount: count,
            });
            sendResInfoToBackend(ResourceType.Gold, count);
        })},
        [ResourceType.RareResource] : {values: [50, 75, 100], update: ((count: number) => {
            setEconomicProps({
                ...economicProps,
                resCount: count,
            });
            sendResInfoToBackend(ResourceType.RareResource, count);
        })}
    }

    return (
        <div>
            <Checkbox size="xs" labelPosition="left" label = "Победа по числу ресурсов"
                checked={checked}
                onChange={(event) => {
                    let checked = event.currentTarget.checked;
                    setChecked(checked);
                    if (checked == true) {
                        resourcesInfo[economicProps.resType].update(economicProps.resType == ResourceType.Gold ? economicProps.goldCount : economicProps.resCount)
                    }
                    else {
                        invoke("update_economic_victory_setting", {isEnabled: false})
                    }
                }}/>
            <div hidden={!checked}>
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
                                    resourcesInfo[resType].update(resType == ResourceType.Gold ? economicProps.goldCount : economicProps.resCount);
                                    setEconomicProps(prev => ({
                                        ...prev,
                                        resType: resType
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
                                (e) => resourcesInfo[economicProps.resType].update(parseInt(e.target.value))
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