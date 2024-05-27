import React, { useState } from 'react';
import { Box, Button, Typography, Stack, Divider, Alert, Paper, IconButton } from '@mui/material';
import { v4 as uuidv4 } from 'uuid';
import { useLocation } from 'react-router-dom';
import { ScriptingEvent } from '../components/ScriptingEvent';
import { useScripts, ScriptEvent } from '../contexts/ScriptsContext';
import MainLayout from '../components/MainLayout';
import dayjs, { Dayjs } from 'dayjs';
import { LocalizationProvider } from '@mui/x-date-pickers/LocalizationProvider';
import { AdapterDayjs } from '@mui/x-date-pickers/AdapterDayjs';
import { createTheme, ThemeProvider } from "@mui/material/styles";
import { menuThemeOptions } from '../css/theme';
import AddIcon from '@mui/icons-material/Add';
import Accordion from '@mui/material/Accordion';
import AccordionDetails from '@mui/material/AccordionDetails';
import AccordionSummary from '@mui/material/AccordionSummary';
import ExpandMoreIcon from '@mui/icons-material/ExpandMore';
import DeleteOutlineOutlinedIcon from '@mui/icons-material/DeleteOutlineOutlined';

const EventsPage: React.FC = () => {
    const location = useLocation();
    const { script } = location.state;
    const { updateEvents } = useScripts();

    const [events, setEvents] = useState<ScriptEvent[]>(script.events ? script.events.map((event: ScriptEvent) => ({
        ...event,
        eventDate: dayjs(event.eventDate),
    })) : []);
    const [unsavedChanges, setUnsavedChanges] = useState(false);
    const [alert, setAlert] = useState<string | null>(null);
    const [expanded, setExpanded] = useState<string | false>(false);

    const handleAddEvent = () => {
        setEvents([...events, { id: uuidv4(), eventDate: dayjs().toDate(), script: '' }]);
        setUnsavedChanges(true);
    };

    const handleRemoveEvent = (id: string) => {
        setEvents(events.filter((event: ScriptEvent) => event.id !== id));
        setUnsavedChanges(true);
    };

    const handleScriptChange = (id: string, script: string) => {
        setEvents(events.map((event: ScriptEvent) => event.id === id ? { ...event, script } : event));
        setUnsavedChanges(true);
    };

    const handleDateChange = (id: string, date: Dayjs | null) => {
        setEvents(events.map((event: ScriptEvent) => event.id === id ? { ...event, eventDate: date!.toDate() } : event));
        setUnsavedChanges(true);
    };

    const handleSave = () => {
        updateEvents(script.id, events);
        setUnsavedChanges(false);
        setAlert('Events saved successfully');
        setTimeout(() => setAlert(null), 3000);
    };

    const handleChange = (panel: string) => (event: React.SyntheticEvent, isExpanded: boolean) => {
        setExpanded(isExpanded ? panel : false);
    };

    const menuTheme = createTheme(menuThemeOptions);

    return (
        <ThemeProvider theme={menuTheme}>
            <LocalizationProvider dateAdapter={AdapterDayjs}>
                <MainLayout currentScriptId={script.id}>
                    <Paper>
                        <Box sx={{ p: 2 }}>
                            <Stack spacing={2}>
                                <Typography variant="h5" component="h2" align="left">Edit Events</Typography>
                                {unsavedChanges && <Alert severity="warning">You have unsaved changes!</Alert>}
                                {alert && <Alert severity="success">{alert}</Alert>}
                                <Divider sx={{ my: 2 }} />
                                <Box sx={{ display: 'flex', justifyContent: 'flex-end', mb: 2 }}>
                                    <Button
                                        onClick={handleAddEvent}
                                        variant="outlined"
                                        startIcon={<AddIcon />}
                                    >
                                        Add Event
                                    </Button>
                                    <Button onClick={handleSave} variant="contained" color="primary" sx={{ ml: 2 }}>
                                        Save
                                    </Button>
                                </Box>
                                <Stack spacing={2}>
                                    {events.map((event: ScriptEvent) => (
                                        <Accordion
                                            key={event.id}
                                            expanded={expanded === event.id}
                                            onChange={handleChange(event.id)}
                                        >
                                            <AccordionSummary
                                                expandIcon={<ExpandMoreIcon />}
                                                aria-controls={`panel-${event.id}-content`}
                                                id={`panel-${event.id}-header`}
                                                sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}
                                            >
                                                <Typography sx={{ flexShrink: 0 }}>{event.id}</Typography>
                                                <IconButton size='small' onClick={() => handleRemoveEvent(event.id)}>
                                                    <DeleteOutlineOutlinedIcon fontSize='inherit' />
                                                </IconButton>
                                            </AccordionSummary>
                                            <AccordionDetails>
                                                <ScriptingEvent
                                                    initialScript={event.script}
                                                    initialDate={dayjs(event.eventDate)}
                                                    onScriptChange={(value) => handleScriptChange(event.id, value)}
                                                    onDateChange={(value) => handleDateChange(event.id, value)}
                                                    scriptError={null}
                                                    dateError={null}
                                                    onRemove={() => handleRemoveEvent(event.id)} // Add onRemove prop here
                                                />
                                            </AccordionDetails>
                                        </Accordion>
                                    ))}
                                </Stack>
                            </Stack>
                        </Box>
                    </Paper>
                </MainLayout>
            </LocalizationProvider>
        </ThemeProvider>
    );
};

export default EventsPage;
