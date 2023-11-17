import { Text, Collapse, Button, ScrollArea } from "@mantine/core";
import { useState } from "react";
import { PatchState } from "../main"
import { useDisclosure } from "@mantine/hooks";
import { invoke } from "@tauri-apps/api";
import { patcherStyles } from "../main";

import teamSelectorBack from "../../assets/actions_panel_back.png";

export type TeamSelectorProps = {
    playersCount: number;
    patchState: PatchState;
}

function generateTeamsInfo(count: number) {
    console.log("Here, ", count)
    let playersData: string [] = [];
    //let teamsData: string [] = [];
    for(let i = 1; i <= count; i++) {
        playersData[i] = i.toString();
      //teamsData.push(i.toString());
    }
    return playersData;
}

export default function TeamSelector(props: TeamSelectorProps) {
    const [playersInfo, setPlayersInfo] = useState<string []>(generateTeamsInfo(props.playersCount));
    
    const [opened, handlers] = useDisclosure(false);

    const {classes} = patcherStyles();

    return (
        <div>
            <Text style={{fontFamily: 'Balsamiq Sans, cursive'}} align="center">Число игроков</Text>
            <Text style={{fontFamily: 'Balsamiq Sans, cursive'}} color="green" align="center" size={14}>{props.playersCount}</Text>
            <Button 
                name="teamSelector"
                disabled={!(props.patchState == PatchState.MapPicked)}
                className={classes.button}
                style={{
                    position: "relative",
                    left: 15,
            }}
            onClick={handlers.toggle}>Команды...</Button>
            <Collapse transitionDuration={1000} in={opened}>
            <div style={
                { 
                    backgroundImage: `url(${teamSelectorBack})`,
                    backgroundRepeat: "no-repeat",
                    backgroundSize: "contain",
                    position: "absolute",
                    top: -50,
                    left: 10,
                    height: 300,
                    width: 300,
                }}> 
            <Text style={{
                position: "relative", top: 25, left: -15,
                fontSize: 12
            }} align="center">Назначьте команды игрокам</Text>
            <ScrollArea style={{position: "relative", top: 30, left: 30}} w={200} h={200} type="always">
                    {playersInfo.map((team, player) => (
                    <div 
                        key={player}>
                        <Text style={{fontSize: 12, fontFamily: 'Balsamiq Sans, cursive'}} size="xs" align="center">Игрок {player}</Text>
                        <select 
                        className={classes.select}
                        style={{
                            position: "relative", 
                            left: 50, 
                            width: 100, 
                            height: 20
                        }} 
                        defaultValue={team}
                        onChange={e => {
                            playersInfo[player] = e.target.value;
                            invoke("update_player_team_info", {player: player, team: parseInt(e.target.value)})
                        }}>
                        {playersInfo.map((value, _) => (
                            value != null && <option>{value}</option>
                        ))}
                        </select>
                    </div>
                    ))}
                </ScrollArea>
            </div>
            </Collapse>
        </div>
    )
}