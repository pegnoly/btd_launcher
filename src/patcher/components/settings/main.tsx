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

// actually means settings not dependent on any mode etc
class MapAdditionalSettings {
    useNightLights: boolean = false;
    disableWeeks: boolean = false;
    disableNeutralTownsDwells: boolean = false;
    enableNewArts: boolean = false;
}

export default function PatcherSettings(props: PatcherSettingsProps) {
    const {classes} = patcherStyles();

    const patcherStateContext = usePatchStateContext();

    const [visible, setVisible] = useState<boolean>(false);
    const [settings, setSettings] = useState<MapAdditionalSettings>(new MapAdditionalSettings());

    useEffect(() => {
        if (patcherStateContext?.state == PatchState.MapPicked) {
            setSettings(new MapAdditionalSettings());
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
                            checked={settings.useNightLights}
                            onChange={(event) => {
                                setSettings({...settings, useNightLights: event.currentTarget.checked});
                                invoke("set_night_lights_setting", {useNightLights: event.currentTarget.checked});
                        }}/>
                        <Checkbox size="xs" labelPosition="left" label="Отключить эффекты недель"
                            checked={settings.disableWeeks}
                            onChange={(event) => {
                                setSettings({...settings, disableWeeks: event.currentTarget.checked});
                                invoke("set_weeks_only_setting", {weeksOnly: event.currentTarget.checked});
                        }}/>
                        <Checkbox size="xs" labelPosition="left" label="Запретить постройку жилищ в нейтральных городах"
                            checked={settings.disableNeutralTownsDwells}
                            onChange={(event) => {
                                setSettings({...settings, disableNeutralTownsDwells: event.currentTarget.checked});
                                invoke("set_neutral_towns_dwells_setting", {isDisabled: event.currentTarget.checked});
                        }}/>
                        <Checkbox size="xs" labelPosition="left" label="Разрешить генерацию экспериментальных артефактов"
                            checked={settings.enableNewArts}
                            onChange={(event) => {
                                setSettings({...settings, enableNewArts: event.currentTarget.checked});
                                invoke("set_enable_new_arts_setting", {isEnabled: event.currentTarget.checked});
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