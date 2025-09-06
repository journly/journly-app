import dayjs from 'dayjs';
import { useState } from 'react';
import { IconClock, IconMapPin, IconPlus, IconUsers } from '@tabler/icons-react';
import { useSubscribe } from 'replicache-react';
import {
  Alert,
  Avatar,
  BackgroundImage,
  Badge,
  Box,
  Button,
  Center,
  Flex,
  Grid,
  Group,
  Image,
  Select,
  Text,
  Title,
} from '@mantine/core';
import { useDisclosure } from '@mantine/hooks';
import { CreateTripModal } from '@/components/Modals/CreateTripModal';
import { getTrip, Trip } from '@/models/trip';
import { useAllTrips } from '@/providers/AllTripsProvider';
import { useReplicache } from '@/providers/ReplicacheProvider';
import classes from './Home.module.css';

export default function HomePage() {
  const [selectedTrip, setSelectedTrip] = useState<string | null>('Default');

  const { rep } = useReplicache();
  const { trips } = useAllTrips();

  const tripOptions = ['Default', ...trips.map((trip) => trip.name)];

  const trip = useSubscribe(
    rep,
    (tx) => getTrip(tx, trips.find((trip) => trip.name === selectedTrip)?.id ?? ''),
    {
      default: null,
      dependencies: [selectedTrip],
    }
  );

  return (
    <>
      <Center w="100%">
        <Grid maw={1150} w="100%" mx={20} mt="xl" justify="center">
          <Grid.Col span={12}>
            <Title order={4} mb={5}>
              Current View
            </Title>
            <Select
              data={tripOptions}
              placeholder="Select a trip"
              w={170}
              value={selectedTrip}
              onChange={setSelectedTrip}
              allowDeselect={false}
            />
          </Grid.Col>
          {trip ? <TripView trip={trip} /> : <DefaultView />}
        </Grid>
      </Center>
    </>
  );
}

function TripView({ trip }: { trip: Trip }) {
  const tripDateInfo = () => {
    const daysSinceEnd = dayjs(trip.endDate).diff(dayjs(), 'day');

    const daysUntil = dayjs(trip.startDate).diff(dayjs(), 'day');
    if (daysUntil > 0) {
      return `${daysUntil} day${daysUntil === 1 ? '' : 's'} until`;
    } else if (daysUntil === 0) {
      return 'Today';
    } else if (daysSinceEnd < 0) {
      return `Ended ${Math.abs(daysSinceEnd)} day${Math.abs(daysSinceEnd) === 1 ? '' : 's'} ago`;
    } else {
      return `${daysSinceEnd + 1} day${daysSinceEnd + 1 === 1 ? '' : 's'} remaining`;
    }
  };

  return (
    <>
      <Grid.Col span={8}>
        <Flex direction="column" gap={10} className={classes.tripViewSection} pb={10}>
          <BackgroundImage
            src={trip.coverImage ?? ''}
            w="100%"
            h={150}
            className={classes.coverImage}
          >
            <Title order={3} ml={15} mb={15}>
              {trip.name}
            </Title>
          </BackgroundImage>
          <Group ml={15} gap={10}>
            <Badge
              color="blue"
              radius="sm"
              variant="light"
              size="md"
              leftSection={<IconClock size={16} />}
            >
              {tripDateInfo()}
            </Badge>
            <Badge
              color="gray"
              radius="sm"
              variant="light"
              size="md"
              leftSection={<IconUsers size={16} />}
            >
              3 Collaborators
            </Badge>
          </Group>
          <Box ml={15} mr={15}>
            <Text>{trip.description}</Text>
          </Box>
          <Group justify="space-between" mr={15} ml={15}>
            <Button w="80%">Continue Planning</Button>
            <Button variant="default" w="14%">
              Invite
            </Button>
          </Group>
        </Flex>
      </Grid.Col>
      <Grid.Col span={4}>
        <Flex direction="column" gap={10} className={classes.section}>
          <Title order={4}>{trip.name}</Title>
          <Text>{trip.description}</Text>
        </Flex>
      </Grid.Col>
    </>
  );
}

function DefaultView() {
  const [createTripModalOpen, { open: openCreateTripModal, close: closeCreateTripModal }] =
    useDisclosure(false);
  return (
    <>
      <Grid.Col span={12}>
        <Center w="100%" className={classes.section} py="xl">
          <Flex direction="column" align="center" gap={20}>
            <Avatar color="blue" size={60}>
              <IconMapPin size={30} />
            </Avatar>
            <Title order={4}>Plan Your Next Trip</Title>
            <Text c="dimmed" size="sm" w="70%" ta="center">
              Create a new trip, add destinations, plan your itinerary, and invite travel companions
              to join your adventure.
            </Text>
            <Button
              leftSection={<IconPlus size={16} />}
              onClick={() => {
                openCreateTripModal();
              }}
            >
              Plan a New Trip
            </Button>
          </Flex>
        </Center>
      </Grid.Col>
      <CreateTripModal open={createTripModalOpen} onClose={closeCreateTripModal} />
    </>
  );
}
