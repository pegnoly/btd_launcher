import { PropsWithChildren, createContext, useContext, useState } from "react";
import { MapMode } from "../components/main";

export type MapModesType = {
    state: MapMode[],
    setState: (state: MapMode[]) => void
}

export const MapModesContext = createContext<MapModesType | undefined>(undefined);

const MapModesProvider = ({children} : PropsWithChildren<{}>) => {
    const [state, setState] = useState<MapModesType['state']>([]);
    console.log("Modes: ", state);
    return(
        <MapModesContext.Provider value={{state, setState}}>
            {children}
        </MapModesContext.Provider>
    )
}

export const useMapModesContext = () => {
    const context = useContext(MapModesContext);
    return context;
}

export default MapModesProvider;