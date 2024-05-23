import { DatePicker } from '@mui/x-date-pickers/DatePicker';
import { LocalizationProvider } from '@mui/x-date-pickers';
import { AdapterDayjs } from '@mui/x-date-pickers/AdapterDayjs';
import { Box, FormControl, FormControlLabel, FormGroup, Stack, Switch, TextField, FormHelperText } from '@mui/material';
import React, { useState } from 'react';
import { Dayjs } from 'dayjs';

interface DateInputProps {
    helperText?: string;
    error?: boolean;
    onSwitchChange: (isChecked: boolean) => void;
    onDateChange: (value: Dayjs | null) => void;
}

export default function DateInput(props: DateInputProps) {
    const [checked, setChecked] = useState<boolean>(false);

    const handleSwitchChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        setChecked(event.target.checked);
        props.onSwitchChange(event.target.checked);
    };

    const handleDateChange = (value: Dayjs | null) => {
        props.onDateChange(value);
    }

    return (
        <Stack spacing={2} sx={{ height: '100%', width: '100%', justifyContent: 'space-between' }}>
            <FormGroup sx={{ alignItems: 'flex-end' }}>
                <FormControlLabel control={<Switch onChange={handleSwitchChange} />} label="Use Form" />
            </FormGroup>
            <FormControl error={props.error}>
                {checked ? (
                    <TextField
                        multiline
                        fullWidth
                    />
                ) : (
                    <LocalizationProvider dateAdapter={AdapterDayjs}>
                        <DatePicker
                            label="Date"
                            onChange={handleDateChange}
                        />
                    </LocalizationProvider>
                )}
            </FormControl>
        </Stack>
    );
}
