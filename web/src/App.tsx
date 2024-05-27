// App.tsx
import './css/App.css';
import { BrowserRouter, Route, Routes } from 'react-router-dom';
import ScriptSettingsPage from './pages/ScriptSettingsPage';
import Home from './pages/Home';
import EventsPage from './pages/EventsPage';
import { ScriptsProvider } from './contexts/ScriptsContext';
import { DrawerProvider } from './contexts/DrawerContext';

function App() {
  return (
    <div className="App">
      <ScriptsProvider>
        <DrawerProvider>
          <BrowserRouter>
            <Routes>
              <Route path="/" element={<Home />} />
              <Route path="/script-settings" element={<ScriptSettingsPage />} />
              <Route path="/events" element={<EventsPage />} />  // Add the route for the events page
            </Routes>
          </BrowserRouter>
        </DrawerProvider>
      </ScriptsProvider>
    </div>
  );
}

export default App;
