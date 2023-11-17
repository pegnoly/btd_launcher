import { Box, Text, Progress, Button, MantineProvider } from "@mantine/core";
import updaterBack from "../updater/assets/updater_back.png";
import { useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api";
import { SingleValuePayload } from "../App";
import { actionsStyles } from "../Actions";
import { useAppStateContext, AppState } from "../contexts/AppState";

export enum UpdateState {
    NotReady,
    Ready,
    InProcess
}

export default function Updater() {
    const [updateState, setUpdateState] = useState<UpdateState>(UpdateState.NotReady);
    const [updaterWindowDisabled, setUpdaterWindowDisabled] = useState<boolean>(true);
    const [currentlyUpdatedFile, changeUpdatedFile] = useState<string>("");
    const [currentDownloadProgress, changeDownloadProgress] = useState<number>(0);

    const appStateContext = useAppStateContext();

    const updatedFileChangedListener = listen("updated_file_changed", (event) => {
        let file = event.payload as SingleValuePayload<string>;
        changeUpdatedFile(file.value);
    })
    
    const downloadProgressChanged = listen("download_progress_changed", (event) => {
        let percent = event.payload as SingleValuePayload<number>;
        changeDownloadProgress(percent.value * 100);
    })

    const updateStateChanged = listen("download_state_changed", (event) => {
        let disabled = event.payload as SingleValuePayload<boolean>;
        setUpdaterWindowDisabled(disabled.value);
        if (disabled.value == true) {
            appStateContext?.setState(AppState.Default);
            setUpdateState(UpdateState.NotReady);
        }
    })

    console.log("updater disabled: ", updaterWindowDisabled);
    const {classes} = actionsStyles();

    return (
        <MantineProvider withGlobalStyles withNormalizeCSS>
            <div hidden={updaterWindowDisabled}>
                <Box style={{
                        position: "absolute",
                        right: 150,
                        bottom: 200,
                        zIndex: 99,
                        width: 350,
                        height: 120,
                        backgroundImage: `url(${updaterBack})`,
                        backgroundRepeat: "no-repeat",
                        backgroundSize: "hover",
                    }}>
                    <Text
                        style={{
                            position: "relative",
                            top: 25,
                            fontFamily: "Gabriela, sans-serif"
                        }}
                        align='center'>
                        {currentlyUpdatedFile}</Text>
                    <Progress 
                        style={{
                            width: 275,
                            position: "relative",
                            left: 30,
                            top: 35
                        }}
                        size="xl"
                        radius={0}
                        value={currentDownloadProgress}>
                    </Progress>
                </Box>
            </div>
            <Button
                className={classes.button}
                onClick={(e) => {
                    setUpdaterWindowDisabled(false);
                    appStateContext?.setState(AppState.Busy);
                    setUpdateState(UpdateState.InProcess);
                    invoke("download_update")
                }}
                disabled={!(appStateContext?.state == AppState.Default) || !(updateState == UpdateState.Ready)}>
                Обновить мод...
            </Button>
        </MantineProvider>
    )
}