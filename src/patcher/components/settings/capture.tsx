import { Checkbox, Text } from "@mantine/core";
import { invoke } from "@tauri-apps/api";
import { PatcherSettingsProps } from "./main";
import { useState, useEffect } from "react";
import { PatchState, usePatchStateContext } from "../../contexts/patch_state";
import { useMapModesContext } from "../../contexts/map_mode";
import { MapMode } from "../map_mode";

export default function CaptureElement(props: PatcherSettingsProps) {

    const patcherStateContext = usePatchStateContext();
    const mapModeContext = useMapModesContext();

    const [enabled, setEnabled] = useState<boolean>();
    const [delay, setDelay] = useState<number>(3);

    useEffect(() => {
        if (patcherStateContext?.state == PatchState.MapPicked) {
            setDelay(3);
        }
    }, [patcherStateContext?.state])

    useEffect(() => {
        if (enabled == false) {
            if (mapModeContext?.state.includes(MapMode.CaptureObject)) {
                setEnabled(true);
                invoke("add_capture_object_mode", {label: "capture_object", delay: delay});
            }
        }
        else {
            if (mapModeContext?.state.includes(MapMode.CaptureObject) == false) {
                setEnabled(false);
                invoke("remove_game_mode", {label: "capture_object"});
            }
        }
    }, [mapModeContext?.state]);

    useEffect(() => {
        if (enabled == true) {
            invoke("add_capture_object_mode", {label: "capture_object", delay: delay});
        }
    }, [delay]);

    return (
        <div>
            <div hidden={!enabled}>
                <Text size="xs">Число дней удержания замка до победы</Text>
                <select value={delay} style={{
                        width: 40, 
                        height: 20, 
                        fontSize: 12, 
                        position: "relative", 
                        left: 120
                    }}
                    onChange={
                        (e) => {
                            let newDelay = parseInt(e.currentTarget.value);
                            setDelay(newDelay);
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
    )
}