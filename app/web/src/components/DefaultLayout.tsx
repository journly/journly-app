import { Outlet } from 'react-router-dom';
import { AppShell } from '@mantine/core';
import { Navbar } from './Navbar/Navbar';

export const DefaultLayout = () => {
  return (
    <AppShell navbar={{ width: 250, breakpoint: 'sm' }}>
      <AppShell.Navbar>
        <Navbar />
      </AppShell.Navbar>
      <AppShell.Main>
        <Outlet />
      </AppShell.Main>
    </AppShell>
  );
};
