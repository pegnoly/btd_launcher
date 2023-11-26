import { Checkbox, ScrollArea, Stack, Button, MantineProvider } from "@mantine/core";
import { event, invoke } from "@tauri-apps/api";
import { useEffect, useState } from "react";
import { PatchState } from "../main";
import { FinalBattleElement } from "./final_battle";
import { EconomicVictoryElement } from "./economic";
import  CaptureElement  from "./capture";
import { patcherStyles } from "../main";

import settingsBack from "../../assets/settingsBack.png";

export type PatcherSettingsProps = {
    state: PatchState;
    template: string;
}

export default function PatcherSettings(props: PatcherSettingsProps) {
    const {classes} = patcherStyles();

    const [visible, setVisible] = useState<boolean>(false);
    const [nightLightsChecked, setNightLightsChecked] = useState<boolean>(false);
    const [weeksOnlyChecked, setWeeksOnlyChecked] = useState<boolean>(false);

    useEffect(() => {
        if (props.state == PatchState.Inactive) {
            setNightLightsChecked(false);
            setWeeksOnlyChecked(false);
        }
    }, [props.state])

    return (
        <MantineProvider theme={{
            fontFamily: 'Balsamiq Sans, cursive'
        }}>
        <Button 
            name="settingsChecker"
            disabled={!(props.state == PatchState.MapPicked)}
            className={classes.button}
            style={{
                position: "relative",
                left: 15,
            }}
            onClick={() => setVisible(!visible)}>Настройки...
        </Button>
        <div
            hidden={!visible}
            style={{
                backgroundImage: `url(${settingsBack})`,
                backgroundRepeat: "no-repeat",
                backgroundSize: "contain",
                width: 350,
                height: 300,
                position: "absolute",
                left: 105,
                top: -30,
                zIndex: 99
            }}>
            <div style={{position: "relative", left: 45, top: 40}}>
                <ScrollArea w={290} h={150} type="hover">
                    <Stack spacing={5} w={280}>
                        <Checkbox size="xs" labelPosition="left" label="Использовать ночное освещение карты"
                            checked={nightLightsChecked}
                            onChange={(event) => {
                                setNightLightsChecked(event.currentTarget.checked);
                                invoke("set_night_lights_setting", {useNightLights: event.currentTarget.checked});
                        }}/>
                        <Checkbox size="xs" labelPosition="left" label="Отключить эффекты недель"
                            checked={weeksOnlyChecked}
                            onChange={(event) => {
                                setWeeksOnlyChecked(event.currentTarget.checked);
                                invoke("set_weeks_only_setting", {weeksOnly: event.currentTarget.checked});
                        }}/>
                        <FinalBattleElement template={props.template} state={props.state}/>
                        <EconomicVictoryElement template={props.template} state={props.state}/>
                        <CaptureElement template={props.template} state={props.state}/>
                    </Stack>
                </ScrollArea>
            </div>
        </div>
    </MantineProvider>
    )
}