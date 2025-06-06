import React from 'react';
import ReactDOM from 'react-dom/client';
import { ThemeProvider } from '@mui/material/styles';
import theme from './theme';
import './index.css';
import App  from './App';
import { UserProvider } from './providers/UserProvider';
import { TripsProvider } from './providers/TripsProvider';
import { BrowserRouter } from 'react-router-dom';

const root = ReactDOM.createRoot(
  document.getElementById('root') as HTMLElement
);

root.render(
  <React.StrictMode>
    <BrowserRouter>
      <ThemeProvider theme={theme}>
          <UserProvider>
            <TripsProvider>
              <App />
            </TripsProvider>
          </UserProvider>
      </ThemeProvider>
    </BrowserRouter>
  </React.StrictMode>
)