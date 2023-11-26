import { SegmentedControl, Text } from "@mantine/core"
import { GameMode, useGameModeContext } from "../contexts/GameMode"
import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { AppState, useAppStateContext } from "../contexts/AppState";

export default function ModeSwitcher() {
    const gameModeContext = useGameModeContext();
    const appStateContext = useAppStateContext();
    
    function gameModeChanged(mode: GameMode) {
        invoke("switch_mode", {newMode: mode});
        gameModeContext?.setState(mode);
        appStateContext?.setState(AppState.Busy);
    }

    const fileTransferEndedListener = listen("file_transfer_ended", (event) => {
        appStateContext?.setState(AppState.Default);
    })

    return(
        <div style={{
            position: "absolute",
            top: -250,
            left: 335
        }}>
            <SegmentedControl disabled={!(appStateContext?.state == AppState.Default)}
                style={{
                    position: 'absolute',
                    top: 500,
                    left: 275
                }}
                defaultValue={gameModeContext?.state}
                onChange={gameModeChanged}
                data={[
                {
                    value: GameMode.Duel,
                    label: (
                        <Text ta='right'>Дуэль</Text>
                    )
                },
                {
                    value: GameMode.RMG,
                    label: (
                        <Text ta='left'>РМГ</Text>
                    ) 
                },
            ]}/>
    </div>
    )
}