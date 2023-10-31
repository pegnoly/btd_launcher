import { useState, useEffect, useRef } from "react";
import { Checkbox, Grid, Text } from "@mantine/core";
import { invoke } from "@tauri-apps/api";

enum ResourceType {
    Gold = "Gold",
    RareResource = "RareResource"
}

class EconomicVictoryProps {
    checked: boolean = false;
    resType: ResourceType = ResourceType.Gold;
    goldCount: number = 200000;
    resCount: number = 50;
}

let economicVictoryProps = new EconomicVictoryProps();

export function EconomicVictoryElement() {
    const economicVictoryRef = useRef(economicVictoryProps);

    const [economicVictoryChecked, setEconomicVictoryChecked] = useState<boolean>(economicVictoryRef.current.checked);
    const [currentResType, setResType] = useState<ResourceType>(economicVictoryRef.current.resType);
    const [currentGoldCount, setGoldCount] = useState<number>(economicVictoryRef.current.goldCount);
    const [currentResCount, setResCount] = useState<number>(economicVictoryRef.current.resCount);

    const resourcesInfo = {
        [ResourceType.Gold] : {values: [200000, 300000, 500000], update: ((count: number) => {
            setGoldCount(count);
            invoke("update_economic_victory_setting", {
                isEnabled: true, 
                resourceInfo: {
                    _type: ResourceType.Gold, 
                    count: count
                }
            });
        })},
        [ResourceType.RareResource] : {values: [50, 75, 100], update: ((count: number) => {
            setResCount(count);
            invoke("update_economic_victory_setting", {
                isEnabled: true, 
                resourceInfo: {
                    _type: ResourceType.RareResource, 
                    count: count
                }
            });
        })}
    }

    useEffect(() => {
        economicVictoryRef.current.checked = economicVictoryChecked;
        economicVictoryRef.current.resType = currentResType;
        economicVictoryRef.current.resCount = currentResCount;
        economicVictoryRef.current.goldCount = currentGoldCount;
    }, [economicVictoryChecked, currentResType, currentResCount, currentGoldCount])

    return (
        <div>
            <Checkbox size="xs" labelPosition="left" label = "Победа по числу ресурсов"
                checked={economicVictoryChecked}
                onChange={(event) => {
                    let checked = event.currentTarget.checked;
                    setEconomicVictoryChecked(checked);
                    if (checked == true) {
                        resourcesInfo[currentResType].update(currentResType == ResourceType.Gold ? currentGoldCount : currentResCount)
                    }
                    else {
                        invoke("update_economic_victory_setting", {isEnabled: false})
                    }
                }}/>
            <div hidden={!economicVictoryChecked}>
                <Grid>
                    <Grid.Col span={5}>
                        <Text align="center" size={10}>Тип ресурсов</Text>
                        <select style={{
                                width: 120, 
                                height: 20, 
                                fontSize: 12, 
                                position: "relative", 
                                left: 9
                            }} 
                            defaultValue={currentResType}
                            onChange={
                                (e) => {
                                    let resType: ResourceType = ResourceType[e.target.value as keyof typeof ResourceType];
                                    setResType(resType);
                                    resourcesInfo[resType].update(resType == ResourceType.Gold ? currentGoldCount: currentResCount);
                                }
                            }>
                            <option value={ResourceType.Gold}>Золото</option>
                            <option value={ResourceType.RareResource}>Редкие ресурсы</option>
                        </select>
                    </Grid.Col>
                    <Grid.Col span={5}>
                        <Text align="center" size={10}>Число ресурсов</Text>
                        <select 
                            style={{
                                width: 55, 
                                height: 20, 
                                fontSize: 12, 
                                position: "relative", 
                                left: 40
                            }}
                            value={currentResType == ResourceType.Gold ? currentGoldCount : currentResCount} 
                            onChange={
                                (e) => resourcesInfo[currentResType].update(parseInt(e.target.value))
                            }>
                            {resourcesInfo[currentResType].values.map((value, index) => (
                                <option key={index} value={value}>{value.toString()}</option>
                            ))}
                        </select>
                    </Grid.Col>
                </Grid>
            </div>
        </div>
    )
}