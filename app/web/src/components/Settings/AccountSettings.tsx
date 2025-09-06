import { useState } from 'react';
import {
  IconAlertHexagon,
  IconCamera,
  IconEdit,
  IconShield,
  IconTrash,
  IconUser,
} from '@tabler/icons-react';
import {
  ActionIcon,
  Alert,
  Avatar,
  Badge,
  Button,
  Card,
  Group,
  Modal,
  PasswordInput,
  ScrollArea,
  Stack,
  Switch,
  Text,
  Textarea,
  TextInput,
  Title,
  Tooltip,
} from '@mantine/core';

export const AccountSettings = () => {
  const user = {
    username: 'John Doe',
    email: 'john.doe@example.com',
    avatar: 'https://via.placeholder.com/150',
    bio: 'I am a software engineer',
  };

  const [editProfile, setEditProfile] = useState(false);
  const [changePassword, setChangePassword] = useState(false);
  const [deleteAccount, setDeleteAccount] = useState(false);
  const [exportData, setExportData] = useState(false);

  // Form states
  const [profileData, setProfileData] = useState({
    username: user?.username || '',
    email: user?.email || '',
    bio: user?.bio || '',
  });

  const [passwordData, setPasswordData] = useState({
    currentPassword: '',
    newPassword: '',
    confirmPassword: '',
  });

  const [notifications, setNotifications] = useState({
    emailNotifications: true,
    pushNotifications: true,
    marketingEmails: false,
    tripReminders: true,
  });

  const handleProfileUpdate = async () => {
    try {
      // TODO: Implement profile update API call
      console.log('Updating profile:', profileData);
      setEditProfile(false);
    } catch (error) {
      console.error('Failed to update profile:', error);
    }
  };

  const handlePasswordChange = async () => {
    try {
      if (passwordData.newPassword !== passwordData.confirmPassword) {
        alert('Passwords do not match');
        return;
      }
      // TODO: Implement password change API call
      console.log('Changing password');
      setChangePassword(false);
      setPasswordData({ currentPassword: '', newPassword: '', confirmPassword: '' });
    } catch (error) {
      console.error('Failed to change password:', error);
    }
  };

  const handleAccountDeletion = async () => {
    try {
      // TODO: Implement account deletion API call
      console.log('Deleting account');
      setDeleteAccount(false);
    } catch (error) {
      console.error('Failed to delete account:', error);
    }
  };

  const handleDataExport = async () => {
    try {
      // TODO: Implement data export API call
      console.log('Exporting data');
      setExportData(false);
    } catch (error) {
      console.error('Failed to export data:', error);
    }
  };

  return (
    <ScrollArea h={400}>
      <Stack gap="lg" mr="md">
        <Title order={3} size="h4">
          Account Settings
        </Title>

        {/* Profile Information */}
        <Card withBorder padding="md">
          <Group justify="space-between" align="center" mb="md">
            <Group>
              <IconUser size={20} />
              <Text fw={500}>Profile Information</Text>
            </Group>
            <Badge variant="light">Active</Badge>
          </Group>

          <Group align="flex-start" mb="md">
            <Avatar src={user?.avatar} size="xl" radius="xl" color="blue">
              {user?.username?.charAt(0) || 'U'}
            </Avatar>
            <Stack gap="xs" style={{ flex: 1 }}>
              <Text fw={500} size="lg">
                {user?.username || 'Username'}
              </Text>
              <Text size="sm" c="dimmed">
                {user?.email || 'email@example.com'}
              </Text>
              <Text size="sm" c="dimmed">
                Member since {new Date().toLocaleDateString()}
              </Text>
            </Stack>
            <Tooltip label="Edit profile">
              <ActionIcon variant="light" onClick={() => setEditProfile(true)}>
                <IconEdit size={16} />
              </ActionIcon>
            </Tooltip>
          </Group>

          <Group>
            <Button variant="light" leftSection={<IconCamera size={16} />} size="sm">
              Change Photo
            </Button>
            <Button
              variant="light"
              leftSection={<IconEdit size={16} />}
              size="sm"
              onClick={() => setEditProfile(true)}
            >
              Edit Profile
            </Button>
          </Group>
        </Card>

        {/* Security Settings */}
        <Card withBorder padding="md">
          <Group justify="space-between" align="center" mb="md">
            <Group>
              <IconShield size={20} />
              <Text fw={500}>Security</Text>
            </Group>
          </Group>

          <Stack gap="md">
            <Group justify="space-between">
              <div>
                <Text fw={500}>Password</Text>
                <Text size="sm" c="dimmed">
                  Last changed 30 days ago
                </Text>
              </div>
              <Button variant="light" size="sm" onClick={() => setChangePassword(true)}>
                Change
              </Button>
            </Group>

            <Group justify="space-between">
              <div>
                <Text fw={500}>Two-Factor Authentication</Text>
                <Text size="sm" c="dimmed">
                  Add an extra layer of security
                </Text>
              </div>
              <Switch />
            </Group>

            <Group justify="space-between">
              <div>
                <Text fw={500}>Login Sessions</Text>
                <Text size="sm" c="dimmed">
                  Manage active sessions
                </Text>
              </div>
              <Button variant="light" size="sm">
                View
              </Button>
            </Group>
          </Stack>
        </Card>
        {/* Danger Zone */}
        <Card withBorder padding="md">
          <Group justify="space-between" align="center" mb="md">
            <Group>
              <IconAlertHexagon size={20} />
              <Text fw={500}>Danger Zone</Text>
            </Group>
          </Group>

          <Stack gap="md">
            <Group justify="space-between">
              <div>
                <Text fw={500}>Delete Account</Text>
                <Text size="sm" c="dimmed">
                  Permanently delete your account
                </Text>
              </div>
              <Button
                variant="light"
                color="red"
                size="sm"
                leftSection={<IconTrash size={16} />}
                onClick={() => setDeleteAccount(true)}
              >
                Delete
              </Button>
            </Group>
          </Stack>
        </Card>

        {/* Edit Profile Modal */}
        <Modal
          opened={editProfile}
          onClose={() => setEditProfile(false)}
          title="Edit Profile"
          size="md"
        >
          <Stack gap="md">
            <TextInput
              label="Username"
              value={profileData.username}
              onChange={(event) =>
                setProfileData({
                  ...profileData,
                  username: event.currentTarget.value,
                })
              }
            />
            <TextInput
              label="Email"
              value={profileData.email}
              onChange={(event) =>
                setProfileData({
                  ...profileData,
                  email: event.currentTarget.value,
                })
              }
            />
            <Textarea
              label="Bio"
              placeholder="Tell us about yourself"
              value={profileData.bio}
              onChange={(event) =>
                setProfileData({
                  ...profileData,
                  bio: event.currentTarget.value,
                })
              }
            />
            <Group justify="flex-end">
              <Button variant="light" onClick={() => setEditProfile(false)}>
                Cancel
              </Button>
              <Button onClick={handleProfileUpdate}>Save Changes</Button>
            </Group>
          </Stack>
        </Modal>

        {/* Change Password Modal */}
        <Modal
          opened={changePassword}
          onClose={() => setChangePassword(false)}
          title="Change Password"
          size="md"
        >
          <Stack gap="md">
            <PasswordInput
              label="Current Password"
              value={passwordData.currentPassword}
              onChange={(event) =>
                setPasswordData({
                  ...passwordData,
                  currentPassword: event.currentTarget.value,
                })
              }
            />
            <PasswordInput
              label="New Password"
              value={passwordData.newPassword}
              onChange={(event) =>
                setPasswordData({
                  ...passwordData,
                  newPassword: event.currentTarget.value,
                })
              }
            />
            <PasswordInput
              label="Confirm New Password"
              value={passwordData.confirmPassword}
              onChange={(event) =>
                setPasswordData({
                  ...passwordData,
                  confirmPassword: event.currentTarget.value,
                })
              }
            />
            <Group justify="flex-end">
              <Button variant="light" onClick={() => setChangePassword(false)}>
                Cancel
              </Button>
              <Button onClick={handlePasswordChange}>Change Password</Button>
            </Group>
          </Stack>
        </Modal>

        {/* Delete Account Modal */}
        <Modal
          opened={deleteAccount}
          onClose={() => setDeleteAccount(false)}
          title="Delete Account"
          size="md"
        >
          <Stack gap="md">
            <Alert color="red" title="Warning">
              This action cannot be undone. All your data will be permanently deleted.
            </Alert>
            <Text size="sm">
              Are you sure you want to delete your account? This action is irreversible.
            </Text>
            <Group justify="flex-end">
              <Button variant="light" onClick={() => setDeleteAccount(false)}>
                Cancel
              </Button>
              <Button color="red" onClick={handleAccountDeletion}>
                Delete Account
              </Button>
            </Group>
          </Stack>
        </Modal>
      </Stack>
    </ScrollArea>
  );
};
