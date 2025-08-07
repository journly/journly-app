import { Outlet } from 'react-router-dom';
import { AppShell } from '@mantine/core';
import { useMediaQuery } from '@mantine/hooks';
import { Navbar } from './Navbar/Navbar';

export const DefaultLayout = () => {
  const isMobile = useMediaQuery('(max-width: 768px)');

  return (
    <AppShell navbar={{ width: 250, breakpoint: 'sm', collapsed: { mobile: isMobile } }}>
      <AppShell.Navbar>
        <Navbar />
      </AppShell.Navbar>
      <AppShell.Main>
        <Outlet />
      </AppShell.Main>
    </AppShell>
  );
};
