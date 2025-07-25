import {
  IconBook,
  IconBulb,
  IconCheckbox,
  IconCompass,
  IconHome,
  IconMap,
  IconPlus,
  IconSearch,
  IconTimeline,
  IconUser,
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
import { useUser } from '@/providers/UserProvider';
import logo from '../../favicon.png';
import classes from './Navbar.module.css';

const links = [
  { icon: IconHome, label: 'Dashboard', path: '/' },
  { icon: IconMap, label: 'Map', path: '/map' },
  { icon: IconBook, label: 'Journal', path: '/journal' },
  { icon: IconCompass, label: 'Explore', path: '/explore' },
  { icon: IconTimeline, label: 'Stats', path: '/stats' },
];

const collections = [
  { emoji: '👍', label: 'Sales' },
  { emoji: '🚚', label: 'Deliveries' },
  { emoji: '💸', label: 'Discounts' },
  { emoji: '💰', label: 'Profits' },
  { emoji: '✨', label: 'Reports' },
  { emoji: '🛒', label: 'Orders' },
  { emoji: '📅', label: 'Events' },
  { emoji: '🙈', label: 'Debts' },
  { emoji: '💁‍♀️', label: 'Customers' },
  { emoji: '👍', label: 'Sales' },
  { emoji: '🚚', label: 'Deliveries' },
  { emoji: '💸', label: 'Discounts' },
  { emoji: '💰', label: 'Profits' },
  { emoji: '✨', label: 'Reports' },
  { emoji: '🛒', label: 'Orders' },
  { emoji: '📅', label: 'Events' },
  { emoji: '🙈', label: 'Debts' },
  { emoji: '💁‍♀️', label: 'Customers' },
];

export const Navbar = () => {
  const navigate = useNavigate();
  const location = useLocation();
  const { user } = useUser();

  const mainLinks = links.map((link) => {
    const isActive =
      location.pathname === link.path ||
      (location.pathname.startsWith(link.path) && link.path !== '/');
    return (
      <UnstyledButton
        key={link.label}
        className={`${classes.mainLink} ${isActive ? classes.active : classes.mainLinkHover}`}
      >
        <div
          className={`${classes.mainLinkInner} ${isActive ? classes.active : ''}`}
          onClick={() => navigate(link.path)}
        >
          <link.icon size={22} className={classes.mainLinkIcon} stroke={2} />
          <span>{link.label}</span>
        </div>
      </UnstyledButton>
    );
  });

  const collectionLinks = collections.map((collection) => (
    <a
      href="#"
      onClick={(event) => event.preventDefault()}
      key={collection.label}
      className={classes.collectionLink}
    >
      <Box component="span" mr={9} fz={16}>
        {collection.emoji}
      </Box>{' '}
      {collection.label}
    </a>
  ));

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
        <Group className={classes.collectionsHeader} justify="space-between">
          <Text size="xs" fw={500} c="dimmed">
            Trips
          </Text>
          <Tooltip label="Create a new trip" withArrow position="right">
            <ActionIcon variant="default" size={18}>
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
        <div className={classes.collections}>{collectionLinks}</div>
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
    </nav>
  );
};
