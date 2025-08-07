import { useNavigate } from 'react-router-dom';
import {
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
import { GoogleSignInButton } from '@/components/Authentication/GoogleSignInButton';
import classes from './SignUp.module.css';

export default function SignUpPage() {
  const navigate = useNavigate();
  const form = useForm({
    initialValues: {
      username: '',
      email: '',
      password: '',
      confirmPassword: '',
    },
    validate: {
      username: (value) =>
        value.length < 3 ? 'Username must be at least 3 characters long' : null,
      email: (value) => (/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(value) ? null : 'Invalid email'),
      password: (value) =>
        value.length < 8 ? 'Password must be at least 8 characters long' : null,
      confirmPassword: (value, values) =>
        value !== values.password ? 'Passwords do not match' : null,
    },
  });

  return (
    <Center className={classes.container}>
      <Flex direction="column" gap={20} w={350}>
        <Text size="xl" fw={500} ta="center">
          Sign Up
        </Text>
        <form
          onSubmit={form.onSubmit((values) => console.log(values))}
          className={classes.signUpForm}
        >
          <TextInput
            placeholder="Username"
            className={classes.input}
            radius="md"
            {...form.getInputProps('username')}
          ></TextInput>
          <TextInput
            placeholder="Email"
            className={classes.input}
            radius="md"
            {...form.getInputProps('email')}
          ></TextInput>
          <PasswordInput
            placeholder="Password"
            className={classes.input}
            radius="md"
            {...form.getInputProps('password')}
          ></PasswordInput>
          <PasswordInput
            placeholder="Confirm Password"
            className={classes.input}
            radius="md"
            {...form.getInputProps('confirmPassword')}
          ></PasswordInput>
          <Button className={classes.button} type="submit">
            Sign up
          </Button>
        </form>
        <Divider label="OR" labelPosition="center" />
        <GoogleSignInButton />
        <Flex direction="row" gap={5} ta="center" justify="center">
          <Text size="sm" ta="center">
            Already have an account?
          </Text>
          <Text
            fw={500}
            size="sm"
            className={classes.signUpLink}
            onClick={() => navigate('/login')}
          >
            Log in here
          </Text>
        </Flex>
      </Flex>
    </Center>
  );
}
