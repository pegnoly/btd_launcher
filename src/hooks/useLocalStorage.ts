import { useEffect, useState } from "react";

export function useLocalStorage2<T>(key: string, initialValue: T) {
    console.log("initial valu321e ", initialValue);
    const [value, setValue] = useState<T>(() => {
        // const json = localStorage.getItem(key);
        // if (json != null) {
        //     return JSON.parse(json);
        // }
        console.log("initial value ", initialValue);
        return initialValue;
    });

    useEffect(() => {
        localStorage.setItem(key, JSON.stringify(value))
    }, [key, value]);

    return [value, setValue] as [typeof value, typeof setValue];
}