import { Checkbox, ScrollArea, Stack, Button, MantineProvider } from "@mantine/core";
import { event, invoke } from "@tauri-apps/api";
import { useEffect, useState } from "react";
import { FinalBattleElement } from "./final_battle";
import { EconomicVictoryElement } from "./economic";
import  CaptureElement  from "./capture";
import { patcherStyles } from "../main";

import settingsBack from "../../assets/settingsBack.png";
import { PatchState, usePatchStateContext } from "../../contexts/patch_state";

export type PatcherSettingsProps = {
    template: string;
}

export default function PatcherSettings(props: PatcherSettingsProps) {
    const {classes} = patcherStyles();

    const patcherStateContext = usePatchStateContext();

    const [visible, setVisible] = useState<boolean>(false);
    const [nightLightsChecked, setNightLightsChecked] = useState<boolean>(false);
    const [weeksOnlyChecked, setWeeksOnlyChecked] = useState<boolean>(false);

    useEffect(() => {
        if (patcherStateContext?.state == PatchState.MapPicked) {
            setNightLightsChecked(false);
            setWeeksOnlyChecked(false);
        }
    }, [patcherStateContext?.state])

    function settingsButtonClicked() {
        setVisible(!visible);
        if (patcherStateContext?.state == PatchState.Active) {
            patcherStateContext.setState(PatchState.Configuring);
        }
        else {
            patcherStateContext?.setState(PatchState.Active);
        }
    }

    return (
        <MantineProvider theme={{
            fontFamily: 'Balsamiq Sans, cursive'
        }}>
        <Button 
            name="settingsChecker"
            disabled={(patcherStateContext?.state == PatchState.Inactive || patcherStateContext?.state == PatchState.Patching)}
            className={classes.button}
            style={{
                position: "relative",
                left: 15,
            }}
            onClick={() => settingsButtonClicked()}>Настройки...
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
                        <FinalBattleElement/>
                        <EconomicVictoryElement/>
                        <CaptureElement template={props.template}/>
                    </Stack>
                </ScrollArea>
            </div>
        </div>
    </MantineProvider>
    )
}