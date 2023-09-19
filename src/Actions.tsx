import { Stack, Button, Box, MantineProvider } from "@mantine/core"
import {GameMode, Info, SingleValuePayload} from "./App"
import {invoke} from "@tauri-apps/api"
import { emit, listen } from '@tauri-apps/api/event'

import { createStyles, Image } from "@mantine/core"
import testButton from "./test_btn.png"
import actionsBackround from "./patcher/assets/actions_panel_back.png"
import { url } from "inspector"
import {WebviewWindow, appWindow, getAll} from "@tauri-apps/api/window"
import activeButton from "./patcher/assets/active.png"
import hoveredButton from "./patcher/assets/hovered.png"
import disabledButton from "./patcher/assets/inactive.png"
import { useState } from "react"


const actionsStyles = createStyles((theme) => ({
    back: {
        // backgroundImage: `url(${actionsBackround})`,
        // backgroundSize: 'contain',
        // backgroundRepeat: 'no-repeat',
        //height: 500,
        position: "absolute",
        top: -300
    },
    button : {
        backgroundColor: "transparent",
        width: 250,
        height: 48,
        backgroundImage: `url(${activeButton})`,
        backgroundSize: 'hover',
        fontFamily: "Gabriela, sans-serif",
        fontSize: 20,   
        ":hover": {
            backgroundImage: `url(${hoveredButton})`,
            backgroundSize: 'hover',
            backgroundColor: "transparent",
            border: "none"
        },
        ":disabled": {
            backgroundImage: `url(${disabledButton})`,
            backgroundSize: 'hover',
            backgroundColor: "transparent",
            border: "none"
        }
    }
}));


// function wheelButtonClicked(x: React.MouseEvent<HTMLButtonElement, MouseEvent>, mode: GameMode) {
//     invoke("check_for_update", {mode: mod});
// }
async function patcherButtonClicked(x: React.MouseEvent<HTMLButtonElement, MouseEvent>) {
    invoke("show_patcher");
}

export function Actions(mode: Info) {
    const {classes} = actionsStyles()
    function wheelButtonClicked(x: React.MouseEvent<HTMLButtonElement, MouseEvent>) {
        invoke("check");
    }

    const [downloaderDisabled, setDownloaderDisabled] = useState<boolean>(true); 

    const downloadStateChangedListener = listen("download_state_changed", (event) => {
        let activity = event.payload as SingleValuePayload<boolean>;
        console.log("activity: ", activity.value);
        setDownloaderDisabled(activity.value);
    })

    return (   
        <MantineProvider withGlobalStyles withNormalizeCSS>
            <Box className={classes.back}>
                <Stack spacing={0} >
                    <Button className={classes.button}>Мануал</Button>
                    <Button className={classes.button} onClick={wheelButtonClicked}>Колесо умений</Button>
                    <Button 
                        className={classes.button}
                        disabled={mode.mode === GameMode.Duel ? true : false} 
                        onClick={patcherButtonClicked}>
                        Патчер карт
                    </Button>
                    <Button
                        className={classes.button}
                        onClick={(e) => invoke("download_update")}
                        disabled={downloaderDisabled}>
                        Обновить мод...
                    </Button>
                    <Button 
                        className={classes.button}
                        onClick={(e) => invoke("start_update_process")}>
                        Запустить игру
                    </Button>
                </Stack>
            </Box>
        </MantineProvider>
    )
}
