import { useState } from 'react';
import {
  IconDeviceDesktop,
  IconMoon,
  IconPalette,
  IconSun,
  IconTypography,
} from '@tabler/icons-react';
import {
  ActionIcon,
  Badge,
  Card,
  ColorInput,
  Divider,
  Group,
  ScrollArea,
  ScrollAreaAutosize,
  Select,
  Slider,
  Stack,
  Switch,
  Text,
  Title,
  Tooltip,
  useMantineColorScheme,
} from '@mantine/core';

export const AppearanceSettings = () => {
  const { setColorScheme, colorScheme } = useMantineColorScheme();
  const [fontSize, setFontSize] = useState(16);
  const [primaryColor, setPrimaryColor] = useState('#228be6');
  const [compactMode, setCompactMode] = useState(false);
  const [showAnimations, setShowAnimations] = useState(true);
  const [reducedMotion, setReducedMotion] = useState(false);

  return (
    <ScrollArea h={400}>
      <Stack gap="lg" mr="md">
        <Title order={3} size="h4">
          Appearance Settings
        </Title>

        {/* Color Scheme */}
        <Card withBorder padding="md">
          <Group justify="space-between" align="center" mb="md">
            <Group>
              <IconPalette size={20} />
              <Text fw={500}>Color Scheme</Text>
            </Group>
            <Badge variant="light">{colorScheme}</Badge>
          </Group>

          <Select
            label="Theme preference"
            description="Choose your preferred color scheme"
            value={colorScheme}
            onChange={(value) => setColorScheme(value as 'light' | 'dark' | 'auto')}
            data={[
              { value: 'light', label: 'Light' },
              { value: 'dark', label: 'Dark' },
              { value: 'auto', label: 'System' },
            ]}
          />
        </Card>
      </Stack>
    </ScrollArea>
  );
};
