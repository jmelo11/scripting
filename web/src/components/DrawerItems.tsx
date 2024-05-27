import React, { useState } from 'react';
import { useNavigate, useLocation } from 'react-router-dom';
import { Box, List, ListItem, ListItemButton, ListItemIcon, ListItemText, Stack, Typography, Collapse, Divider, Tooltip } from '@mui/material';
import { ExpandLess, ExpandMore } from '@mui/icons-material';
import FolderOutlinedIcon from '@mui/icons-material/FolderOutlined';
import AddIcon from '@mui/icons-material/Add';
import SettingsIcon from '@mui/icons-material/Settings';
import ArticleIcon from '@mui/icons-material/Article';
import NewScriptModal from './NewScriptModal';
import PinOutlinedIcon from '@mui/icons-material/PinOutlined';
import { useScripts } from '../contexts/ScriptsContext';
import { useDrawer } from '../contexts/DrawerContext';

interface DrawerItemsProps {
    currentScriptId?: string;
}

const generalButtons = [
    { icon: <AddIcon />, label: 'New' },
    { icon: <SettingsIcon />, label: 'Global Settings' },
];

const subMenuButtons = [
    { icon: <PinOutlinedIcon />, label: 'Events' },
    { icon: <ArticleIcon />, label: 'Script' },
    { icon: <SettingsIcon />, label: 'Settings' },
];

const DrawerItems: React.FC<DrawerItemsProps> = ({ currentScriptId }) => {
    const { scripts, addScript } = useScripts();
    const { drawerOpen } = useDrawer();
    const location = useLocation();
    const [openSubMenus, setOpenSubMenus] = useState<{ [key: string]: boolean }>({});
    const [isNewScriptModalOpen, setIsNewScriptModalOpen] = useState(false);
    const navigate = useNavigate();

    const handleGeneralButtonClick = (label: string) => {
        if (label === 'New') {
            setIsNewScriptModalOpen(true);
        } else {
            console.log(`General button ${label} clicked`);
        }
    };

    const handleSubMenuButtonClick = (label: string, script: any) => {
        if (label === 'Settings') {
            navigate('/script-settings', { state: { script, drawerOpen: true } });
        } else if (label === 'Events') {
            navigate('/events', { state: { script, drawerOpen: true } });
        } else {
            console.log(`Sub-menu button ${label} clicked`);
        }
    };

    const handleToggleSubMenu = (id: string) => {
        setOpenSubMenus((prevOpenSubMenus) => ({
            ...prevOpenSubMenus,
            [id]: !prevOpenSubMenus[id],
        }));
    };

    const handleAddScript = (scriptName: string) => {
        addScript({ id: scriptName, referenceDate: new Date(), macros: [], events: [] });
    };

    const existingScriptNames = scripts.map(script => script.id);

    return (
        <Box display={'flex'}>
            <Stack sx={{ width: '100%' }} spacing={1}>
                <Typography variant="h6">Scripts</Typography>
                <List dense>
                    {generalButtons.map((item, index) => (
                        <ListItem key={index} disablePadding>
                            <ListItemButton onClick={() => handleGeneralButtonClick(item.label)}>
                                <ListItemIcon>{item.icon}</ListItemIcon>
                                <ListItemText primary={item.label} />
                            </ListItemButton>
                        </ListItem>
                    ))}
                </List>
                <Divider />
                <Typography variant="h6">My Scripts</Typography>
                <List dense>
                    {scripts.map((item, index) => (
                        <React.Fragment key={index}>
                            <ListItem disablePadding>
                                <ListItemButton
                                    onClick={() => handleToggleSubMenu(item.id)}
                                    sx={{ backgroundColor: item.id === currentScriptId ? 'rgba(0, 0, 255, 0.1)' : 'transparent' }}
                                >
                                    <ListItemIcon>
                                        <FolderOutlinedIcon />
                                    </ListItemIcon>
                                    <Tooltip title={item.id}>
                                        <ListItemText primary={
                                            <Typography noWrap>
                                                {item.id}
                                            </Typography>
                                        } />
                                    </Tooltip>
                                    {openSubMenus[item.id] ? <ExpandLess /> : <ExpandMore />}
                                </ListItemButton>
                            </ListItem>
                            <Collapse in={openSubMenus[item.id]} timeout="auto" unmountOnExit>
                                <List component="div" disablePadding dense>
                                    {subMenuButtons.map((subItem, subIndex) => (
                                        <ListItem key={subIndex} disablePadding>
                                            <ListItemButton sx={{ pl: 4 }} onClick={() => handleSubMenuButtonClick(subItem.label, item)}>
                                                <ListItemIcon>{subItem.icon}</ListItemIcon>
                                                <ListItemText primary={subItem.label} />
                                            </ListItemButton>
                                        </ListItem>
                                    ))}
                                </List>
                            </Collapse>
                        </React.Fragment>
                    ))}
                </List>
            </Stack>
            <NewScriptModal
                open={isNewScriptModalOpen}
                onClose={() => setIsNewScriptModalOpen(false)}
                onAddScript={handleAddScript}
                existingScriptNames={existingScriptNames}
            />
        </Box>
    );
};

export default DrawerItems;
