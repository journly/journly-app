import { Outlet } from 'react-router-dom';
import { Affix, Button, Flex } from '@mantine/core';
import { useAllTrips } from '@/providers/AllTripsProvider';

export const Toolbar = () => {
  const { deleteAllTrips } = useAllTrips();

  const handleDeleteAllReplicacheData = async () => {
    await deleteAllTrips();
  };
  return (
    <>
      <Outlet />
      <Affix>
        <Flex gap="xs" align="center" w="100vw" mb={10} justify="center">
          <Button onClick={handleDeleteAllReplicacheData}>Delete trips</Button>
        </Flex>
      </Affix>
    </>
  );
};
