import React, { createContext, useContext, useState, ReactNode } from 'react';

interface DrawerContextType {
    drawerOpen: boolean;
    toggleDrawer: (open: boolean) => void;
}

const DrawerContext = createContext<DrawerContextType | undefined>(undefined);

export const useDrawer = () => {
    const context = useContext(DrawerContext);
    if (!context) {
        throw new Error('useDrawer must be used within a DrawerProvider');
    }
    return context;
};

export const DrawerProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
    const [drawerOpen, setDrawerOpen] = useState<boolean>(() => {
        const savedState = localStorage.getItem('drawerOpen');
        return savedState ? JSON.parse(savedState) : false;
    });

    const toggleDrawer = (open: boolean) => {
        setDrawerOpen(open);
        localStorage.setItem('drawerOpen', JSON.stringify(open));
    };

    return (
        <DrawerContext.Provider value={{ drawerOpen, toggleDrawer }}>
            {children}
        </DrawerContext.Provider>
    );
};

