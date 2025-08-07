import type { StorybookConfig } from '@storybook/react-vite';

const config: StorybookConfig = {
  core: {
    disableWhatsNewNotifications: true,
    disableTelemetry: true,
    enableCrashReports: false,
  },
  stories: [
    '../src/**/*.mdx',
    '../src/**/*.story.@(js|jsx|ts|tsx)',
    '../__stories__/**/*.@(js|jsx|ts|tsx)',
  ],
  framework: {
    name: '@storybook/react-vite',
    options: {},
  },
};

export default config;
