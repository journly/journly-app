import { useEffect, useState } from 'react';
import {
  IconCalendar,
  IconCurrencyDollar,
  IconHome,
  IconSofa,
  IconUsers,
} from '@tabler/icons-react';
import { useParams } from 'react-router-dom';
import { useSubscribe } from 'replicache-react';
import {
  Avatar,
  AvatarGroup,
  BackgroundImage,
  Box,
  Center,
  Flex,
  Popover,
  Tabs,
  Text,
  TextInput,
  Title,
  Tooltip,
  UnstyledButton,
} from '@mantine/core';
import { DatePicker } from '@mantine/dates';
import { getCollaboratorsByTrip } from '@/models/collaborators';
import { expensesByTrip } from '@/models/expenses';
import { itineraryItemsByTrip } from '@/models/itineraryItem';
import { getTrip, Trip } from '@/models/trip';
import { useReplicache } from '@/providers/ReplicacheProvider';
import { formatTripDatesDisplay } from '@/utils/dates';
import { useEventSourcePoke } from '@/utils/poke';
import OverviewTab from './tabs/Overview.tab';
import classes from './Trip.module.css';

export default function TripPage() {
  const { tripId } = useParams();
  const { rep } = useReplicache();
  const [activeTab, setActiveTab] = useState('overview');
  const [editingName, setEditingName] = useState(false);
  const [tripTitle, setTripTitle] = useState('');
  const [tripDates, setTripDates] = useState<[string | null, string | null]>([null, null]);
  const currentTripId = tripId ? `trip/${tripId}` : '';

  const trip = useSubscribe(rep, (tx) => getTrip(tx, currentTripId), {
    default: null,
    dependencies: [currentTripId],
  });

  const collaborators = useSubscribe(rep, (tx) => getCollaboratorsByTrip(tx, currentTripId), {
    default: [],
    dependencies: [currentTripId],
  });

  useEffect(() => {
    setTripTitle(trip?.name || '');
    setTripDates([trip?.startDate || null, trip?.endDate || null]);
  }, [trip]);

  const itinerary = useSubscribe(rep, (tx) => itineraryItemsByTrip(tx, currentTripId), {
    default: [],
    dependencies: [currentTripId],
  });

  const expenses = useSubscribe(rep, (tx) => expensesByTrip(tx, currentTripId), {
    default: [],
    dependencies: [currentTripId],
  });

  const updateTrip = async (updates: Partial<Trip>) => {
    if (!rep || !trip) return;
    await rep.mutate.updateTrip({ ...trip, ...updates, updatedAt: new Date().toISOString() });
  };

  useEventSourcePoke(`${import.meta.env.VITE_REPLICACHE_POKE_URL}?channel=${currentTripId}`, rep);

  const tabConfig = [
    {
      value: 'overview',
      label: 'Overview',
      icon: <IconHome size={16} />,
    },
    {
      value: 'itinerary',
      label: 'Itinerary',
      icon: <IconCalendar size={16} />,
    },
    {
      value: 'bookings',
      label: 'Bookings',
      icon: <IconSofa size={16} />,
    },
    {
      value: 'budget',
      label: 'Budget',
      icon: <IconCurrencyDollar size={16} />,
    },
  ];

  return (
    <>
      <Box className={classes.coverImageContainer} w="100%" h={200}>
        <BackgroundImage src={trip?.coverImage ?? ''} w="100%" h={200}>
          <Flex w="100%" h="100%" justify="center" align="flex-end">
            <Flex maw={1150} w="100%" mb={20} mx={20} justify="space-between" align="flex-end">
              <Box ml={5} className={classes.bannerImageText}>
                {editingName ? (
                  <TextInput
                    variant="unstyled"
                    value={tripTitle}
                    autoFocus
                    size="26px"
                    fw={700}
                    mb={12}
                    pt={7}
                    key={trip?.id}
                    spellCheck={false}
                    onChange={(e: React.ChangeEvent<HTMLInputElement>) =>
                      setTripTitle(e.target.value)
                    }
                    onBlur={() => {
                      setEditingName(false);
                      setTripTitle(trip?.name || '');
                    }}
                    onKeyDown={(e) => {
                      if (e.key === 'Enter') {
                        setEditingName(false);
                        updateTrip({ name: tripTitle.trimEnd() });
                      }
                    }}
                    className={classes.editNameInput}
                  />
                ) : (
                  <Title
                    order={2}
                    mb={10}
                    onClick={() => setEditingName(true)}
                    style={{ cursor: 'pointer' }}
                    title="Click to edit"
                  >
                    {trip?.name}
                  </Title>
                )}
                <Flex align="center" gap={4} className={classes.dateContainer}>
                  <IconCalendar size={16} />
                  <Popover
                    position="bottom"
                    withArrow
                    arrowPosition="center"
                    arrowOffset={10}
                    arrowSize={10}
                  >
                    <Popover.Target>
                      <UnstyledButton>
                        <Text size="xs">{formatTripDatesDisplay(tripDates)}</Text>
                      </UnstyledButton>
                    </Popover.Target>
                    <Popover.Dropdown>
                      <DatePicker
                        value={tripDates}
                        onChange={(dates) => {
                          updateTrip({
                            startDate: dates[0] ? new Date(dates[0]).toISOString() : undefined,
                            endDate: dates[1] ? new Date(dates[1]).toISOString() : undefined,
                          });
                        }}
                        type="range"
                        allowSingleDateInRange
                      />
                    </Popover.Dropdown>
                  </Popover>
                </Flex>
              </Box>
              <Flex align="center" gap={10} justify="center">
                <AvatarGroup spacing="sm">
                  {collaborators && collaborators.length > 0 && (
                    <>
                      {collaborators.slice(0, 3).map((collaborator) => (
                        <Tooltip key={collaborator.id} label={collaborator.username} position="top">
                          <Avatar src={collaborator.avatarUrl} />
                        </Tooltip>
                      ))}
                      {collaborators.length > 3 && <Avatar>+{collaborators.length - 3}</Avatar>}
                    </>
                  )}
                </AvatarGroup>
                <UnstyledButton className={classes.addCollaboratorButton}>
                  <IconUsers size={25} stroke={2.5} />
                </UnstyledButton>
              </Flex>
            </Flex>
          </Flex>
        </BackgroundImage>
      </Box>
      <Center w="100%">
        <Tabs
          value={activeTab}
          onChange={(value) => setActiveTab(value ?? 'overview')}
          w="100%"
          className={classes.tabsContainer}
        >
          <Tabs.List w="100%" className={classes.tabsList} mb={25}>
            <Flex maw={1150} w="100%" mx={20}>
              {tabConfig.map((tab) => (
                <Tabs.Tab
                  key={tab.value}
                  value={tab.value}
                  className={classes.tab + ' ' + (activeTab === tab.value ? classes.activeTab : '')}
                  leftSection={tab.icon}
                >
                  {tab.label}
                </Tabs.Tab>
              ))}
            </Flex>
          </Tabs.List>
          <Tabs.Panel value="overview" w="100%">
            <Flex justify="center">
              <OverviewTab trip={trip} updateTrip={updateTrip} />
            </Flex>
          </Tabs.Panel>
        </Tabs>
      </Center>
    </>
  );
}
