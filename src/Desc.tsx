import { MantineProvider, Text } from "@mantine/core";
import { Info, RenderProps } from "./App";
import { invoke } from "@tauri-apps/api";
import { useState } from "react";

interface DescriptionModel {
    title: string,
    desc: string
}

export function DescriptionRender(mode: Info) {
    const [currentDesc, changeDesc] = useState<string>("");
    const [currentTitle, changeTitle] = useState<string>("");
    invoke("set_desc_with_locale", {gameMode: mode.mode as string, locale: mode.locale as string}).then((val) => {
        //console.log("val: ", val);
        let d: DescriptionModel = JSON.parse(val as string);
        //console.log("desc ", d.desc);
        changeTitle(d.title);
        changeDesc(d.desc);
    });
    return(
    <MantineProvider theme={{fontFamily: "Geologica, sans-serif"}}>
        <Text 
            variant="gradient"
            gradient={{from: "red", to: "blue", deg: 138}}
            size="xl">
            {currentTitle}
        </Text>
        <Text>
            {currentDesc}
        </Text>
    </MantineProvider>
    )
}