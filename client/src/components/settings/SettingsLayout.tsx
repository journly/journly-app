import { Outlet } from 'react-router-dom';
import { Box } from '@mui/material';
import SettingsBar from './SettingsBar';

export default function SettingsLayout() {
  return (
    <Box className="flex h-screen w-full bg-gray-50">
      <SettingsBar />
      <Box className="flex-1 overflow-auto">
        <Outlet />
      </Box>
    </Box>
  );
}
