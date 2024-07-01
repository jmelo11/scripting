import { DatePicker } from '@mui/x-date-pickers/DatePicker';
import { LocalizationProvider } from '@mui/x-date-pickers';
import { AdapterDayjs } from '@mui/x-date-pickers/AdapterDayjs';
import { FormControl, Stack } from '@mui/material';
import { Dayjs } from 'dayjs';

interface DateInputProps {
    helperText?: string;
    error?: boolean;
    onDateChange: (value: Dayjs | null) => void;
}

export default function DateInput(props: DateInputProps) {
    const handleDateChange = (value: Dayjs | null) => {
        props.onDateChange(value);
    }

    return (
        <FormControl error={props.error}>
            <LocalizationProvider dateAdapter={AdapterDayjs}>
                <Stack spacing={1}>
                    <DatePicker
                        label="Event Date"
                        onChange={handleDateChange}
                        format='YYYY/MM/DD'
                    />
                </Stack>
            </LocalizationProvider>
        </FormControl >
    );
}
