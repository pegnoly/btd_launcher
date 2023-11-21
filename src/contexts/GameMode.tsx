import { useState, useEffect, createContext, PropsWithChildren, useContext } from "react";

export enum GameMode {
    Duel = "Duel",
    RMG = "RMG",
}

type GameModeType = {
    state: GameMode;
    setState: (state: GameMode) => void;
};

export const GameModeContext = createContext<GameModeType | undefined>(undefined);

const GameModeProvider = ({children} : PropsWithChildren<{}>) => {
    const [state, setState] = useState<GameModeType['state']>(() => {
        let mode = localStorage.getItem("global_game_mode");
        if (mode == null) {
            return GameMode.Duel;
        }
        else {
            return JSON.parse(mode) as GameMode;
        }
    });

    useEffect(() => {
        localStorage.setItem("global_game_mode", JSON.stringify(state))
    }, [state])

    return(
        <GameModeContext.Provider value={{state, setState}}>
            {children}
        </GameModeContext.Provider>
    )
}

export const useGameModeContext = () => {
    const context = useContext(GameModeContext);
    return context;
}

export default GameModeProvider;