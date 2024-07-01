// ScriptsContext.tsx
import React, { createContext, useContext, useState, ReactNode } from 'react';

export interface ScriptEvent {
    id: string;
    eventDate: Date;
    script: string;
}

interface ScriptMacros {
    id: string;
    key: string;
    value: string;
}

export interface Script {
    id: string;
    referenceDate: Date;
    macros: ScriptMacros[];
    events?: ScriptEvent[];
}

interface ScriptsContextType {
    scripts: Script[];
    currentScript: Script | undefined;
    setCurrentScript: (script: Script | undefined) => void;
    addScript: (script: Script) => void;
    updateScript: (updatedScript: Script) => void;
    updateEvents: (events: ScriptEvent[]) => void;
}

const ScriptsContext = createContext<ScriptsContextType | undefined>(undefined);

export const useScripts = (): ScriptsContextType => {
    const context = useContext(ScriptsContext);
    if (!context) {
        throw new Error('useScripts must be used within a ScriptsProvider');
    }
    return context;
};

export const ScriptsProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
    const [scripts, setScripts] = useState<Script[]>([]);
    const [currentScript, setCurrentScript] = useState<Script | undefined>(undefined);

    const addScript = (script: Script) => {
        setScripts([...scripts, script]);
    };

    const updateScript = (updatedScript: Script) => {
        setScripts(scripts.map(script => script.id === updatedScript.id ? updatedScript : script));
    };

    const updateEvents = (events: ScriptEvent[]) => {
        setScripts(scripts.map(script => script.id === currentScript?.id ? { ...script, events } : script));
        setCurrentScript({ ...currentScript!, events });
    };


    return (
        <ScriptsContext.Provider value={{ scripts, currentScript, addScript, updateScript, updateEvents, setCurrentScript }}>
            {children}
        </ScriptsContext.Provider>
    );
};
