import { BackgroundImage, Box, Grid, MantineProvider, 
  SegmentedControl, Stack, Text, Image, createStyles, rem, Center, StarIcon } from '@mantine/core';
import { HeroImageRight } from './back';
import { Button, Group } from '@mantine/core';
import { IconEye, IconCode, IconExternalLink } from '@tabler/icons-react';
import { Actions } from './Actions';
import { DescriptionRender } from './Desc';

import regular from './fonts/Belanosima-Regular.ttf'

import { useState } from 'react';
import styles from "./App.css"
import { event, invoke } from '@tauri-apps/api';
import { emit, listen } from '@tauri-apps/api/event'

import btdLogo from "./patcher/assets/btd_logo.png"
import Patcher from './patcher/patcher';

export enum GameMode {
  Duel = "Duel",
  RMG = "RMG",
  Blitz = "Blitz"
}

export enum Locale {
  Ru = "Ru",
  En = "En"
}

export type Info = {
  mode: GameMode,
  locale: Locale
}

const useStyles = createStyles((theme) => ({
  hero: {
    position: 'relative',
    backgroundImage: 'url("https://i.ibb.co/gZ57Py7/Pwl-C3-Texture.png")',
    backgroundSize: 'hover',
    backgroundRepeat: 'no-repeat',
    // backgroundPosition: 'center',
    // backgroundColor: "aquamarine",
    overflow: "hidden",
    overflowY: "hidden",
  },
  stack_main: {
    position: 'absolute',
    top: 500,
    left: 275
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
  duel_button: {
    backgroundImage:
      'url("https://i.ibb.co/G2cHnWG/Screenshot-2.png")',
    backgroundSize: 'cover',
    backgroundPosition: 'center',
  },
  main_desc: {
    fontFamily: 'Josefin Sans, sans-serif'
  },
  low_div: {
    position: "absolute",
    top: -250,
    left: 335
  },
  actions_div: {
    position: "relative",
    top: 450,
    right: -40,
    height: 459
  }
}))

let currentGameMode: Info = {
  mode: GameMode.Duel,
  locale: Locale.Ru
};

export type RenderProps = {
  s: string
}

type AppConfig = {
  configDir: string
}

let currentConfig: AppConfig = {
  configDir: ""
}

const unlisten = listen("started", (event) => {
  currentConfig = event.payload as AppConfig
  console.log("current config path: ", currentConfig.configDir)
})

export type PatcherVisibility = {
  visible: boolean
}


export default function App() {
  const { classes } = useStyles();

  const [currentMode, changeMode] = useState<Info>(currentGameMode);
  const [patcherVisibility, setPatcherVisibility] = useState<boolean>(true);

  function gameModeChanged(s: GameMode) {
    invoke("disable_current_mode", {prevMode: currentGameMode.mode})
    currentGameMode.mode = s;
    invoke("set_active_mode", {newMode: currentGameMode.mode})
    let k: Info = {mode: s, locale: currentGameMode.locale};
    changeMode(k);
  }

  const patcherVisibilityChangedListener = listen("patcher_visibility_changed", (event) => {
    let visibility = event.payload as PatcherVisibility;
    setPatcherVisibility(visibility.visible);
  })

  return (
    <MantineProvider withGlobalStyles withNormalizeCSS>
      <Box className={classes.hero}>
        <div>
          <Grid className={classes.grid_main}>
            <Grid.Col>
              <div className={classes.head}>
              </div>
              <div className={classes.patcher_div}>
                <Patcher visible={patcherVisibility}/>
              </div>
            </Grid.Col>
            <Grid.Col offset={7.3}>
              <div className={classes.low_div}>
                <SegmentedControl className={classes.stack_main}
                  defaultValue={currentMode.mode}
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
                  ]}
                />
              </div>
              <div className={classes.actions_div}>
                <Actions mode={currentMode.mode} locale={currentMode.locale}/>
              </div>
            </Grid.Col>
          </Grid>
        </div>
      </Box>
    </MantineProvider>
  );
}