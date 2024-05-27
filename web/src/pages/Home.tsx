import { createTheme, ThemeProvider } from "@mui/material/styles";
import { menuThemeOptions } from '../css/theme';
import MainLayout from "../components/MainLayout";
import { Box } from "@mui/material";

export default function Home() {
    const menuTheme = createTheme(menuThemeOptions);

    return (
        <ThemeProvider theme={menuTheme}>
            <MainLayout children={
                <Box></Box>
            } />
        </ThemeProvider>
    );
}
