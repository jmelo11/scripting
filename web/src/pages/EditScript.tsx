import { Box, Stack } from '@mui/material'
import Workbench from '../components/Workbench'
import Navbar from '../components/Navbar'
export default function EditScript() {
    return (
        <Box>
            <Navbar />
            <Stack spacing={2} width={'100%'}>
                <Workbench />
            </Stack >
        </Box>
    )
}
