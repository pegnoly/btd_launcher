import { listen } from "@tauri-apps/api/event";
import { Actions } from "../../Actions";
import ModeSwitcher from "../../components/ModeSwitcher";
import { SingleValuePayload } from "../../App";
import { AppState, useAppStateContext } from "../../contexts/AppState";

export default function MainContainer() {
    const appStateContext = useAppStateContext();
    const gameClosedListener = listen("game_closed", (e) => {
        let isClosed = e.payload as SingleValuePayload<boolean>;
        if (isClosed.value == false) {
            appStateContext?.setState(AppState.Busy)
        }
        else {
            appStateContext?.setState(AppState.Default)
        }
    });

    return(
        <>
            <ModeSwitcher/>
            <Actions/>
        </>
    )
}