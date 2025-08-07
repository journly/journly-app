import { useMemo, useState } from 'react';
import {
  IconArrowsMaximize,
  IconCalendar,
  IconCurrencyDollar,
  IconMapPin,
  IconPlane,
  IconPlus,
  IconSofa,
  IconUsers,
} from '@tabler/icons-react';
import { useSubscribe } from 'replicache-react';
import {
  Badge,
  Box,
  Center,
  Flex,
  Grid,
  Group,
  ScrollArea,
  Select,
  Stack,
  Text,
  Title,
  Tooltip,
  UnstyledButton,
} from '@mantine/core';
import { SummaryInfoButton } from '@/components/Buttons/SummaryInfoButton';
import { EditDescriptionModal } from '@/components/Modals/EditDescriptionModal';
import { TaskItem } from '@/components/Trip/TaskItem';
import { getTasksByTrip } from '@/models/tasks';
import { Trip, updateTrip } from '@/models/trip';
import { useReplicache } from '@/providers/ReplicacheProvider';
import { getTripDurationString } from '@/utils/dates';
import classes from './Tab.module.css';

interface OverviewTabProps {
  trip: Trip | null;
  numberOfContributors: number;
  updateTrip: (trip: Partial<Trip>) => void;
  openCollaboratorsModal: () => void;
}

export default function OverviewTab({
  trip,
  updateTrip,
  numberOfContributors,
  openCollaboratorsModal,
}: OverviewTabProps) {
  if (!trip) return null;

  return (
    <Flex wrap="wrap" gap={20} justify="space-between" maw={1150} w="100%" mx={20} mb={20}>
      <SummarySection
        trip={trip}
        updateTrip={updateTrip}
        openCollaboratorsModal={openCollaboratorsModal}
        numberOfContributors={numberOfContributors}
      />
      <TaskSection tripId={trip.id} />
    </Flex>
  );
}

const SummarySection = ({
  trip,
  updateTrip,
  openCollaboratorsModal,
  numberOfContributors,
}: {
  trip: Trip;
  updateTrip: (trip: Partial<Trip>) => void;
  openCollaboratorsModal: () => void;
  numberOfContributors: number;
}) => {
  const [editDescriptionModalOpened, setEditDescriptionModalOpened] = useState(false);

  return (
    <>
      <EditDescriptionModal
        opened={editDescriptionModalOpened}
        onClose={() => setEditDescriptionModalOpened(false)}
        onSave={(description) => {
          updateTrip({ description });
        }}
        tripDescription={trip.description ?? ''}
      />
      <Box
        w={{ base: '100%', sm: '100%', md: '100%', lg: '65%' }}
        h="fit-content"
        className={classes.sectionContainer}
      >
        <Title order={6}>Trip Summary</Title>
        <Box mt={10}>
          <Text fz="sm" fw={600}>
            Description
          </Text>
          <SummaryInfoButton onClick={() => setEditDescriptionModalOpened(true)}>
            {trip?.description === undefined || trip.description === ''
              ? 'No description'
              : trip.description}
          </SummaryInfoButton>
        </Box>
        <Grid mt={10}>
          <Grid.Col span={6}>
            <Text fz="sm" fw={600}>
              Destination
            </Text>
            <Flex align="center" gap={5}>
              <SummaryInfoButton onClick={() => {}} leftIcon={<IconMapPin size={16} />}>
                Tokyo - Kyoto - Osaka
              </SummaryInfoButton>
            </Flex>
          </Grid.Col>
          <Grid.Col span={6}>
            <Text fz="sm" fw={600}>
              Budget Status
            </Text>
            <Flex align="center" gap={5}>
              <SummaryInfoButton onClick={() => {}} leftIcon={<IconCurrencyDollar size={16} />}>
                $2,180 spent of $3,500
              </SummaryInfoButton>
            </Flex>
          </Grid.Col>
          <Grid.Col span={6}>
            <Text fz="sm" fw={600}>
              Duration
            </Text>
            <Flex align="center" gap={5}>
              <SummaryInfoButton onClick={() => {}} leftIcon={<IconCalendar size={16} />}>
                {getTripDurationString([trip.startDate, trip.endDate])}
              </SummaryInfoButton>
            </Flex>
          </Grid.Col>
          <Grid.Col span={6}>
            <Text fz="sm" fw={600}>
              Transportation
            </Text>
            <Flex align="center" gap={5}>
              <SummaryInfoButton onClick={() => {}} leftIcon={<IconPlane size={16} />}>
                2 flights
              </SummaryInfoButton>
            </Flex>
          </Grid.Col>
          <Grid.Col span={6}>
            <Text fz="sm" fw={600}>
              Travellers
            </Text>
            <Flex align="center" gap={5}>
              <SummaryInfoButton
                onClick={openCollaboratorsModal}
                leftIcon={<IconUsers size={16} />}
              >
                {numberOfContributors} traveller{numberOfContributors > 1 ? 's' : ''}
              </SummaryInfoButton>
            </Flex>
          </Grid.Col>
          <Grid.Col span={6}>
            <Text fz="sm" fw={600}>
              Accommodation
            </Text>
            <Flex align="center" gap={5}>
              <SummaryInfoButton onClick={() => {}} leftIcon={<IconSofa size={16} />}>
                2 hotels
              </SummaryInfoButton>
            </Flex>
          </Grid.Col>
        </Grid>
      </Box>
      <EditDescriptionModal
        opened={editDescriptionModalOpened}
        onClose={() => setEditDescriptionModalOpened(false)}
        onSave={(description) => {
          updateTrip({ description });
        }}
        tripDescription={trip.description ?? ''}
      />
    </>
  );
};

const TaskSection = ({ tripId }: { tripId: string }) => {
  const { rep } = useReplicache();
  const [filter, setFilter] = useState<'all' | 'completed' | 'incomplete'>('all');

  const tasks = useSubscribe(rep, (tx) => getTasksByTrip(tx, tripId ?? ''), {
    default: [],
    dependencies: [tripId],
  });

  const addTask = async () => {
    if (!rep) return;

    await rep.mutate.createTask({
      tripId,
      title: 'New Task',
      description: '',
      completed: false,
      urgency: 'low',
    });
  };

  const filteredTasks = useMemo(() => {
    if (filter === 'all') return tasks;
    if (filter === 'completed') return tasks.filter((task) => task.completed);
    return tasks.filter((task) => !task.completed);
  }, [tasks, filter]);

  const highUrgencyTasks = useMemo(() => {
    return filteredTasks.filter((task) => task.urgency === 'high');
  }, [filteredTasks]);

  const mediumUrgencyTasks = useMemo(() => {
    return filteredTasks.filter((task) => task.urgency === 'medium');
  }, [filteredTasks]);

  const lowUrgencyTasks = useMemo(() => {
    return filteredTasks.filter((task) => task.urgency === 'low');
  }, [filteredTasks]);

  return (
    <Box
      w={{ base: '100%', sm: '100%', md: '100%', lg: '32%' }}
      className={classes.sectionContainer}
    >
      <Box>
        <Group justify="space-between">
          <Title order={6}>Tasks</Title>
          <Group>
            <Select
              data={['all', 'completed', 'incomplete']}
              w={130}
              value={filter}
              onChange={(value) => setFilter(value as 'all' | 'completed' | 'incomplete')}
            />
            <Tooltip label="Add Task">
              <UnstyledButton className={classes.addButton} onClick={addTask}>
                <IconPlus size={16} />
              </UnstyledButton>
            </Tooltip>
            <Tooltip label="Expand Task List">
              <UnstyledButton className={classes.taskExpandButton}>
                <IconArrowsMaximize size={16} />
              </UnstyledButton>
            </Tooltip>
          </Group>
        </Group>
        {filteredTasks.length === 0 ? (
          <Center h="100%" mt={25}>
            <Text>No tasks yet</Text>
          </Center>
        ) : (
          <Stack gap={10} mt={10}>
            {highUrgencyTasks.length > 0 && (
              <>
                <Badge color="red" size="sm">
                  High
                </Badge>
                {highUrgencyTasks.map((task) => (
                  <TaskItem key={task.id} task={task} />
                ))}
              </>
            )}
            {mediumUrgencyTasks.length > 0 && (
              <>
                <Badge color="yellow" size="sm">
                  Medium
                </Badge>
                {mediumUrgencyTasks.map((task) => (
                  <TaskItem key={task.id} task={task} />
                ))}
              </>
            )}
            {lowUrgencyTasks.length > 0 && (
              <>
                <Badge color="green" size="sm">
                  Low
                </Badge>
                {lowUrgencyTasks.map((task) => (
                  <TaskItem key={task.id} task={task} />
                ))}
              </>
            )}
          </Stack>
        )}
      </Box>
    </Box>
  );
};
