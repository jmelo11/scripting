import React, { useState } from 'react';
import { Box, Button, Typography, Stack, Divider, Alert, Paper, IconButton, Grid } from '@mui/material';
import { v4 as uuidv4 } from 'uuid';
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
    const { currentScript, updateEvents } = useScripts();

    const [events, setEvents] = useState<ScriptEvent[]>(currentScript?.events ? currentScript.events.map((event: ScriptEvent) => ({
        ...event,
        eventDate: event.eventDate,
    })) : []);
    const [unsavedChanges, setUnsavedChanges] = useState(false);
    const [alert, setAlert] = useState<string | null>(null);
    const [expanded, setExpanded] = useState<string | false>(false);

    const handleAddEvent = () => {
        setEvents([...events, { id: uuidv4(), eventDate: new Date(), script: '' }]);
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
        updateEvents(events);
        setUnsavedChanges(false);
        setAlert('Events saved successfully');
        setTimeout(() => setAlert(null), 3000);

        // save a json file with the events in the assets
        // folder

        const eventsJson = JSON.stringify(events, null, 2);
        const blob = new Blob([eventsJson], { type: 'application/json' });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = 'events.json';
        a.click();
        URL.revokeObjectURL(url);

    };

    const handleChange = (panel: string) => (_: React.SyntheticEvent, isExpanded: boolean) => {
        setExpanded(isExpanded ? panel : false);
    };



    const handleRun = () => {
        const request = {
            ...currentScript,
            referenceDate: currentScript?.referenceDate.toISOString().split('T')[0],
            events: events.map((event: ScriptEvent) => ({
                ...event,
                eventDate: event.eventDate.toISOString().split('T')[0]
            }))
        }

        console.log(request);

        fetch('http://localhost:8000/execute', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(request)
        })
            .then(response => response.json())
            .then(data => {
                console.log(data);
            })
            .catch((error) => {
                console.error('Error:', error);
            });
    }

    const menuTheme = createTheme(menuThemeOptions);

    return (
        <ThemeProvider theme={menuTheme}>
            <LocalizationProvider dateAdapter={AdapterDayjs}>
                <MainLayout>
                    <Paper>
                        <Box sx={{ p: 2 }}>
                            <Stack spacing={2}>
                                <Grid container sx={{
                                    display: 'flex',
                                    justifyContent: 'space-between',
                                    alignItems: 'center',
                                }}>
                                    <Grid item sm={6}>
                                        <Typography variant="h5" component="h2" align="left">Edit Events</Typography>
                                    </Grid>
                                    <Grid item sm={6}>
                                        {unsavedChanges && <Alert severity="warning">You have unsaved changes!</Alert>}
                                        {alert && <Alert severity="success">{alert}</Alert>}
                                    </Grid>
                                </Grid>
                                <Divider />
                                <Box sx={{ display: 'flex', justifyContent: 'flex-end' }}>
                                    <Stack direction="row" spacing={1}>
                                        <Button
                                            onClick={handleAddEvent}
                                            variant="outlined"
                                            startIcon={<AddIcon />}
                                        >
                                            Add Event
                                        </Button>
                                        <Button variant="contained" onClick={handleRun}>
                                            Run
                                        </Button>
                                        <Button onClick={handleSave} variant="contained" color="primary">
                                            Save
                                        </Button>
                                    </Stack>
                                </Box>
                                <Stack spacing={2}>
                                    {events.map((event: ScriptEvent) => (
                                        <Accordion
                                            key={event.id}
                                            onChange={handleChange(event.id)}
                                        >
                                            <AccordionSummary
                                                expandIcon={<ExpandMoreIcon />}
                                                aria-controls={`panel-${event.id}-content`}
                                                id={`panel-${event.id}-header`}
                                                sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}
                                            >
                                                <Typography variant='body1'>Id: {event.id}</Typography>
                                            </AccordionSummary>
                                            <AccordionDetails>
                                                <Box
                                                    sx={{
                                                        display: 'flex',
                                                        justifyContent: 'flex-end',
                                                        width: '100%',
                                                        marginBottom: 2,

                                                    }}>
                                                    <Button
                                                        startIcon={<DeleteOutlineOutlinedIcon />}
                                                        size='small'
                                                        onClick={() => handleRemoveEvent(event.id)}>
                                                        Delete
                                                    </Button>
                                                </Box>
                                                <ScriptingEvent
                                                    initialScript={event.script}
                                                    initialDate={dayjs(event.eventDate)}
                                                    onScriptChange={(value) => handleScriptChange(event.id, value)}
                                                    onDateChange={(value) => handleDateChange(event.id, value)}
                                                    scriptError={null}
                                                    dateError={null}
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
