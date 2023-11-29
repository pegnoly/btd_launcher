import { Checkbox, Text } from "@mantine/core";
import { invoke } from "@tauri-apps/api";
import { PatcherSettingsProps } from "./main";
import { useState, useEffect } from "react";
import { PatchState, usePatchStateContext } from "../../contexts/patch_state";

const captureTemplates: string[] = ["BTD-Universe", "BTD-UniverseX6", "BTD-KingsBounty", "BTD-JebusCross-Castle", "BTD-JebusCross-2x2-Castle"];

class CaptureProps {
    checked: boolean = false;
    delay: number = 3;
}

export default function CaptureElement(props: PatcherSettingsProps) {

    const patcherStateContext = usePatchStateContext();

    const [captureProps, setCaptureProps] = useState<CaptureProps>(new CaptureProps());

    useEffect(() => {
        if (patcherStateContext?.state == PatchState.MapPicked) {
            setCaptureProps(new CaptureProps());
        }
    }, [patcherStateContext?.state])

    return (
        <div>
            <div hidden={!captureTemplates.includes(props.template)}>
                <Checkbox size="xs" labelPosition="left" label = "Победа при удержании объекта"
                    checked={captureProps.checked}
                    onChange={(event) => {
                        let checked = event.currentTarget.checked;
                        setCaptureProps(prev => ({
                            ...prev,
                            checked: checked
                        }));
                        if (checked == true) {
                            invoke("update_capture_object_setting", {isEnabled: true, delay: captureProps.delay});
                        }
                        else {
                            invoke("update_capture_object_setting", {isEnabled: false});
                        }
                    }}/>
                <div hidden={!captureProps.checked} style={{
                        position: "relative",
                        left: -40
                    }}>
                    <Text align="center" size={10}>Число дней удержания до победы</Text>
                    <select value={captureProps.delay} style={{
                            width: 40, 
                            height: 20, 
                            fontSize: 12, 
                            position: "relative", 
                            left: 120
                        }}
                        onChange={
                            (e) => {
                                let newDelay = parseInt(e.currentTarget.value);
                                setCaptureProps(prev => ({
                                    ...prev,
                                    delay: newDelay
                                }));
                                invoke("update_capture_object_setting", {isEnabled: true, delay: newDelay});
                            }
                        }>
                        <option>3</option>
                        <option>7</option>
                        <option>10</option>
                        <option>14</option>
                        <option>21</option>
                    </select>
                </div>
            </div>
        </div>
    )
}