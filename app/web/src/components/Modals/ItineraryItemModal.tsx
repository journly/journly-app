import { useEffect, useState } from 'react';
import usePlacesService from 'react-google-autocomplete/lib/usePlacesAutocompleteService';
import {
  Autocomplete,
  Button,
  Checkbox,
  Flex,
  Group,
  Loader,
  Modal,
  Text,
  Textarea,
  TextInput,
} from '@mantine/core';
import { DatePickerInput, TimePicker } from '@mantine/dates';
import { GooglePlaceDetails } from '@/types/googleMaps';

export const ItineraryItemModal = ({
  opened,
  onClose,
  title,
  maxDate,
  minDate,
}: {
  maxDate?: Date;
  minDate?: Date;
  opened: boolean;
  onClose: () => void;
  title: string;
}) => {
  const { placesService, placePredictions, getPlacePredictions, isPlacePredictionsLoading } =
    usePlacesService({
      apiKey: import.meta.env.VITE_GOOGLE_MAPS_API_KEY || '',
    });

  useEffect(() => {
    if (placePredictions.length > 0 && placesService) {
      placesService.getDetails(
        {
          placeId: placePredictions[0].place_id,
        },
        (placeDetails: GooglePlaceDetails) => console.log(placeDetails.geometry.location.lat())
      );
    }
  }, [placePredictions, placesService]);

  return (
    <Modal opened={opened} onClose={onClose} centered title={title}>
      <Flex direction="column" gap={10}>
        <TextInput placeholder="Enter title" label="Title" />
        <Autocomplete
          placeholder="Enter location"
          label="Location"
          data={placePredictions?.map((prediction) => prediction.description) || []}
          onChange={(value) => {
            if (value && value.length > 2) {
              getPlacePredictions({ input: value });
            }
          }}
          rightSection={isPlacePredictionsLoading ? <Loader size="xs" /> : null}
        />

        <Group>
          <Text w="100px">Start time</Text>
          <DatePickerInput placeholder="Start date" maxDate={maxDate} minDate={minDate} />
          <TimePicker withDropdown hoursStep={1} minutesStep={5} />
        </Group>
        <Group>
          <Text w="100px">End time</Text>
          <DatePickerInput placeholder="End date" maxDate={maxDate} minDate={minDate} />
          <TimePicker withDropdown hoursStep={1} minutesStep={5} />
        </Group>
        <Checkbox label="All day" />
        <Textarea placeholder="Enter note" label="Note" />
      </Flex>
    </Modal>
  );
};
