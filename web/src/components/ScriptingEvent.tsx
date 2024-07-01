import { Box, Divider, Grid, Typography } from '@mui/material';
import { useState } from 'react';
import DateInput from './DateInput';
import ScriptingArea from './ScriptingArea';
import { Dayjs } from 'dayjs';

export interface ScriptingEventProps {
    initialScript: string | null;
    initialDate: Dayjs | null;
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
            <Divider />
            <Box sx={{
                backgroundColor: 'white',
                color: 'black',
                padding: 1,
            }}>
                <Grid container spacing={1}>
                    <Grid item xs={3}>
                        <DateInput onDateChange={handleDateChange} />
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
