

import { Box, BoxProps, useTheme } from '@mui/material';

import AceEditor from "react-ace";

import "ace-builds/src-noconflict/mode-java";
import "ace-builds/src-noconflict/theme-github";

interface ScriptingAreaProps {
    onScriptChange: (value: string) => void;
}


export default function ScriptingArea(props: ScriptingAreaProps) {
    const theme = useTheme();

    const handleScriptChange = (value: string) => {
        props.onScriptChange(value);
    }

    return (

        <Box
            sx={{
                border: '1px solid rgba(0, 0, 0, 0.25)',
                borderRadius: '4px',
                padding: '10.5px 14px',
                '&:hover': {
                    borderColor: 'black',
                },
                '&:focus-within': {
                    borderColor: theme.palette.primary.main,
                },
            }}
        >
            <AceEditor
                mode="java"
                theme="github"
                editorProps={{ $blockScrolling: true }}
                maxLines={6}
                minLines={6}
                onChange={handleScriptChange}
                style={{
                    width: '100%',
                    borderRadius: '4px',
                }}
            />
        </Box>
    );
}