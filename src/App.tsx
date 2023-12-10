import { Box, Grid, MantineProvider, createStyles } from '@mantine/core';

import { Actions } from './Actions';
import { invoke } from '@tauri-apps/api';

import mainBack from "./assets/main_back.png"
import btdLogo from "./patcher/assets/btd_logo.png"
// import { Donates } from './Donate';
import AppStateProvider from './contexts/AppState';
import GameModeProvider from './contexts/GameMode';
import ModeSwitcher from './components/ModeSwitcher';
import MainContainer from './patcher/components/main_container';
import { Donates } from './Donate';

invoke("start_update_thread");

const useStyles = createStyles((theme) => ({
  hero: {
    position: 'relative',
    backgroundImage: `url(${mainBack})`,
    backgroundSize: 'hover',
    backgroundRepeat: 'no-repeat',
    // backgroundPosition: 'center',
    // backgroundColor: "aquamarine",
    overflow: "hidden",
    overflowY: "hidden",
  },
  head: {
    height: 125,
    width: 300,
    backgroundImage: `url(${btdLogo})`,
    backgroundSize: 'contain',
    backgroundRepeat: 'no-repeat',
    position: "relative",
    top: 25,
    left: 25
  },
  patcher_div: {
    position: "absolute",
    top: 175
  },
  grid_main: {
    position: "relative",
  },
}))

export type SingleValuePayload<T> = {
  value: T
}

export default function App() {
  const { classes } = useStyles();
  return (
    <MantineProvider withGlobalStyles withNormalizeCSS>
      <Box data-tauri-drag-region className={classes.hero}>
        <div data-tauri-drag-region>
          <Grid data-tauri-drag-region className={classes.grid_main}>
            <Grid.Col data-tauri-drag-region>
              <div data-tauri-drag-region className={classes.head}>
              </div>
              <div data-tauri-drag-region className={classes.patcher_div}>
                {/* <Patcher data-tauri-drag-region visible={patcherVisibility}/> */}
              </div>
            </Grid.Col>
            <Grid.Col data-tauri-drag-region offset={7.3}>
              <GameModeProvider>
                <AppStateProvider>
                  <MainContainer/>
                </AppStateProvider>
              </GameModeProvider>
            </Grid.Col>
          </Grid>
        </div>
        <Donates/>
      </Box>
    </MantineProvider>
  );
}