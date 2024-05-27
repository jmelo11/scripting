import React, { useState, useEffect } from 'react';
import { Box, Button, Divider, IconButton, Modal, Stack, Typography, Alert } from '@mui/material';
import { DatePicker } from '@mui/x-date-pickers/DatePicker';
import { v4 as uuidv4 } from 'uuid';
import { LocalizationProvider } from '@mui/x-date-pickers/LocalizationProvider';
import { AdapterDayjs } from '@mui/x-date-pickers/AdapterDayjs';
import { DataGrid, GridColDef, GridRowId, GridRowModel } from '@mui/x-data-grid';
import dayjs, { Dayjs } from 'dayjs';
import DeleteOutlineOutlinedIcon from '@mui/icons-material/DeleteOutlineOutlined';
import AddIcon from '@mui/icons-material/Add';

interface SettingsModalProps {
    open: boolean;
    onClose: () => void;
    script: any;
    onSave: (updatedScript: any) => void;
}

interface ScriptRow {
    id: string;
    key: string;
    value: string;
}

const SettingsModal: React.FC<SettingsModalProps> = ({ open, onClose, script, onSave }) => {
    const [referenceDate, setReferenceDate] = useState<Dayjs | null>(dayjs(script.referenceDate));
    const [rows, setRows] = useState<ScriptRow[]>(script.rows || []);
    const [alert, setAlert] = useState<string | null>(null);

    useEffect(() => {
        if (open) {
            setReferenceDate(dayjs(script.referenceDate));
            setRows(script.rows || []);
            setAlert(null);  // Reset alert when modal is opened
        }
    }, [open, script]);

    const handleAddRow = () => {
        setRows([...rows, { id: uuidv4(), key: '', value: '' }]);
    };

    const handleDeleteRow = (id: GridRowId) => {
        setRows(rows.filter(row => row.id !== id));
    };

    const handleSave = () => {
        const validRows = rows.filter(row => row.key.trim() !== '' && row.value.trim() !== '');
        if (validRows.length !== rows.length) {
            setAlert("Please fill in all key and value fields.");
            return;
        }
        setAlert(null);
        onSave({ ...script, referenceDate, rows: validRows });
        onClose();
    };

    const processRowUpdate = (newRow: GridRowModel, oldRow: GridRowModel): ScriptRow => {
        const updatedRow = { ...oldRow, ...newRow } as ScriptRow;
        const updatedRows = rows.map(row => (row.id === updatedRow.id ? updatedRow : row));
        setRows(updatedRows);
        return updatedRow;
    };

    const handleProcessRowUpdateError = (error: any) => {
        console.error("Error updating row: ", error);
        setAlert("Error updating row. Please try again.");
    };

    const columns: GridColDef[] = [
        { field: 'key', headerName: 'Key', width: 150, editable: true },
        { field: 'value', headerName: 'Value', width: 150, editable: true },
        {
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

    return (
        <Modal open={open} onClose={onClose}>
            <Box sx={{
                position: 'absolute',
                top: '50%',
                left: '50%',
                transform: 'translate(-50%, -50%)',
                bgcolor: 'background.paper',
                boxShadow: 24,
                p: 4,
            }}>
                <Stack spacing={2}>
                    <Typography variant="h5" component="h2">
                        Script Settings
                    </Typography>
                    <Typography variant="h6" component="p">
                        Evaluation settings
                    </Typography>
                    <Typography variant="body2" component="p">
                        This configuration will be used to evaluate the script.
                    </Typography>
                    <Divider />
                    <LocalizationProvider dateAdapter={AdapterDayjs}>
                        <DatePicker
                            label="Reference Date"
                            value={referenceDate}
                            onChange={(newValue) => setReferenceDate(newValue)}
                        />
                    </LocalizationProvider>
                    <Typography variant="h6" component="p">
                        Macros
                    </Typography>
                    <Typography variant="body2" component="p">
                        Macros are key-value pairs that can be used to replace placeholders in the script.
                    </Typography>
                    {alert && <Alert severity="error">{alert}</Alert>}
                    <Divider />
                    <Stack spacing={2} sx={{ height: '300px' }}>
                        <Box sx={{
                            display: 'flex',
                            justifyContent: 'flex-end',
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
                        />
                    </Stack>

                    <Box sx={{ display: 'flex', justifyContent: 'flex-end', mt: 2 }}>
                        <Button onClick={handleSave} variant="contained">
                            Save
                        </Button>
                    </Box>
                </Stack>
            </Box>
        </Modal>
    );
};

export default SettingsModal;
