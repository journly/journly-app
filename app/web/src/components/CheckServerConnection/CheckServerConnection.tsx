import { useEffect, useState } from 'react';
import { IconWifiOff } from '@tabler/icons-react';
import { Outlet } from 'react-router-dom';
import { Affix, Flex, Text, Transition } from '@mantine/core';
import styles from './CheckServerConnection.module.css';

export const CheckServerConnection = () => {
  const [connected, setConnected] = useState(true);

  const checkConnection = async () => {
    console.log('Checking connection...');
    try {
      const response = await fetch(`${import.meta.env.VITE_API_BASE_URL}/health`);
      setConnected(response.ok);
    } catch (error) {
      setConnected(false);
    }
  };

  useEffect(() => {
    checkConnection();
  }, []);

  return (
    <>
      <Outlet />
      <Affix position={{ bottom: 20, right: 20 }}>
        <Transition transition="slide-up" duration={200} mounted={!connected}>
          {(transitionStyles) => (
            <Flex gap="xs" align="center" style={transitionStyles} className={styles.message}>
              <IconWifiOff />
              <Text fw={500}>Server unavailable</Text>
            </Flex>
          )}
        </Transition>
      </Affix>
    </>
  );
};
