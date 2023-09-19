import { BackgroundImage, Box, Grid, MantineProvider, Progress,
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
import updaterBack from "./assets/updater_back.png"
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

export type PatcherVisibility = {
  visible: boolean
}

export type SingleValuePayload<T> = {
  value: T
}

invoke("start_updater");
invoke("check_can_activate_download");

export default function App() {
  const { classes } = useStyles();

  const [currentMode, changeMode] = useState<Info>(currentGameMode);
  const [patcherVisibility, setPatcherVisibility] = useState<boolean>(true);
  const [updaterWindowDisabled, setUpdaterWindowDisabled] = useState<boolean>(true);

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

  const updaterVisibilityChangedListener = listen("updater_visibility_changed", (event) => {
    let visibility = event.payload as SingleValuePayload<boolean>;
    console.log("updater visibility changed: ", visibility.value);
    setUpdaterWindowDisabled(visibility.value);
  })

  const updatedFileChangedListener = listen("updated_file_changed", (event) => {
    let file = event.payload as SingleValuePayload<string>;
    changeUpdatedFile(file.value);
  })

  const downloadProgressChanged = listen("download_progress_changed", (event) => {
    let percent = event.payload as SingleValuePayload<number>;
    console.log(percent.value)
    changeDownloadProgress(percent.value * 100);
  })

  const [currentlyUpdatedFile, changeUpdatedFile] = useState<string>("123");
  const [currentDownloadProgress, changeDownloadProgress] = useState<number>(0);

  return (
    <MantineProvider withGlobalStyles withNormalizeCSS>
        <Box hidden={updaterWindowDisabled}
          style={{
            position: "absolute",
            left: 300,
            top: 200,
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