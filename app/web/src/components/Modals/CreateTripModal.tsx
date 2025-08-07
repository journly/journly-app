import { useEffect } from 'react';
import { Button, Modal, TextInput } from '@mantine/core';
import { DatePickerInput } from '@mantine/dates';
import { useForm } from '@mantine/form';
import { useAllTrips } from '@/providers/AllTripsProvider';
import classes from './CreateTripModal.module.css';

interface CreateTripModalProps {
  open: boolean;
  onClose: () => void;
}

export const CreateTripModal = ({ open, onClose }: CreateTripModalProps) => {
  const { createTrip } = useAllTrips();

  const form = useForm({
    initialValues: {
      name: '',
      description: '',
      dates: [null, null] as [Date | null, Date | null],
    },
    validate: {
      name: (value) => (value.length > 0 ? null : 'Trip name is required'),
    },
  });

  // Reset form when modal is closed
  useEffect(() => {
    if (!open) {
      form.reset();
    }
  }, [open]); // reset when open becomes false

  // Handler for closing modal and resetting form
  const handleClose = () => {
    form.reset();
    onClose();
  };

  // Handler for form submit
  const handleSubmit = async (values: typeof form.values) => {
    await createTrip({
      name: values.name,
      description: values.description,
      startDate: values.dates[0] ? new Date(values.dates[0]).toISOString() : undefined,
      endDate: values.dates[1] ? new Date(values.dates[1]).toISOString() : undefined,
    });

    form.reset();
    onClose();
  };

  return (
    <Modal opened={open} onClose={handleClose} title="Create Trip" centered>
      <form onSubmit={form.onSubmit(handleSubmit)} className={classes.form}>
        <TextInput label="Trip Name" placeholder="Trip Name" {...form.getInputProps('name')} />
        <TextInput
          label="Trip Description"
          placeholder="Trip Description"
          {...form.getInputProps('description')}
        />
        <DatePickerInput
          label="Trip dates"
          placeholder="Trip dates"
          type="range"
          {...form.getInputProps('dates')}
          clearable
        />
        <Button type="submit">Create Trip</Button>
      </form>
    </Modal>
  );
};
