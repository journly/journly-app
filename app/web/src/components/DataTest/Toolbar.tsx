import { useState } from 'react';
import { IconSettings } from '@tabler/icons-react';
import { Outlet } from 'react-router-dom';
import { Affix, Button, Menu, Text } from '@mantine/core';
import { useAllTrips } from '@/providers/AllTripsProvider';

export const Toolbar = () => {
  const [openToolbar, setOpenToolbar] = useState(false);
  const { deleteAllTrips } = useAllTrips();

  const handleDeleteAllReplicacheData = async () => {
    await deleteAllTrips();
  };
  return (
    <>
      <Outlet />
      <Affix position={{ right: 20, bottom: 20 }}>
        <Menu>
          <Menu.Target>
            <Button variant="default" leftSection={<IconSettings size={16} />}>
              <Text>Dev tools</Text>
            </Button>
          </Menu.Target>
          <Menu.Dropdown>
            <Menu.Item onClick={handleDeleteAllReplicacheData}>Delete trips</Menu.Item>
          </Menu.Dropdown>
        </Menu>
      </Affix>
    </>
  );
};
