import { useNavigate } from 'react-router-dom';
import {
  Anchor,
  Box,
  Button,
  Center,
  Divider,
  Flex,
  Image,
  PasswordInput,
  Text,
  TextInput,
} from '@mantine/core';
import { useForm } from '@mantine/form';
import { LoginCredentials } from '@/api-client';
import { GoogleSignInButton } from '@/components/Authentication/GoogleSignInButton';
import { useAuth } from '@/providers/AuthProvider';
import classes from './Login.module.css';

export default function LoginPage() {
  const navigate = useNavigate();
  const { login } = useAuth();

  const form = useForm({
    initialValues: {
      email: '',
      password: '',
    },
    validate: {
      email: (value) => {
        if (value.length === 0) return 'Email is required';
        if (!value.includes('@') || value.length < 3) return 'Invalid email';
        return null;
      },
      password: (value) => (value.length > 0 ? null : 'Password is required'),
    },
  });

  const handleLogin = async (values: { email: string; password: string }) => {
    try {
      await login({ email: values.email, password: values.password } as LoginCredentials);
      navigate('/');
    } catch (err) {
      return;
    }
  };

  return (
    <Center className={classes.container}>
      <Flex direction="column" gap={20} w={350}>
        <Text size="xl" fw={500} ta="center">
          Login
        </Text>
        <form onSubmit={form.onSubmit(handleLogin)} className={classes.loginForm}>
          <TextInput
            placeholder="Email"
            className={classes.input}
            radius="md"
            {...form.getInputProps('email')}
          />
          <PasswordInput
            placeholder="Password"
            className={classes.input}
            radius="md"
            {...form.getInputProps('password')}
          />
          <Button className={classes.button} type="submit">
            Login
          </Button>
        </form>
        <Divider label="OR" labelPosition="center" />
        <GoogleSignInButton />
        <Flex direction="row" gap={5} ta="center" justify="center">
          <Text size="sm" ta="center">
            Don't have an account?
          </Text>
          <Text
            fw={500}
            size="sm"
            className={classes.signUpLink}
            onClick={() => navigate('/signup')}
          >
            Sign up here
          </Text>
        </Flex>
      </Flex>
    </Center>
  );
}
