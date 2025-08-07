import { IconPlus } from '@tabler/icons-react';
import { useSubscribe } from 'replicache-react';
import {
  Box,
  Button,
  Divider,
  Flex,
  Group,
  Text,
  Title,
  Tooltip,
  UnstyledButton,
} from '@mantine/core';
import { useDisclosure } from '@mantine/hooks';
import { ItineraryItemModal } from '@/components/Modals/ItineraryItemModal';
import { getItineraryItemsByTrip } from '@/models/itineraryItem';
import { getTrip, Trip } from '@/models/trip';
import { useReplicache } from '@/providers/ReplicacheProvider';
import classes from './Tab.module.css';

export default function ItineraryTab({ trip }: { trip: Trip }) {
  const { rep } = useReplicache();
  const [opened, { open, close }] = useDisclosure(false);

  const itineraryItems = useSubscribe(rep, (tx) => getItineraryItemsByTrip(tx, trip.id), {
    default: [],
    dependencies: [trip.id],
  });

  return (
    <>
      <ItineraryItemModal
        opened={opened}
        onClose={close}
        title="New Itinerary Item"
        maxDate={trip.endDate ? new Date(trip.endDate) : undefined}
        minDate={trip.startDate ? new Date(trip.startDate) : undefined}
      />
      <Flex wrap="wrap" gap={20} justify="space-between" maw={1150} w="100%" mx={20} mb={20}>
        <Box
          w={{ base: '100%', sm: '100%', md: '100%', lg: '65%' }}
          h="fit-content"
          className={classes.sectionContainer}
        >
          <Group justify="space-between">
            <Title order={6}>Daily Itinerary</Title>
            <Tooltip label="New itinerary item">
              <UnstyledButton className={classes.addButton} onClick={open}>
                <IconPlus size={16} />
              </UnstyledButton>
            </Tooltip>
          </Group>
          <Divider my={10} />
        </Box>
      </Flex>
    </>
  );
}
