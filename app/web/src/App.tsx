import '@mantine/core/styles.css';
import '@mantine/dates/styles.css';

import { StrictMode } from 'react';
import { BrowserRouter } from 'react-router-dom';
import { MantineProvider } from '@mantine/core';
import { AllTripsProvider } from './providers/AllTripsProvider';
import { AuthProvider } from './providers/AuthProvider';
import { ReplicacheProvider } from './providers/ReplicacheProvider';
import { UserProvider } from './providers/UserProvider';
import { Router } from './Router';
import { theme } from './theme';

export default function App() {
  return (
    <StrictMode>
      <MantineProvider theme={theme}>
        <AuthProvider>
          <ReplicacheProvider>
            <UserProvider>
              <AllTripsProvider>
                <BrowserRouter>
                  <Router />
                </BrowserRouter>
              </AllTripsProvider>
            </UserProvider>
          </ReplicacheProvider>
        </AuthProvider>
      </MantineProvider>
    </StrictMode>
  );
}
