import { Stack, Button, Box, MantineProvider } from "@mantine/core"
import {invoke} from "@tauri-apps/api"
import { createStyles } from "@mantine/core"
import activeButton from "./patcher/assets/active.png"
import hoveredButton from "./patcher/assets/hovered.png"
import disabledButton from "./patcher/assets/inactive.png"
import { AppState, useAppStateContext} from "./contexts/AppState"
import { useGameModeContext } from "./contexts/GameMode"
import Updater from "./updater/Updater"
import Patcher from "./patcher/patcher"


export const actionsStyles = createStyles((theme) => ({
    back: {
        position: "absolute",
        top: -300
    },
    button : {
        backgroundColor: "transparent",
        width: 250,
        height: 48,
        backgroundImage: `url(${activeButton})`,
        backgroundSize: 'hover',
        fontFamily: "Gabriela",
        fontSize: 22,   
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

export type ActionButtonProps = {
    text: string;
    onClickFunction: (event: React.MouseEvent<HTMLButtonElement, MouseEvent>) => void;
    disabled: boolean;
}

export function ActionButton(props: ActionButtonProps) {
    const {classes} = actionsStyles();
    return (
      <MantineProvider withGlobalStyles withNormalizeCSS>
        <Button 
            className={classes.button}
            onClick={props.onClickFunction}
            disabled={props.disabled}>
          {props.text}
        </Button>
      </MantineProvider>
    )
  }
  
export function Actions() {
    const {classes} = actionsStyles()

    function wheelButtonClicked(x: React.MouseEvent<HTMLButtonElement, MouseEvent>) {
        invoke("show_wheel");
    }
    function manualButtonClicked(x: React.MouseEvent<HTMLButtonElement, MouseEvent>) {
        if (x.ctrlKey === true) {
            invoke("scan_files");
        }
        else {
            invoke("show_manual");
        }
    }
    
    const appStateContext = useAppStateContext();
    const gameModeContext = useGameModeContext();

    return (   
        <MantineProvider withGlobalStyles withNormalizeCSS>
            <div style={{
                position: "relative",
                top: 450,
                right: -40,
                height: 459
            }}>
            <Box className={classes.back}>
                <Stack spacing={0} >
                    <ActionButton onClickFunction={manualButtonClicked} text="Мануал" disabled={false}/>
                    <ActionButton onClickFunction={wheelButtonClicked} text="Колесо умений" disabled={false}/>
                    <Patcher/> 
                    <Updater/>
                    <Button
                        className={classes.button}
                        onClick={(e) => invoke("start_game")}
                        disabled={!(appStateContext?.state == AppState.Default)}>
                        Запустить игру
                    </Button>
                </Stack>
            </Box>
            </div>
        </MantineProvider>
    )
}
