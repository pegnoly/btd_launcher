import { MantineProvider } from "@mantine/core";
import { useState } from "react";

import { ActionButton } from "../Actions";
import { useAppStateContext, AppState } from "../contexts/AppState";
import { useGameModeContext, GameMode } from "../contexts/GameMode";
import PatcherMain from "./components/main";

export default function Patcher() {
  const [visible, setVisible] = useState<boolean>(false);

  const appStateContext = useAppStateContext();
  const gameModeContext = useGameModeContext();

  async function patcherButtonClicked(x: React.MouseEvent<HTMLButtonElement, MouseEvent>) {
      if (visible == false) {
          appStateContext?.setState(AppState.Patching)
      }
      else {
          appStateContext?.setState(AppState.Default)
      }
      setVisible(!visible);
  }
  
  return (
    <MantineProvider theme={{fontFamily: "Geologica, sans-serif"}} withGlobalStyles withNormalizeCSS>
      <ActionButton
        disabled={appStateContext?.state == AppState.Busy || gameModeContext?.state == GameMode.Duel}
        onClickFunction={patcherButtonClicked}
        text="Патчер карт"
      />
      <PatcherMain visible={visible}/>
    </MantineProvider>
  );
}