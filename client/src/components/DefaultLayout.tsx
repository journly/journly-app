import { Outlet } from 'react-router-dom';
import { MenuBar } from './menu/MenuBar';
import { Box } from '@mui/material';
import { MenuProvider } from '../providers/MenuProvider';

export default function DefaultLayout() {
  return (
    <MenuProvider>
      <Box className="flex h-screen w-full bg-gray-50">
        <MenuBar />
        <Box className="flex-1 overflow-auto">
          <Outlet />
        </Box>
      </Box>
    </MenuProvider>
  );
}
