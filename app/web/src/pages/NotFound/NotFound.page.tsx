import { Button, Flex, Text, Title } from '@mantine/core';

export default function NotFoundPage() {
  return (
    <Flex direction="column" align="center" justify="center" h="100vh">
      <Title order={1} fw={500}>
        404 - Page Not Found
      </Title>
      <Text size="lg" fw={500}>
        Oops! We couldn’t find the page you were looking for.
      </Text>
    </Flex>
  );
}
