import { createTheme, ThemeProvider } from "@mui/material/styles";
import { menuThemeOptions } from '../css/theme';
import MainLayout from "../components/MainLayout";
import { Stack, Typography } from "@mui/material";

export default function Home() {
    const menuTheme = createTheme(menuThemeOptions);

    return (
        <ThemeProvider theme={menuTheme}>
            <MainLayout>
                <Stack spacing={2} sx={{
                    display: 'flex',
                    flexDirection: 'column',
                    alignItems: 'center',
                    justifyContent: 'center',
                    height: '100%',
                }}>
                    <Typography variant="h1" sx={{
                        fontFamily: 'Cairo Play',
                        fontWeight: 'bold',
                    }}>DerivaLogic</Typography>
                    <Typography variant="h5" sx={{
                        fontFamily: 'Roboto',
                        fontWeight: 'light',
                    }}>Financial derivatives redefined.</Typography>
                </Stack>
            </MainLayout>
        </ThemeProvider>
    );
}
