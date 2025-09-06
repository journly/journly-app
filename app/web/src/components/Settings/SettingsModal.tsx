import { IconBell, IconPalette, IconUser } from '@tabler/icons-react';
import { Modal, Tabs } from '@mantine/core';
import { AccountSettings } from './AccountSettings';
import { AppearanceSettings } from './AppearanceSettings';
import { NotificationSettings } from './NotificationSettings';

interface SettingsModalProps {
  opened: boolean;
  setOpened: (opened: boolean) => void;
}

export const SettingsModal = ({ opened, setOpened }: SettingsModalProps) => {
  return (
    <Modal opened={opened} onClose={() => setOpened(false)} size="xl" centered title="Settings">
      <Tabs orientation="vertical" defaultValue="account" h={400}>
        <Tabs.List mr="md">
          <Tabs.Tab value="account" leftSection={<IconUser size={16} />}>
            Account
          </Tabs.Tab>
          <Tabs.Tab value="notifications" leftSection={<IconBell size={16} />}>
            Notifications
          </Tabs.Tab>
          <Tabs.Tab value="appearance" leftSection={<IconPalette size={16} />}>
            Appearance
          </Tabs.Tab>
        </Tabs.List>
        <Tabs.Panel value="account">
          <AccountSettings />
        </Tabs.Panel>
        <Tabs.Panel value="notifications">
          <NotificationSettings />
        </Tabs.Panel>
        <Tabs.Panel value="appearance">
          <AppearanceSettings />
        </Tabs.Panel>
      </Tabs>
    </Modal>
  );
};
