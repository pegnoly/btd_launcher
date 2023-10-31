import { useState, useEffect, useRef } from "react";
import { Checkbox, Grid, Text } from "@mantine/core";
import { invoke } from "@tauri-apps/api";

class FinalBattlTiming {
    month: number = 1;
    week: number = 1;
    day: number = 1;
}

class FinalBattleInfo {
    checked: boolean = false
    timing: FinalBattlTiming = new FinalBattlTiming();
}

let finalBattleInfo = new FinalBattleInfo();

export function FinalBattleElement() {
    const finalBattleRef = useRef<FinalBattleInfo>(finalBattleInfo);

    const [finalBattleChecked, setFinalBattleChecked] = useState<boolean>(finalBattleRef.current.checked);
    const [currentMonth, setCurrentMonth] = useState<number>(finalBattleRef.current.timing.month);
    const [currentWeek, setCurrentWeek] = useState<number>(finalBattleRef.current.timing.week);
    const [currentDay, setCurrentDay] = useState<number>(finalBattleRef.current.timing.day);

    useEffect(() => {
        finalBattleRef.current.checked = finalBattleChecked;
        finalBattleRef.current.timing.month = currentMonth;
        finalBattleRef.current.timing.week = currentWeek;
        finalBattleRef.current.timing.day = currentDay;
    }, [finalBattleChecked, currentMonth, currentWeek, currentDay]);

    const monthNumber: number = 12;

    function generateMonths() {
        let options = [];
        for (let index = 1; index <= monthNumber; index++) {
            options.push(<option key={index}>{index.toString()}</option>)
        }    
        return options;
    }

    return (
        <div>
            <Checkbox size="xs" labelPosition="left" label = "Назначить дату финальной битвы"
            checked={finalBattleChecked}
            onChange={(event) => {
                let checked = event.currentTarget.checked;
                setFinalBattleChecked(event.currentTarget.checked);
                if (checked == true) {
                    invoke("update_final_battle_setting", {
                        isEnabled: true, 
                        finalBattleTime: {
                            month: currentMonth, 
                            week: currentWeek,
                            day: currentDay
                    }});
                }
                else {
                    invoke("update_final_battle_setting", {
                        isEnabled: false
                    });
                }
        }}/>
        <div hidden={!finalBattleChecked}>
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
                        defaultValue={currentMonth}
                        onChange={(event) => {
                            setCurrentMonth(parseInt(event.currentTarget.value));
                            invoke("update_final_battle_setting", {
                                isEnabled: true, 
                                finalBattleTime: {
                                    month: parseInt(event.currentTarget.value), 
                                    week: currentWeek,
                                    day: currentDay
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
                        defaultValue={currentWeek}
                        onChange={(event) => {
                            setCurrentWeek(parseInt(event.currentTarget.value));
                            invoke("update_final_battle_setting", {
                                isEnabled: true, 
                                finalBattleTime: {
                                    month: currentMonth, 
                                    week: parseInt(event.currentTarget.value),
                                    day: currentDay
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
                    defaultValue={currentDay}
                    onChange={(event) => {
                        setCurrentDay(parseInt(event.currentTarget.value));
                        invoke("update_final_battle_setting", {
                            isEnabled: true, 
                            finalBattleTime: {
                                month: currentMonth, 
                                week: currentWeek,
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
    </div>
    )
}