import { Text, Button, ScrollArea } from "@mantine/core";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api";
import { patcherStyles } from "../main";

import teamSelectorBack from "../../assets/actions_panel_back.png";
import { PatchState, usePatchStateContext } from "../../contexts/patch_state";

export type TeamSelectorProps = {
    playersCount: number;
}

function generateTeamsInfo(count: number) {
    let playersData: string [] = [];
    for(let i = 1; i <= count; i++) {
        playersData[i] = i.toString();
    }
    console.log("players data: ", playersData);
    return playersData;
}

export default function TeamSelector(props: TeamSelectorProps) {

    const patcherStateContext = usePatchStateContext();

    const [visible, setVisible] = useState<boolean>(false);
    const [playersInfo, setPlayersInfo] = useState<string []>(() => {return generateTeamsInfo(props.playersCount)});

    useEffect(() => {
        setPlayersInfo(generateTeamsInfo(props.playersCount));
    }, [props.playersCount])

    function selectorButtonClicked() {
        setVisible(!visible);
        if (patcherStateContext?.state == PatchState.Active) {
            patcherStateContext.setState(PatchState.Configuring);
        }
        else {
            patcherStateContext?.setState(PatchState.Active);
        }
    }

    const {classes} = patcherStyles();

    return (
        <div>
            <Text 
                style={{fontFamily: 'Balsamiq Sans, cursive'}} 
                align="center"
            >Число игроков</Text>
            <Text 
                hidden={patcherStateContext?.state == PatchState.Inactive} 
                style={{fontFamily: 'Balsamiq Sans, cursive'}} 
                color="green" 
                align="center" 
                size={14}
            >{props.playersCount}</Text>
            <Button 
                name="teamSelector"
                disabled={(patcherStateContext?.state == PatchState.Inactive || patcherStateContext?.state == PatchState.Patching)}
                className={classes.button}
                style={{
                    position: "relative",
                    left: 15,
            }}
            onClick={() => selectorButtonClicked()}>Команды...</Button>
            <div hidden={!visible}
                style={
                { 
                    backgroundImage: `url(${teamSelectorBack})`,
                    backgroundRepeat: "no-repeat",
                    backgroundSize: "contain",
                    position: "absolute",
                    top: -50,
                    left: 10,
                    height: 300,
                    width: 300
                }}> 
            <Text style={{
                fontFamily: "Balsamiq Sans, cursive",
                position: "relative", top: 25, left: -15,
                fontSize: 12
            }} align="center">Назначьте команды игрокам</Text>
            <ScrollArea 
                style={{position: "relative", top: 30, left: 30}} 
                w={200} h={200} type="always">
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
                        {playersInfo.map((_, value) => (
                            value != null && <option>{value}</option>
                        ))}
                        </select>
                    </div>
                    ))}
                </ScrollArea>
            </div>
        </div>
    )
}