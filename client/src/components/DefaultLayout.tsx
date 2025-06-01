import { Outlet } from 'react-router-dom';
import { MenuBar } from './menu/MenuBar';
import { Box } from '@mui/material';
import Page from './generic/Page';

export default function DefaultLayout() {
  return (
    <Box className="flex h-screen w-full bg-gray-50">
      <MenuBar />
      <Box className="flex-1 overflow-auto">
        <Page>
          <Outlet />
        </Page>
      </Box>
    </Box>
  );
}