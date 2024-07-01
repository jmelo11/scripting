// MainLayout.tsx
import React from 'react';
import Navbar from './Navbar';
import Drawer from './Drawer';
import { Box } from '@mui/material';
import { useDrawer } from '../contexts/DrawerContext';
import { useScripts } from '../contexts/ScriptsContext';
import { useLocation } from 'react-router-dom';

interface MainLayoutProps {
    children: React.ReactNode;
}

export default function MainLayout(props: MainLayoutProps) {

    const { drawerOpen, toggleDrawer } = useDrawer();
    const { currentScript } = useScripts();

    const location = useLocation();
    const isHomePage = location.pathname === '/';

    return (
        <Box sx={{ display: 'flex', height: '100vh' }}>
            <Drawer open={drawerOpen} toggleDrawer={toggleDrawer} />
            <Box
                sx={{
                    display: 'flex',
                    flexDirection: 'column',
                    marginLeft: drawerOpen ? '250px' : '0', // Fixed width to start with
                    transition: 'margin-left 0.3s',
                    width: '100%',
                    height: '100vh', // Ensure this box takes up the full height
                }}
            >
                <Navbar onMenuClick={() => toggleDrawer(!drawerOpen)} />
                <Box
                    sx={{
                        padding: '1rem',
                        backgroundColor: '#FAFAF9', // Light gray background
                        backgroundImage: 'radial-gradient(#ccc 1px, transparent 1px)',
                        backgroundSize: '25px 25px', // Adjust the size of the dots and grid
                        height: '100%', // Ensure this box takes up the full available height
                        flexGrow: 1, // Ensure it grows to fill the parent
                    }}
                >
                    {currentScript || isHomePage ? props.children : <Box>404 Not Found</Box>}
                </Box>
            </Box>
        </Box>
    );
}
