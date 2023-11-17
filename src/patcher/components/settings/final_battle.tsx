import { useState, useEffect, useRef } from "react";
import { Checkbox, Grid, Text } from "@mantine/core";
import { invoke } from "@tauri-apps/api";

class FinalBattleTiming {
    month: number = 1;
    week: number = 1;
    day: number = 1;
}

const monthNumber: number = 12;

function generateMonths() {
    let options = [];
    for (let index = 1; index <= monthNumber; index++) {
        options.push(<option key={index}>{index.toString()}</option>)
    }    
    return options;
}

export function FinalBattleElement() {
    const [checked, setChecked] = useState<boolean>(() => {
        let json = localStorage.getItem("patcher_final_battle_checked");
        if (json != null) {
            return JSON.parse(json) as boolean
        }
        else {
            return false;
        }
    });
    const [timing, setTiming] = useState<FinalBattleTiming>(() => {
        let json = localStorage.getItem("patcher_final_battle_timing");
        if (json != null) {
            return JSON.parse(json) as FinalBattleTiming
        }
        else {
            return new FinalBattleTiming();
        }
    });

    useEffect(() => {
        localStorage.setItem("patcher_final_battle_checked", JSON.stringify(checked));
    }, [checked]);

    useEffect(() => {
        localStorage.setItem("patcher_final_battle_timing", JSON.stringify(timing));
    }, [timing]);

    return (
        <>
            <Checkbox size="xs" labelPosition="left" label = "Назначить дату финальной битвы"
            checked={checked}
            onChange={(event) => {
                let checked = event.currentTarget.checked;
                setChecked(checked)
                if (checked == true) {
                    invoke("update_final_battle_setting", {
                        isEnabled: true, 
                        finalBattleTime: {
                            month: timing.month, 
                            week: timing.week,
                            day: timing.day
                    }});
                }
                else {
                    invoke("update_final_battle_setting", {
                        isEnabled: false
                    });
                }
        }}/>
        <div hidden={!checked}>
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
                        defaultValue={timing.month}
                        onChange={(event) => {
                            setTiming({
                                ...timing,
                                month: parseInt(event.currentTarget.value),
                            })
                            invoke("update_final_battle_setting", {
                                isEnabled: true, 
                                finalBattleTime: {
                                    month: parseInt(event.currentTarget.value), 
                                    week: timing.week,
                                    day: timing.day
                                }})
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
                        defaultValue={timing.week}
                        onChange={(event) => {
                            setTiming({
                                ...timing,
                                week: parseInt(event.currentTarget.value),
                            })
                            invoke("update_final_battle_setting", {
                                isEnabled: true, 
                                finalBattleTime: {
                                    month: timing.month, 
                                    week: parseInt(event.currentTarget.value),
                                    day: timing.day
                                }})
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
                    defaultValue={timing.day}
                    onChange={(event) => {
                        setTiming({
                            ...timing,
                            day: parseInt(event.currentTarget.value),
                        })
                        invoke("update_final_battle_setting", {
                            isEnabled: true, 
                            finalBattleTime: {
                                month: timing.month, 
                                week: timing.week,
                                day: parseInt(event.currentTarget.value)
                            }})
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