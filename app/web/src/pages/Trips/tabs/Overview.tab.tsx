import { useState } from 'react';
import {
  IconCalendar,
  IconCurrencyDollar,
  IconHotelService,
  IconMapPin,
  IconPlane,
  IconSofa,
  IconUsers,
} from '@tabler/icons-react';
import { Box, Flex, Grid, Text, Title, UnstyledButton } from '@mantine/core';
import { SummaryInfoButton } from '@/components/Buttons/SummaryInfoButton';
import { EditDescriptionModal } from '@/components/Modals/EditDescriptionModal';
import { Trip } from '@/models/trip';
import { getTripDurationString } from '@/utils/dates';
import classes from './Tab.module.css';

interface OverviewTabProps {
  trip: Trip | null;
  updateTrip: (trip: Partial<Trip>) => void;
}

export default function OverviewTab({ trip, updateTrip }: OverviewTabProps) {
  const [editDescriptionModalOpened, setEditDescriptionModalOpened] = useState(false);

  if (!trip) return null;

  return (
    <Flex wrap="wrap" gap={20} justify="space-between" maw={1150} w="100%" mx={20}>
      <Box
        w={{ base: '100%', sm: '100%', md: '100%', lg: '65%' }}
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
              <SummaryInfoButton onClick={() => {}} leftIcon={<IconUsers size={16} />}>
                2 travellers
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
      <Box
        w={{ base: '100%', sm: '100%', md: '100%', lg: '32%' }}
        className={classes.sectionContainer}
      >
        bye
      </Box>
      <Box w="100%" className={classes.sectionContainer}>
        shy
      </Box>
      <EditDescriptionModal
        opened={editDescriptionModalOpened}
        onClose={() => setEditDescriptionModalOpened(false)}
        onSave={(description) => {
          updateTrip({ description });
        }}
        tripDescription={trip.description ?? ''}
      />
    </Flex>
  );
}
