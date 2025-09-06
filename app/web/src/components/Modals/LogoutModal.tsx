import { IconLogout } from '@tabler/icons-react';
import { Button, Group, Modal, Stack, Text, Title } from '@mantine/core';
import { useAuth } from '@/providers/AuthProvider';

interface LogoutModalProps {
  opened: boolean;
  setOpened: (opened: boolean) => void;
}

export const LogoutModal = ({ opened, setOpened }: LogoutModalProps) => {
  const { logout } = useAuth();

  return (
    <Modal
      opened={opened}
      onClose={() => setOpened(false)}
      size="xs"
      centered
      withCloseButton={false}
    >
      <Stack gap="md">
        <Title order={4}>Logout</Title>
        <Text>Are you sure you want to logout?</Text>
        <Group w="100%" justify="center">
          <Button onClick={() => logout()} color="red" leftSection={<IconLogout size={16} />}>
            Logout
          </Button>
          <Button variant="default" onClick={() => setOpened(false)}>
            Cancel
          </Button>
        </Group>
      </Stack>
    </Modal>
  );
};
