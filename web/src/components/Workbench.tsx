import { Box, Button, ButtonGroup, Grow, Popper, Stack } from '@mui/material'
import React from 'react'
import Paper from '@mui/material/Paper';
import MenuItem from '@mui/material/MenuItem';
import MenuList from '@mui/material/MenuList';
import ClickAwayListener from '@mui/material/ClickAwayListener';
import ArrowDropDownIcon from '@mui/icons-material/ArrowDropDown';
import AddCircleOutlineIcon from '@mui/icons-material/AddCircleOutline';
import { ScriptingEvent } from './ScriptingEvent';
import { Dayjs } from 'dayjs';

const generateUniqueId = () => {
    return 'xxxx-xxxx-xxx-xxxx'.replace(/[x]/g, (c) => {
        const r = Math.floor(Math.random() * 16);
        return r.toString(16);
    });
}

interface WorkbenchMenuProps {
    onRun: () => void;
    onStop: () => void;
    onPause: () => void;
    onAddEvent: () => void;
}

function WorkbenchMenu(props: WorkbenchMenuProps) {
    const options = [
        'Run',
        'Stop',
        'Pause',
    ];

    const anchorRef = React.useRef<HTMLDivElement>(null);
    const [open, setOpen] = React.useState<boolean>(false);
    const [selectedIndex, setSelectedIndex] = React.useState<number>(0);

    const handleClick = () => {
        if (selectedIndex === 0) {
            props.onRun();
        }
        if (selectedIndex === 1) {
            props.onStop();
        }
        if (selectedIndex === 2) {
            props.onPause();
        }
    };

    const handleMenuItemClick = (
        event: React.MouseEvent<HTMLLIElement, MouseEvent>,
        index: number,
    ) => {
        setSelectedIndex(index);
        setOpen(false);
    };

    const handleToggle = () => {
        setOpen((prevOpen) => !prevOpen);
    };

    const handleClose = (event: Event) => {
        if (anchorRef.current && anchorRef.current.contains(event.target as HTMLElement)) {
            return;
        }

        setOpen(false);
    };

    const handleAddEvent = () => {
        props.onAddEvent();
    }

    return (
        <Stack direction="row" sx={{ justifyContent: 'space-between', width: '100%' }}>
            <Stack direction="row" spacing={2}>
                <Button onClick={handleAddEvent} startIcon={<AddCircleOutlineIcon />}>
                    Event
                </Button>
            </Stack>
            <ButtonGroup
                variant="outlined"
                ref={anchorRef}
                aria-label="Button group with a nested menu"
            >
                <Button onClick={handleClick}>{options[selectedIndex]}</Button>
                <Button
                    aria-controls={open ? 'split-button-menu' : undefined}
                    aria-expanded={open ? 'true' : undefined}
                    aria-label="select merge strategy"
                    aria-haspopup="menu"
                    onClick={handleToggle}
                >
                    <ArrowDropDownIcon />
                </Button>
            </ButtonGroup>
            <Popper
                sx={{ zIndex: 1 }}
                open={open}
                anchorEl={anchorRef.current}
                role={undefined}
                transition
                disablePortal
            >
                {({ TransitionProps, placement }) => (
                    <Grow
                        {...TransitionProps}
                        style={{
                            transformOrigin:
                                placement === 'bottom' ? 'center top' : 'center bottom',
                        }}
                    >
                        <Paper>
                            <ClickAwayListener onClickAway={handleClose}>
                                <MenuList id="split-button-menu" autoFocusItem>
                                    {options.map((option, index) => (
                                        <MenuItem
                                            key={option}
                                            disabled={index === 2}
                                            selected={index === selectedIndex}
                                            onClick={(event) => handleMenuItemClick(event, index)}
                                        >
                                            {option}
                                        </MenuItem>
                                    ))}
                                </MenuList>
                            </ClickAwayListener>
                        </Paper>
                    </Grow>
                )}
            </Popper>
        </Stack>
    );
}

interface EventData {
    id: string;
    script: string | null;
    date: string | null;
    scriptError: string | null;
    dateError: string | null;
}

export default function Workbench() {
    const [events, setEvents] = React.useState<EventData[]>([]);

    const handleRun = () => {
        let allValid = true;
        const newEvents = events.map((event) => {
            const scriptError = !event.script ? 'Script is required' : null;
            const dateError = !event.date ? 'Date is required' : null;
            if (scriptError || dateError) {
                allValid = false;
            }
            return { ...event, scriptError, dateError };
        });
        setEvents(newEvents);

        if (allValid) {
            events.forEach((event) => {
                console.log('Run', event);
            });
        }
    };

    const handleStop = () => {
        console.log('Stop');
    };

    const handlePause = () => {
        console.log('Pause');
    };

    const handleRemoveEvent = (id: string) => {
        const newEvents = events.filter((event) => event.id !== id);
        setEvents(newEvents);
    };

    const handleAddEvent = () => {
        const newEvent = generateUniqueId();
        setEvents([...events, { id: newEvent, script: null, date: null, scriptError: null, dateError: null }]);
    };

    const handleUpdateEventScript = (id: string, script: string) => {
        const newEvents = events.map((event) => {
            if (event.id === id) {
                return { ...event, script, scriptError: null };
            }
            return event;
        });
        setEvents(newEvents);
    }

    const handleUpdateEventDate = (id: string, date: Dayjs | null) => {
        const newEvents = events.map((event) => {
            if (event.id === id) {
                return { ...event, date: date?.format() || null, dateError: null };
            }
            return event;
        });
        setEvents(newEvents);
    }

    return (
        <Box sx={{ padding: 2, backgroundColor: 'white', borderRadius: 1 }}>
            <WorkbenchMenu
                onRun={handleRun}
                onStop={handleStop}
                onPause={handlePause}
                onAddEvent={handleAddEvent}
            />
            <Stack spacing={2} p={2}>
                {events.map((event, index) => (
                    <Box key={index}>
                        <ScriptingEvent
                            eventId={event.id}
                            onRemove={() => handleRemoveEvent(event.id)}
                            onScriptChange={(value) => handleUpdateEventScript(event.id, value)}
                            onDateChange={(value) => handleUpdateEventDate(event.id, value)}
                            initialScript={null}
                            initialDate={null}
                            scriptError={event.scriptError}
                            dateError={event.dateError}
                        />
                    </Box>
                ))}
            </Stack>
        </Box>
    );
}