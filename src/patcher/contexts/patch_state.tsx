import { useState, createContext, PropsWithChildren, useContext } from "react";

export enum PatchState {
    Inactive = "Inactive", // can press only map pick button
    MapPicked = "MapPicked", // technical state to refresh settings
    Active = "Active", // can press any button
    Configuring = "Configuring", // using settings windows, can only close them
    Patching = "Patching" // can't press anything
}

export type PatchStateType = {
    state: PatchState;
    setState: (state: PatchState) => void;
}

export const PatchStateContext = createContext<PatchStateType | undefined>(undefined);

const PatchStateProvider = ({children} : PropsWithChildren<{}>) => {
    const [state, setState] = useState<PatchStateType['state']>(PatchState.Inactive);

    return(
        <PatchStateContext.Provider value={{state, setState}}>
            {children}
        </PatchStateContext.Provider>
    )
}

export const usePatchStateContext = () => {
    const context = useContext(PatchStateContext);
    return context;
}

export default PatchStateProvider;