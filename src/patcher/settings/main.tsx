import { Checkbox, Grid, ScrollArea, Select, Stack, Text } from "@mantine/core";
import { event, invoke } from "@tauri-apps/api";
import { useState, useRef, useEffect } from "react";
import { TemplateInfo } from "../patcher";
import { FinalBattleElement } from "./final_battle";
import { EconomicVictoryElement } from "./economic";
import { CaptureElement } from "./capture";

class PatcherSettingsProps {
    nightLightOptionChecked: boolean = false;
    weeksOnlyOptionChecked: boolean = false;
}

let pp = new PatcherSettingsProps();

export default function PatcherSettings() {
    const patcherSettingsProps = useRef<PatcherSettingsProps>(pp);
    const [nightLightsChecked, setNightLightsChecked] = useState<boolean>(patcherSettingsProps.current.nightLightOptionChecked);
    const [weeksOnlyChecked, setWeeksOnlyChecked] = useState<boolean>(patcherSettingsProps.current.weeksOnlyOptionChecked);

    useEffect(() => {
        patcherSettingsProps.current.nightLightOptionChecked = nightLightsChecked
        patcherSettingsProps.current.weeksOnlyOptionChecked = weeksOnlyChecked
    }, [nightLightsChecked, weeksOnlyChecked])

    return (
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
                    <CaptureElement/>
                </Stack>
            </ScrollArea>
        </div>
    )
}