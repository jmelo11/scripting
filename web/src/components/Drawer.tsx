import { Box, Grid, IconButton } from '@mui/material';
import { createTheme } from '@mui/material/styles';
import { themeOptions } from '../css/theme';
import CloseIcon from '@mui/icons-material/Close';
import React from 'react';
import DrawerItems from './DrawerItems';

interface DrawerProps {
    open: boolean;
    toggleDrawer: (open: boolean) => void;
    currentScriptId?: string;
}

const Drawer: React.FC<DrawerProps> = (props) => {
    const theme = createTheme(themeOptions);

    return (
        <Box
            sx={{
                position: 'fixed',
                width: props.open ? '250px' : '0',
                height: '100%',
                overflowX: 'hidden',
                transition: '0.3s',
                boxShadow: theme.shadows[1],
            }}
        >
            <Grid container sx={{ height: '100%' }}>
                <Grid item xs={12} sx={{ display: 'flex', justifyContent: 'flex-end', paddingRight: 1 }}>
                    <IconButton onClick={() => props.toggleDrawer(!props.open)}>
                        <CloseIcon />
                    </IconButton>
                </Grid>
                <Grid item xs={12} sx={{ height: 'calc(100% - 48px)', overflowY: 'auto' }}>
                    <DrawerItems currentScriptId={props.currentScriptId} />
                </Grid>
            </Grid>
        </Box>
    );
};

export default Drawer;
