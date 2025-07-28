import { IconChevronDown } from '@tabler/icons-react';
import { Avatar, Box, Combobox, Flex, Group, InputBase, Modal, Select, Text } from '@mantine/core';
import { Collaborator } from '@/models/collaborators';
import { useReplicache } from '@/providers/ReplicacheProvider';

interface CollaboratorsModalProps {
  collaborators: Collaborator[];
  opened: boolean;
  ownerId: string;
  setOpened: (opened: boolean) => void;
}

const collaboratorRoles = [
  {
    value: 'owner',
    label: 'Owner',
  },
  {
    value: 'editor',
    label: 'Editor',
  },
  {
    value: 'viewer',
    label: 'Viewer',
  },
];

export const CollaboratorsModal = ({
  collaborators,
  ownerId,
  opened,
  setOpened,
}: CollaboratorsModalProps) => {
  const { rep } = useReplicache();

  const setCollaboratorRole = async (collaboratorId: string, role: string) => {
    if (!rep) return;
    const collaborator = collaborators.find((c) => c.id === collaboratorId);
    if (!collaborator) return;

    await rep.mutate.updateCollaborator({
      ...collaborator,
      role: role as 'owner' | 'editor' | 'viewer',
    });
  };

  return (
    <Modal
      opened={opened}
      onClose={() => setOpened(false)}
      title="Collaborators"
      size="md"
      centered
    >
      <Flex direction="column" gap={10}>
        {collaborators.map((collaborator) => (
          <Flex key={collaborator.id} align="center" gap={15} justify="space-between">
            <Group>
              <Avatar src={collaborator.avatarUrl} />
              <Box>
                <Text>{collaborator.username}</Text>
                <Text mt={-3} fz="xs" c="dimmed">
                  {collaborator.email}
                </Text>
              </Box>
            </Group>

            <Select
              data={collaboratorRoles}
              value={collaborator.role}
              onChange={(value) => value && setCollaboratorRole(collaborator.id, value)}
              rightSection={<IconChevronDown size={16} />}
              w={100}
            />
          </Flex>
        ))}
      </Flex>
    </Modal>
  );
};
