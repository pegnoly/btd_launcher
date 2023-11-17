import { useState, createContext, PropsWithChildren, useContext } from "react";

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
    const [state, setState] = useState<GameModeType['state']>(GameMode.Duel)
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