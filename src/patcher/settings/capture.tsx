import { useState, useEffect, useRef, useContext } from "react";
import { Checkbox, Grid, Text } from "@mantine/core";
import { invoke } from "@tauri-apps/api";
import { templateContext } from "../patcher";

const captureTemplates: string[] = ["BTD-Universe", "BTD-UniverseX6", "BTD-KingsBounty"];

class CaptureProps {
    checked: boolean = false;
    delay: number = 3;
}

let captureProps = new CaptureProps();

export function CaptureElement() {
    const template = useContext(templateContext);
    const captureRef = useRef(captureProps);
    const [captureVictoryChecked, setcaptureVictoryChecked] = useState<boolean>(captureRef.current.checked);
    const [currentCaptureDelay, setCaptureDelay] = useState<number>(captureRef.current.delay);

    useEffect(() => {
        captureRef.current.checked = captureVictoryChecked;
        captureRef.current.delay = currentCaptureDelay;
    }, [captureVictoryChecked, currentCaptureDelay]);

    return (
        <div>
            <div hidden={!captureTemplates.includes(template)}>
                <Checkbox size="xs" labelPosition="left" label = "Победа при удержании объекта"
                    checked={captureVictoryChecked}
                    onChange={(event) => {
                        let checked = event.currentTarget.checked;
                        setcaptureVictoryChecked(checked);
                        if (checked == true) {
                            invoke("update_capture_object_setting", {isEnabled: true, delay: currentCaptureDelay});
                        }
                        else {
                            invoke("update_capture_object_setting", {isEnabled: false});
                        }
                    }}/>
                <div hidden={!captureVictoryChecked} style={{
                        position: "relative",
                        left: -40
                    }}>
                    <Text align="center" size={10}>Число дней удержания до победы</Text>
                    <select value={currentCaptureDelay} style={{
                            width: 40, 
                            height: 20, 
                            fontSize: 12, 
                            position: "relative", 
                            left: 150
                        }}
                        onChange={
                            (e) => {
                                let newDelay = parseInt(e.currentTarget.value);
                                setCaptureDelay(newDelay);
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