import React from 'react';
import './App.css';
import { BrowserRouter, Route, Routes } from 'react-router-dom';
import EditScript from './pages/EditScript';

function App() {
  return (
    <div className="App">
    
        <BrowserRouter>
          <Routes>
            <Route path="/" element={<EditScript />} />
          </Routes>
        </BrowserRouter>
    </div>
  );
}

export default App;
