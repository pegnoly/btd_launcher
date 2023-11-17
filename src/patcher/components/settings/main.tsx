import { Checkbox, ScrollArea, Stack, Collapse, Button } from "@mantine/core";
import { event, invoke } from "@tauri-apps/api";
import { useState } from "react";
import { PatchState } from "../main";
import { FinalBattleElement } from "./final_battle";
import { EconomicVictoryElement } from "./economic";
import  CaptureElement  from "./capture";
import { useDisclosure } from "@mantine/hooks";
import { patcherStyles } from "../main";

import settingsBack from "../../assets/settingsBack.png";

export type PatcherSettingsProps = {
    state: PatchState;
    template: string;
}

export default function PatcherSettings(props: PatcherSettingsProps) {
    const {classes} = patcherStyles();

    const [nightLightsChecked, setNightLightsChecked] = useState<boolean>(false);
    const [weeksOnlyChecked, setWeeksOnlyChecked] = useState<boolean>(false);

    const [settingsOpened, settingsHandlers] = useDisclosure(false);

    return (
        <>
        <Button 
            name="settingsChecker"
            disabled={!(props.state == PatchState.MapPicked)}
            className={classes.button}
            style={{
                position: "relative",
                left: 15,
            }}
            onClick={settingsHandlers.toggle}>Настройки...
        </Button>
        <Collapse transitionDuration={1000} in={settingsOpened}>
            <div
                style={{
                    backgroundImage: `url(${settingsBack})`,
                    backgroundRepeat: "no-repeat",
                    backgroundSize: "contain",
                    width: 350,
                    height: 300,
                    position: "absolute",
                    left: 135,
                    top: -30,
                    zIndex: 99
                }}>
                <div style={{position: "relative", left: 35, top: 40}}>
                    <ScrollArea w={300} h={150} type="hover">
                        <Stack spacing={5} w={290}>
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
                            <FinalBattleElement/>
                            <EconomicVictoryElement/>
                            <CaptureElement template={props.template} state={props.state}/>
                        </Stack>
                    </ScrollArea>
                </div>
            </div>
        </Collapse>
    </>
    )
}