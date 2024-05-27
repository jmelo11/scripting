import React, { useState } from 'react';
import { Box, Button, Modal, TextField, Typography, Alert } from '@mui/material';
import { v4 as uuidv4 } from 'uuid';

interface NewScriptModalProps {
    open: boolean;
    onClose: () => void;
    onAddScript: (scriptName: string) => void;
    existingScriptNames: string[];
}

const NewScriptModal: React.FC<NewScriptModalProps> = ({ open, onClose, onAddScript, existingScriptNames }) => {
    const [scriptName, setScriptName] = useState('');
    const [alert, setAlert] = useState<string | null>(null);

    const handleAddScript = () => {
        if (scriptName.trim() === '') {
            setAlert('Script name is required.');
            return;
        }
        if (existingScriptNames.includes(scriptName.trim())) {
            setAlert('Script name must be unique.');
            return;
        }
        onAddScript(scriptName.trim());
        setScriptName('');
        setAlert(null);
        onClose();
    };

    const handleGenerateUUID = () => {
        setScriptName(uuidv4());
        setAlert(null);
    };

    return (
        <Modal open={open} onClose={onClose}>
            <Box sx={{
                position: 'absolute',
                top: '50%',
                left: '50%',
                transform: 'translate(-50%, -50%)',
                width: 400,
                bgcolor: 'background.paper',
                boxShadow: 24,
                p: 4,
            }}>
                <Typography variant="h5">
                    Add New Script
                </Typography>
                {alert && <Alert severity="error" sx={{ mt: 2 }}>{alert}</Alert>}
                <TextField
                    fullWidth
                    label="Script Name"
                    value={scriptName}
                    onChange={(e) => setScriptName(e.target.value)}
                    sx={{ mt: 2 }}
                />
                <Box sx={{ display: 'flex', justifyContent: 'flex-end', mt: 2 }}>
                    <Button onClick={handleGenerateUUID} variant="outlined" sx={{ mr: 2 }}>
                        Generate UUID
                    </Button>
                    <Button onClick={handleAddScript} variant="contained">
                        Add
                    </Button>
                </Box>
            </Box>
        </Modal>
    );
};

export default NewScriptModal;
