import { Button, Group, Image, Popover, Stack, Text } from "@mantine/core"
import mir_icon from "./assets/mir.png"
import telegram_icon from "./assets/telegram.png"
import discord_icon from "./assets/discord_icon.png"
import qiwi_icon from "./assets/qiwi.png"
import alerts_icon from "./assets/alerts.png"
import { useState } from "react"
import { invoke } from "@tauri-apps/api"

export function Donates() {
    return(
        <>
            <div style={{
                position: "absolute",
                top: 5,
                right: -30,
                width: 200
            }}>
                <Stack spacing={2} style={{
                    }}>
                    <Text align="center" style={{
                        fontFamily: "Pattaya, sans-serif",
                        fontSize: 19,
                        fontVariant: "full-width",
                        textShadow: "initial",
                        borderColor: "black",
                        color: "darkviolet",
                    }}>by Gerter</Text>
                    {/* <Button style={{
                        fontFamily: "Gabriela, sans-serif",
                        color: "burlywood",
                        backgroundColor: "green",
                        height: 30,
                        width: 125
                    }} onClick={() => setContactsHidden(!contactsHidden)}>Контакты</Button> */}
                    <div>
                        <Group
                            style={{
                                position: "relative",
                                left: 65
                            }} 
                            spacing={10}>
                            <Button size="xs" style={{
                                backgroundImage: `url("${telegram_icon}")`,
                                backgroundRepeat: "no-repeat",
                                backgroundSize: "contain",
                                backgroundColor: "transparent",
                            }}
                            onClick={()=> invoke("start_telegram_dialog")}
                            ></Button>
                            <Button size="xs" style={{
                                backgroundImage: `url("${discord_icon}")`,
                                backgroundRepeat: "no-repeat",
                                backgroundSize: "contain",
                                backgroundColor: "transparent",
                            }}
                            onClick={() => invoke("open_discord_dialog")}
                            ></Button>
                        </Group>
                    </div>
                    <div>
                        <Text align="center" style={{
                            fontFamily: "Pattaya, sans-serif",
                            fontSize: 19,
                            fontVariant: "full-width",
                            textShadow: "initial",
                            borderColor: "black",
                            color: "darkviolet",
                        }}>Поддержать</Text>
                        <Group style={{
                                position: "relative",
                                top: 3,
                                left: 37
                            }}>
                            <Popover>
                                <Popover.Target>
                                    <Button size="xs" style={{
                                        backgroundImage: `url("${mir_icon}")`,
                                        backgroundRepeat: "no-repeat",
                                        backgroundSize: "contain",
                                    }}>
                                    </Button>
                                </Popover.Target>
                                <Popover.Dropdown>
                                    <Text size={12}>2200700897016969</Text>
                                </Popover.Dropdown>
                            </Popover>
                            <Button size="xs" style={{
                                backgroundImage: `url("${qiwi_icon}")`,
                                backgroundRepeat: "no-repeat",
                                backgroundSize: "contain",
                                backgroundColor: "transparent",
                            }}
                            onClick={() => invoke("start_qiwi_pay")}
                            ></Button>
                            <Button size="xs" style={{
                                backgroundImage: `url("${alerts_icon}")`,
                                backgroundRepeat: "no-repeat",
                                backgroundSize: "contain",
                                backgroundColor: "transparent",
                            }}></Button>
                        </Group>
                    </div>
                </Stack>
            </div>
        </>
    )
}