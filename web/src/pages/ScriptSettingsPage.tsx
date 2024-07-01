import React, { useState, useEffect } from 'react';
import { Box, Button, Divider, IconButton, Stack, Typography, Alert, Paper } from '@mui/material';
import { DatePicker } from '@mui/x-date-pickers/DatePicker';
import { v4 as uuidv4 } from 'uuid';
import { LocalizationProvider } from '@mui/x-date-pickers/LocalizationProvider';
import { AdapterDayjs } from '@mui/x-date-pickers/AdapterDayjs';
import { DataGrid, GridColDef, GridRowId, GridRowModel } from '@mui/x-data-grid';
import dayjs, { Dayjs } from 'dayjs';
import DeleteOutlineOutlinedIcon from '@mui/icons-material/DeleteOutlineOutlined';
import AddIcon from '@mui/icons-material/Add';
import { useLocation, useNavigate } from 'react-router-dom';
import MainLayout from '../components/MainLayout';
import { createTheme, ThemeProvider } from "@mui/material/styles";
import { menuThemeOptions } from '../css/theme';
import { useScripts } from '../contexts/ScriptsContext';
import { useDrawer } from '../contexts/DrawerContext';

interface ScriptRow {
    id: string;
    key: string;
    value: string;
}

const ScriptSettingsPage: React.FC = () => {
    const location = useLocation();
    const { updateScript } = useScripts();
    const { toggleDrawer } = useDrawer();
    const { script = { id: '', referenceDate: new Date(), rows: [] } } = location.state || {};

    const [referenceDate, setReferenceDate] = useState<Dayjs | null>(dayjs(script.referenceDate));
    const [rows, setRows] = useState<ScriptRow[]>(script.rows || []);
    const [alert, setAlert] = useState<string | null>(null);
    const [unsavedChanges, setUnsavedChanges] = useState(false);

    useEffect(() => {
        setReferenceDate(dayjs(script.referenceDate));
        setRows(script.rows || []);
        setAlert(null);  // Reset alert when component mounts or script changes
        setUnsavedChanges(false); // Reset unsaved changes when component mounts or script changes
    }, [script, toggleDrawer]);

    const handleAddRow = () => {
        setRows([...rows, { id: uuidv4(), key: '', value: '' }]);
        setUnsavedChanges(true);
    };

    const handleDeleteRow = (id: GridRowId) => {
        setRows(rows.filter(row => row.id !== id));
        setUnsavedChanges(true);
    };

    const handleSave = () => {
        const validRows = rows.filter(row => row.key.trim() !== '' && row.value.trim() !== '');
        if (validRows.length !== rows.length) {
            setAlert("Please fill in all key and value fields.");
            return;
        }
        setAlert(null);
        updateScript({ ...script, referenceDate: referenceDate ? referenceDate.toDate() : new Date(), rows: validRows });
        setUnsavedChanges(false);
    };

    const processRowUpdate = (newRow: GridRowModel, oldRow: GridRowModel): ScriptRow => {
        const updatedRow = { ...oldRow, ...newRow } as ScriptRow;
        const updatedRows = rows.map(row => (row.id === updatedRow.id ? updatedRow : row));
        setRows(updatedRows);
        setUnsavedChanges(true);
        return updatedRow;
    };

    const handleProcessRowUpdateError = (error: any) => {
        console.error("Error updating row: ", error);
        setAlert("Error updating row. Please try again.");
    };


    const columns: GridColDef[] = [
        { field: 'key', headerName: 'Key', width: 150, editable: true, flex: 0.4 },
        { field: 'value', headerName: 'Value', width: 150, editable: true, flex: 0.4 }, {
            field: 'actions',
            headerName: 'Actions',
            align: 'center',
            renderCell: (params) => (
                <IconButton size='small' onClick={() => handleDeleteRow(params.id)}>
                    <DeleteOutlineOutlinedIcon />
                </IconButton>
            )
        },
    ];

    const menuTheme = createTheme(menuThemeOptions);

    return (
        <ThemeProvider theme={menuTheme}>
            <MainLayout>
                <Paper>
                    <Box sx={{ p: 2 }}>
                        <Typography variant="h5" component="p" align="left">
                            Script settings
                        </Typography>
                        <Stack spacing={1}>
                            {unsavedChanges && <Alert severity="warning">You have unsaved changes!</Alert>}
                            {alert && <Alert severity="error">{alert}</Alert>}
                        </Stack>
                        <Stack spacing={2} alignItems="flex-start" pt={1}>
                            <Typography variant="h6" component="p" align="left">
                                Evaluation settings
                            </Typography>
                            <Typography variant="body2" component="p" align="left">
                                This configuration will be used to evaluate the script.
                            </Typography>
                            <Divider />
                            <LocalizationProvider dateAdapter={AdapterDayjs}>
                                <DatePicker
                                    label="Reference Date"
                                    value={referenceDate}
                                    onChange={(newValue) => {
                                        setReferenceDate(newValue);
                                        setUnsavedChanges(true);
                                    }}
                                    sx={{ width: '100%' }}
                                />
                            </LocalizationProvider>
                            <Typography variant="h6" component="p" align="left">
                                Macros
                            </Typography>
                            <Typography variant="body2" component="p" align="left">
                                Macros are key-value pairs that can be used to replace placeholders in the script.
                            </Typography>
                            <Divider />
                            <Stack spacing={2} sx={{ height: '300px', width: '100%' }}>
                                <Box sx={{
                                    display: 'flex',
                                    justifyContent: 'flex-end',
                                    width: '100%',
                                }}>
                                    <Button onClick={handleAddRow}
                                        startIcon={<AddIcon />}
                                        variant="outlined"
                                        color="primary">
                                        Add Row
                                    </Button>
                                </Box>
                                <DataGrid
                                    rows={rows}
                                    columns={columns}
                                    rowHeight={30}
                                    pageSizeOptions={[5, 10, 20]}
                                    autoHeight
                                    processRowUpdate={processRowUpdate}
                                    onProcessRowUpdateError={handleProcessRowUpdateError}
                                    sx={{ width: '100%' }}
                                />
                            </Stack>

                            <Box sx={{ display: 'flex', justifyContent: 'flex-end', mt: 2, width: '100%' }}>
                                <Button onClick={handleSave} variant="contained">
                                    Save
                                </Button>
                            </Box>
                        </Stack>
                    </Box>
                </Paper>
            </MainLayout>
        </ThemeProvider>
    );
};

export default ScriptSettingsPage;
