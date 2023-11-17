import { BackgroundImage, Box, Grid, MantineProvider, Progress,
  SegmentedControl, Stack, Text, Image, createStyles, rem, Center, StarIcon } from '@mantine/core';
import { HeroImageRight } from './back';
import { Button, Group } from '@mantine/core';
import { IconEye, IconCode, IconExternalLink } from '@tabler/icons-react';
import { Actions } from './Actions';
import { DescriptionRender } from './Desc';

import { useState } from 'react';
import styles from "./App.css"
import { event, invoke } from '@tauri-apps/api';
import { emit, listen } from '@tauri-apps/api/event'

import mainBack from "./assets/main_back.png"
import btdLogo from "./patcher/assets/btd_logo.png"
import Patcher from './patcher/patcher';
import { Donates } from './Donate';
import AppStateProvider, { AppStateContext } from './contexts/AppState';
import GameModeProvider from './contexts/GameMode';
import ModeSwitcher from './components/ModeSwitcher';

invoke("start_update_thread");

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
  duel_button: {
    // backgroundImage:
    //   'url("https://i.ibb.co/G2cHnWG/Screenshot-2.png")',
    backgroundSize: 'cover',
    backgroundPosition: 'center',
  },
  main_desc: {
    fontFamily: 'Josefin Sans, sans-serif'
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

function gameModeChanged(s: GameMode) {
  invoke("switch_mode", {newMode: s})
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
                  <ModeSwitcher/>
                  <Actions/>
                </AppStateProvider>
              </GameModeProvider>
            </Grid.Col>
          </Grid>
        </div>
        <Donates data-tauri-drag-region />
      </Box>
    </MantineProvider>
  );
}