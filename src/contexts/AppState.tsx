import { useState, createContext, PropsWithChildren, useContext, useEffect } from "react";

export enum AppState {
    Default,
    Patching,
    Busy
}

export type AppStateType = {
    state: AppState;
    setState: (state: AppState) => void;
}

export const AppStateContext = createContext<AppStateType | undefined>(undefined);
const AppStateProvider = ({children} : PropsWithChildren<{}>) => {
    const [state, setState] = useState<AppStateType['state']>(AppState.Default);

    return(
        <AppStateContext.Provider value={{state, setState}}>
            {children}
        </AppStateContext.Provider>
    )
}

export const useAppStateContext = () => {
    const context = useContext(AppStateContext);
    return context;
}

export default AppStateProvider;