import { useEffect, useState } from 'react';
import { Navigate, Outlet } from 'react-router-dom';
import { Center, Flex, Loader, Text } from '@mantine/core';
import { AuthStatus, useAuth } from '../../providers/AuthProvider';

export const ProtectedRoute = () => {
  const { checkAuthenticated } = useAuth();
  const [authStatus, setAuthStatus] = useState<AuthStatus | null>(null);

  useEffect(() => {
    let cancelled = false;

    const verify = async () => {
      try {
        const res = await checkAuthenticated();

        console.log(res);

        if (!cancelled) setAuthStatus(res);
      } catch (err) {
        if (!cancelled) setAuthStatus(AuthStatus.Unauthenticated);
      }
    };

    verify();

    return () => {
      cancelled = true;
    };
  }, [checkAuthenticated]);

  if (authStatus === null) {
    return (
      <Center h="100vh" w="100vw">
        <Flex gap="xs" direction="column" align="center">
          <Loader size="lg" />
          <Text fw={500}>Checking authentication...</Text>
        </Flex>
      </Center>
    );
  }

  if (authStatus == AuthStatus.Unauthenticated) {
    return <Navigate to="login" />;
  }

  if (authStatus == AuthStatus.Unverified) {
    return <Navigate to="verify" />;
  }

  return <Outlet />;
};
