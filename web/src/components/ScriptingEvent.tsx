import { Box, Divider, Grid, IconButton, Typography, TextField } from '@mui/material';
import { useState } from 'react';
import DateInput from './DateInput';
import ScriptingArea from './ScriptingArea';
import CloseIcon from '@mui/icons-material/Close';
import { Dayjs } from 'dayjs';

interface ScriptingEventMenuProps {
    eventId: string;
    onRemove: () => void;
}

function ScriptingEventMenu(props: ScriptingEventMenuProps) {

    const handleRemove = () => {
        props.onRemove();
    };

    return (
        <Box sx={{
            color: 'gray',
            paddingLeft: 1,
            paddingRight: 1,
            display: 'flex',
            justifyContent: 'space-between',
            backgroundColor: 'rgba(0, 0, 0, 0.1)',
        }}>
            <Typography variant='body1'>ID {props.eventId}</Typography>
            <IconButton size='small' onClick={handleRemove}>
                <CloseIcon fontSize='inherit' />
            </IconButton>
        </Box>
    )
}

export interface ScriptingEventProps {
    eventId: string;
    initialScript: string | null;
    initialDate: Dayjs | null;
    onRemove: () => void;
    onScriptChange: (value: string) => void;
    onDateChange: (value: Dayjs | null) => void;
    scriptError: string | null;
    dateError: string | null;
}

export function ScriptingEvent(props: ScriptingEventProps) {

    const [multiEvent, setMultiEvent] = useState<boolean>(false);
    const [script, setScript] = useState<string | null>(props.initialScript);
    const [date, setDate] = useState<Dayjs | null>(props.initialDate ? props.initialDate : null);

    const handleSwitchChange = (isChecked: boolean) => {
        setMultiEvent(isChecked);
    };

    const handleScriptChange = (value: string) => {
        setScript(value);
        props.onScriptChange(value);
    }

    const handleDateChange = (value: Dayjs | null) => {
        setDate(value);
        props.onDateChange(value);
    }

    return (
        <Box sx={{
            border: '1px solid rgba(0, 0, 0, 0.25)',
            borderRadius: 1,
            marginBottom: 2,
        }}>
            <ScriptingEventMenu
                onRemove={props.onRemove}
                eventId={props.eventId} />
            <Divider />
            <Box sx={{
                backgroundColor: 'white',
                color: 'black',
                padding: 1,
            }}>
                <Grid container spacing={2}>
                    <Grid item xs={3}>
                        <DateInput onSwitchChange={handleSwitchChange} onDateChange={handleDateChange} />
                    </Grid>
                    <Grid item xs={9}>
                        <ScriptingArea onScriptChange={handleScriptChange} />
                    </Grid>
                </Grid>
                {props.dateError && <Typography color="error">{props.dateError}</Typography>}
                {props.scriptError && <Typography color="error">{props.scriptError}</Typography>}
            </Box>
        </Box >
    )
}