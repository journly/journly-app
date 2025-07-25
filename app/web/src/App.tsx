import '@mantine/core/styles.css';

import { StrictMode } from 'react';
import { BrowserRouter } from 'react-router-dom';
import { MantineProvider } from '@mantine/core';
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
              <BrowserRouter>
                <Router />
              </BrowserRouter>
            </UserProvider>
          </ReplicacheProvider>
        </AuthProvider>
      </MantineProvider>
    </StrictMode>
  );
}
