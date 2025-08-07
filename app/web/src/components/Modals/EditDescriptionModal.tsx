// generate a modal that allows the user to edit the description of a trip

import { useEffect, useState } from 'react';
import { Button, Group, Modal, Text, Textarea } from '@mantine/core';

interface EditDescriptionModalProps {
  opened: boolean;
  onClose: () => void;
  onSave: (description: string) => void;
  tripDescription: string;
}

const MAX_LENGTH = 150;

export const EditDescriptionModal = ({
  opened,
  onClose,
  onSave,
  tripDescription,
}: EditDescriptionModalProps) => {
  const [description, setDescription] = useState(tripDescription);

  useEffect(() => {
    if (opened) {
      setDescription(tripDescription);
    }
  }, [opened, tripDescription]);

  const handleClose = () => {
    setDescription(tripDescription);
    onClose();
  };

  const handleChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    if (e.target.value.length <= MAX_LENGTH) {
      setDescription(e.target.value);
    }
  };

  const handleSave = () => {
    onSave(description);
    handleClose();
  };

  return (
    <Modal opened={opened} onClose={handleClose} centered title="Edit Description">
      <Textarea
        value={description}
        onChange={handleChange}
        maxLength={MAX_LENGTH}
        minRows={3}
        autosize
        placeholder="Enter trip description"
      />
      <Group justify="space-between" mt={8} mb={4}>
        <Text size="xs" color={description.length === MAX_LENGTH ? 'red' : 'dimmed'}>
          {description.length}/{MAX_LENGTH} characters
        </Text>
        <Button onClick={handleSave} disabled={description.length > MAX_LENGTH}>
          Save
        </Button>
      </Group>
    </Modal>
  );
};
