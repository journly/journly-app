import {
  IconBook,
  IconCompass,
  IconHome,
  IconMap,
  IconPlus,
  IconSearch,
  IconTimeline,
} from '@tabler/icons-react';
import { useLocation, useNavigate } from 'react-router-dom';
import {
  ActionIcon,
  AppShell,
  Avatar,
  Box,
  Code,
  Flex,
  Group,
  Image,
  ScrollArea,
  Text,
  TextInput,
  Title,
  Tooltip,
  UnstyledButton,
} from '@mantine/core';
import { useDisclosure } from '@mantine/hooks';
import { useAllTrips } from '@/providers/AllTripsProvider';
import { useUser } from '@/providers/UserProvider';
import { formatTripDatesSimple } from '@/utils/dates';
import logo from '../../favicon.png';
import { CreateTripModal } from '../Modals/CreateTripModal';
import classes from './Navbar.module.css';

const links = [
  { icon: IconHome, label: 'Dashboard', path: '/' },
  { icon: IconMap, label: 'Map', path: '/map' },
  { icon: IconBook, label: 'Journal', path: '/journal' },
  { icon: IconCompass, label: 'Explore', path: '/explore' },
  { icon: IconTimeline, label: 'Stats', path: '/stats' },
];

export const Navbar = () => {
  const navigate = useNavigate();
  const location = useLocation();
  const { user } = useUser();
  const [opened, { open, close }] = useDisclosure(false);
  const { trips } = useAllTrips();

  const mainLinks = links.map((link) => {
    const isActive =
      location.pathname === link.path ||
      (location.pathname.startsWith(link.path) && link.path !== '/');
    return (
      <UnstyledButton
        key={link.label}
        className={`${classes.mainLink} ${isActive ? classes.active : classes.mainLinkHover}`}
        onClick={() => navigate(link.path)}
      >
        <div className={`${classes.mainLinkInner} ${isActive ? classes.active : ''}`}>
          <link.icon size={22} className={classes.mainLinkIcon} stroke={2} />
          <span>{link.label}</span>
        </div>
      </UnstyledButton>
    );
  });

  const tripLinks = trips.map((trip) => {
    const isActive = location.pathname.startsWith(`/trip/${trip.id}`);
    return (
      <UnstyledButton
        onClick={() => {
          navigate(`/trip/${trip.id}`);
        }}
        key={trip.id}
        className={`${classes.tripLink} ${isActive ? classes.active : classes.tripLinkHover}`}
      >
        <Flex align="center" gap={9}>
          <Box component="span" mr={9} fz={16} className={isActive ? classes.active : ''}>
            {trip.name.charAt(0)}
          </Box>{' '}
          <Box>
            <Text size="sm" fw={500} className={isActive ? classes.active : ''}>
              {trip.name}
            </Text>
            {trip.startDate && trip.endDate && (
              <Text size="xs" c="dimmed">
                {formatTripDatesSimple([trip.startDate, trip.endDate])}
              </Text>
            )}
          </Box>
        </Flex>
      </UnstyledButton>
    );
  });

  return (
    <nav className={classes.navbar}>
      <AppShell.Section className={classes.section}>
        <Flex className={classes.logoContainer} justify="start" mx="md" align="center" gap={8}>
          <Image src={logo} alt="Journly" w={45} h={45} />
          <Title order={3} className={classes.title}>
            Journly
          </Title>
        </Flex>
      </AppShell.Section>
      <AppShell.Section className={classes.section}>
        <div className={classes.mainLinks}>{mainLinks}</div>
      </AppShell.Section>

      <TextInput
        placeholder="Search"
        size="xs"
        leftSection={<IconSearch size={12} stroke={1.5} />}
        rightSectionWidth={70}
        rightSection={<Code className={classes.searchCode}>Ctrl + K</Code>}
        styles={{ section: { pointerEvents: 'none' } }}
        mb="sm"
        pt={10}
      />
      <AppShell.Section className={classes.subSection}>
        <Group className={classes.tripsHeader} justify="space-between">
          <Text size="xs" fw={500} c="dimmed">
            Trips
          </Text>
          <Tooltip label="Create a new trip" withArrow position="right">
            <ActionIcon variant="default" size={18} onClick={open}>
              <IconPlus size={12} stroke={1.5} />
            </ActionIcon>
          </Tooltip>
        </Group>
      </AppShell.Section>
      <AppShell.Section
        grow
        component={ScrollArea}
        scrollHideDelay={2}
        scrollbarSize={5}
        className={classes.subSection}
      >
        <div className={classes.trips}>{tripLinks}</div>
      </AppShell.Section>
      <AppShell.Section className={classes.userSection}>
        <Flex align="center" justify="flex-start" gap={10} className={classes.user}>
          <Avatar
            src={user?.avatar ?? null}
            name={user?.username ?? 'Guest'}
            alt="User avatar"
            size="md"
          />
          <Box>
            <Text size="sm">{user?.username ?? 'Guest'}</Text>
            <Text size="xs" c="dimmed">
              {user?.email ?? 'No email'}
            </Text>
          </Box>
        </Flex>
      </AppShell.Section>
      <CreateTripModal open={opened} onClose={close} />
    </nav>
  );
};
