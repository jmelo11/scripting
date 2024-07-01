import { IconButton, Stack } from '@mui/material';
import AppBar from '@mui/material/AppBar';
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import MenuIcon from '@mui/icons-material/Menu';

interface NavbarProps {
    onMenuClick?: () => void;
}

export default function Navbar(props: NavbarProps) {
    return (
        <AppBar position="static" sx={{
            height: '2rem',
            boxShadow: 0,
            justifyContent: 'center'
        }}>
            <Toolbar>
                <Stack sx={{
                    justifyContent: 'space-between',
                    width: '100%',
                    alignItems: 'center'
                }} direction={'row'}>
                    <IconButton
                        size="small"
                        edge="start"
                        color="inherit"
                        aria-label="menu"
                        sx={{ mr: 2 }}
                        onClick={props.onMenuClick}
                    >
                        <MenuIcon />
                    </IconButton>
                    {/* add current date and time */}
                    <Typography variant="body2" component="p" align="right">
                        {new Date().toLocaleString()}
                    </Typography>
                </Stack>
            </Toolbar>
        </AppBar>
    );
}
