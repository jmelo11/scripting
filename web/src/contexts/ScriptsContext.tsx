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

interface Script {
    id: string;
    referenceDate: Date;
    macros: ScriptMacros[];
    events?: ScriptEvent[];
}

interface ScriptsContextType {
    scripts: Script[];
    addScript: (script: Script) => void;
    updateScript: (updatedScript: Script) => void;
    updateEvents: (scriptId: string, events: ScriptEvent[]) => void;
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

    const addScript = (script: Script) => {
        setScripts([...scripts, script]);
    };

    const updateScript = (updatedScript: Script) => {
        setScripts(scripts.map(script => script.id === updatedScript.id ? updatedScript : script));
    };

    const updateEvents = (scriptId: string, events: ScriptEvent[]) => {
        setScripts(scripts.map(script => script.id === scriptId ? { ...script, events } : script));
    };

    return (
        <ScriptsContext.Provider value={{ scripts, addScript, updateScript, updateEvents }}>
            {children}
        </ScriptsContext.Provider>
    );
};
